use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
use std::path::Path;
use time::OffsetDateTime;
use tracing::{info, warn, error};
use uuid::Uuid;

// Import shared models from our models module
use crate::models::{User, Group, Client, ClaimsRegistry, ClaimDefinition, UserStatus, ClientType, AuditEvent};


// File format structures
#[derive(Debug, Serialize, Deserialize)]
struct GroupsFile {
    groups: Vec<Group>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClientsFile {
    clients: Vec<Client>,
}

#[derive(Debug, Clone)]
pub struct DataSyncState {
    pub last_auth_reload: SystemTime,
    pub last_data_update: SystemTime,
}

#[derive(Debug, Clone)]
pub struct AdminStorage {
    // Primary data (read-write)
    users: HashMap<String, User>, // user_id -> User
    organizations: HashMap<String, Vec<String>>, // org -> [user_ids]
    groups: HashMap<String, Group>,
    clients: HashMap<String, Client>,
    claims_registry: ClaimsRegistry,

    // Computed indices
    email_index: HashMap<String, String>, // email -> user_id

    // System state
    data_dir: String,
    auth_pid_file: String,
    sync_state: DataSyncState,
}

impl AdminStorage {
    pub async fn load(data_dir: &str, auth_pid_file: &str) -> Result<Self> {
        info!(
            event = "admin_storage_load_start",
            data_dir = data_dir
        );

        // Load claims registry first (required)
        let claims_registry = load_claims_registry(data_dir).await?;

        // Load users from org-based directories
        let (users, organizations) = load_org_based_users(data_dir, &claims_registry).await?;

        // Load groups and clients
        let groups = load_groups_file(data_dir).await?;
        let clients = load_clients_file(data_dir).await?;

        // Convert to HashMaps
        let users_map: HashMap<String, User> = users
            .into_iter()
            .map(|u| (u.id.clone(), u))
            .collect();

        let groups_map: HashMap<String, Group> = groups
            .into_iter()
            .map(|g| (g.id.clone(), g))
            .collect();

        let clients_map: HashMap<String, Client> = clients
            .into_iter()
            .map(|c| (c.client_id.clone(), c))
            .collect();

        // Build email index
        let email_index: HashMap<String, String> = users_map
            .iter()
            .map(|(id, user)| (user.email.clone(), id.clone()))
            .collect();

        let now = SystemTime::now();
        let sync_state = DataSyncState {
            last_auth_reload: now,
            last_data_update: now,
        };

        info!(
            event = "admin_storage_loaded",
            users_count = users_map.len(),
            organizations_count = organizations.len(),
            groups_count = groups_map.len(),
            clients_count = clients_map.len()
        );

        Ok(Self {
            users: users_map,
            organizations,
            groups: groups_map,
            clients: clients_map,
            claims_registry,
            email_index,
            data_dir: data_dir.to_string(),
            auth_pid_file: auth_pid_file.to_string(),
            sync_state,
        })
    }

    // System operations
    pub fn is_auth_stale(&self) -> bool {
        self.sync_state.last_data_update > self.sync_state.last_auth_reload
    }

    pub async fn trigger_auth_reload(&mut self) -> Result<()> {
        let start_time = std::time::Instant::now();

        // Read PID from file
        let pid_str = tokio::fs::read_to_string(&self.auth_pid_file).await
            .context("Failed to read auth service PID file")?;

        let pid: i32 = pid_str.trim().parse()
            .context("Invalid PID in file")?;

        // Send SIGHUP
        let result = unsafe {
            libc::kill(pid, libc::SIGHUP)
        };

        if result == 0 {
            self.sync_state.last_auth_reload = SystemTime::now();

            info!(
                service = "admin-service",
                event = "auth_reload_triggered",
                auth_pid = pid,
                success = true,
                duration_ms = start_time.elapsed().as_millis()
            );

            Ok(())
        } else {
            let error = std::io::Error::last_os_error();

            error!(
                service = "admin-service",
                event = "auth_reload_failed",
                auth_pid = pid,
                error = %error,
                duration_ms = start_time.elapsed().as_millis()
            );

            Err(anyhow::anyhow!("Failed to send SIGHUP: {}", error))
        }
    }

    // User operations with immediate persistence
    pub async fn create_user(&mut self, user: User) -> Result<User> {
        // Update memory
        self.email_index.insert(user.email.clone(), user.id.clone());

        // Update organization index
        self.organizations
            .entry(user.org.clone())
            .or_insert_with(Vec::new)
            .push(user.id.clone());

        self.users.insert(user.id.clone(), user.clone());

        // Immediate persistence
        self.persist_user(&user).await?;
        self.sync_state.last_data_update = SystemTime::now();

        info!(
            service = "admin-service",
            event = "user_created",
            user_id = %user.id,
            email = %user.email,
            org = %user.org
        );

        Ok(user)
    }

    pub async fn update_user(&mut self, user_id: &str, user: User) -> Result<User> {
        // Remove old user from indices
        if let Some(old_user) = self.users.get(user_id) {
            self.email_index.remove(&old_user.email);

            // Remove from old org
            if let Some(org_users) = self.organizations.get_mut(&old_user.org) {
                org_users.retain(|id| id != user_id);
            }
        }

        // Update indices
        self.email_index.insert(user.email.clone(), user.id.clone());

        // Add to new org
        self.organizations
            .entry(user.org.clone())
            .or_insert_with(Vec::new)
            .push(user.id.clone());

        self.users.insert(user_id.to_string(), user.clone());

        // Immediate persistence
        self.persist_user(&user).await?;
        self.sync_state.last_data_update = SystemTime::now();

        info!(
            service = "admin-service",
            event = "user_updated",
            user_id = %user_id,
            org = %user.org
        );

        Ok(user)
    }

    pub async fn delete_user(&mut self, user_id: &str) -> Result<()> {
        if let Some(user) = self.users.remove(user_id) {
            // Remove from indices
            self.email_index.remove(&user.email);

            // Remove from organization
            if let Some(org_users) = self.organizations.get_mut(&user.org) {
                org_users.retain(|id| id != user_id);
            }

            // Delete user file
            self.delete_user_file(&user).await?;
            self.sync_state.last_data_update = SystemTime::now();

            info!(
                service = "admin-service",
                event = "user_deleted",
                user_id = %user_id,
                org = %user.org
            );
        }

        Ok(())
    }

    // Read operations (no persistence needed)
    pub fn get_user(&self, user_id: &str) -> Option<&User> {
        self.users.get(user_id)
    }

    pub fn get_user_by_email(&self, email: &str) -> Option<&User> {
        let user_id = self.email_index.get(email)?;
        self.users.get(user_id)
    }

    pub fn get_all_users(&self) -> impl Iterator<Item = &User> {
        self.users.values()
    }

    pub fn get_users_by_org(&self, org: &str) -> Vec<&User> {
        self.organizations
            .get(org)
            .map(|user_ids| {
                user_ids
                    .iter()
                    .filter_map(|id| self.users.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn search_users(&self, query: &str) -> Vec<&User> {
        let query_lower = query.to_lowercase();
        self.users
            .values()
            .filter(|u| {
                u.email.to_lowercase().contains(&query_lower)
                    || u.first_name.to_lowercase().contains(&query_lower)
                    || u.last_name.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    pub fn find_users_by_claim(&self, claim_key: &str, claim_value: &str) -> Vec<&User> {
        self.users
            .values()
            .filter(|u| {
                u.claims.get(claim_key)
                    .map(|v| match v {
                        serde_json::Value::String(s) => s == claim_value,
                        serde_json::Value::Array(arr) => {
                            arr.iter().any(|item| item.as_str() == Some(claim_value))
                        }
                        _ => false,
                    })
                    .unwrap_or(false)
            })
            .collect()
    }

    // Group operations
    pub async fn create_group(&mut self, group: Group) -> Result<Group> {
        self.groups.insert(group.id.clone(), group.clone());
        self.persist_groups().await?;
        self.sync_state.last_data_update = SystemTime::now();

        info!(
            service = "admin-service",
            event = "group_created",
            group_id = %group.id
        );

        Ok(group)
    }

    pub fn get_group(&self, group_id: &str) -> Option<&Group> {
        self.groups.get(group_id)
    }

    pub fn get_all_groups(&self) -> impl Iterator<Item = &Group> {
        self.groups.values()
    }


    // Statistics
    pub fn users_count(&self) -> usize {
        self.users.len()
    }

    pub fn groups_count(&self) -> usize {
        self.groups.len()
    }

    pub fn clients_count(&self) -> usize {
        self.clients.len()
    }

    pub fn organizations_count(&self) -> usize {
        self.organizations.len()
    }

    pub fn get_all_clients(&self) -> impl Iterator<Item = &Client> {
        self.clients.values()
    }

    pub fn get_client(&self, client_id: &str) -> Option<&Client> {
        self.clients.get(client_id)
    }

    // Claims registry
    pub fn get_claims_registry(&self) -> &ClaimsRegistry {
        &self.claims_registry
    }

    // Persistence operations
    async fn persist_user(&self, user: &User) -> Result<()> {
        let org_dir = format!("{}/users/{}", self.data_dir, user.org);

        // Ensure org directory exists
        tokio::fs::create_dir_all(&org_dir).await
            .context("Failed to create org directory")?;

        let user_path = format!("{}/{}.json", org_dir, user.id);
        let temp_path = format!("{}.tmp", user_path);

        tokio::fs::write(&temp_path, serde_json::to_string_pretty(user)?)
            .await
            .context("Failed to write user temp file")?;

        tokio::fs::rename(temp_path, user_path)
            .await
            .context("Failed to rename user file")?;

        Ok(())
    }

    async fn delete_user_file(&self, user: &User) -> Result<()> {
        let user_path = format!("{}/users/{}/{}.json", self.data_dir, user.org, user.id);

        if Path::new(&user_path).exists() {
            tokio::fs::remove_file(&user_path).await
                .context("Failed to delete user file")?;
        }

        Ok(())
    }

    async fn persist_groups(&self) -> Result<()> {
        let groups_file = GroupsFile {
            groups: self.groups.values().cloned().collect(),
        };

        let temp_path = format!("{}/groups.json.tmp", self.data_dir);
        let final_path = format!("{}/groups.json", self.data_dir);

        tokio::fs::write(&temp_path, serde_json::to_string_pretty(&groups_file)?)
            .await
            .context("Failed to write groups temp file")?;

        tokio::fs::rename(temp_path, final_path)
            .await
            .context("Failed to rename groups file")?;

        Ok(())
    }
}

// File loading functions
async fn load_claims_registry(data_dir: &str) -> Result<ClaimsRegistry> {
    let path = format!("{}/claims.conf", data_dir);
    let content = tokio::fs::read_to_string(&path).await
        .context("Failed to read claims registry")?;

    let registry: ClaimsRegistry = serde_json::from_str(&content)
        .context("Failed to parse claims registry")?;

    Ok(registry)
}

async fn load_org_based_users(data_dir: &str, claims_registry: &ClaimsRegistry) -> Result<(Vec<User>, HashMap<String, Vec<String>>)> {
    let users_dir = format!("{}/users", data_dir);
    let mut all_users = Vec::new();
    let mut organizations = HashMap::new();

    // Check if users directory exists
    if !Path::new(&users_dir).exists() {
        warn!("Users directory not found: {}", users_dir);
        return Ok((all_users, organizations));
    }

    // Read all organization directories
    let mut entries = tokio::fs::read_dir(&users_dir).await
        .context("Failed to read users directory")?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();

        if path.is_dir() {
            let org_name = path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown")
                .to_string();

            info!("Loading users for organization: {}", org_name);

            let org_users = load_users_from_org_dir(&path, &org_name, claims_registry).await?;
            let user_ids: Vec<String> = org_users.iter().map(|u| u.id.clone()).collect();

            organizations.insert(org_name, user_ids);
            all_users.extend(org_users);
        }
    }

    Ok((all_users, organizations))
}

async fn load_users_from_org_dir(org_dir: &Path, org_name: &str, claims_registry: &ClaimsRegistry) -> Result<Vec<User>> {
    let mut users = Vec::new();
    let mut entries = tokio::fs::read_dir(org_dir).await
        .context("Failed to read org directory")?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
            match load_user_file(&path, claims_registry).await {
                Ok(user) => {
                    // Validate that user's org matches directory
                    if user.org != org_name {
                        warn!(
                            "User {} org mismatch: file in {}, user.org = {}",
                            user.id, org_name, user.org
                        );
                    }
                    users.push(user);
                }
                Err(e) => {
                    error!(
                        "Failed to load user file {:?}: {}",
                        path, e
                    );
                }
            }
        }
    }

    Ok(users)
}

async fn load_user_file(path: &Path, claims_registry: &ClaimsRegistry) -> Result<User> {
    let content = tokio::fs::read_to_string(path).await
        .with_context(|| format!("Failed to read user file: {:?}", path))?;

    let mut user: User = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse user file: {:?}", path))?;

    // Validate claims against registry
    validate_user_claims(&mut user, claims_registry)?;

    Ok(user)
}

fn validate_user_claims(user: &mut User, registry: &ClaimsRegistry) -> Result<()> {
    let mut validated_claims = HashMap::new();

    for (claim_key, claim_value) in &user.claims {
        if let Some(definition) = registry.claims.get(claim_key) {
            // Basic validation - could be extended
            if definition.default_allowed || user.is_admin() {
                validated_claims.insert(claim_key.clone(), claim_value.clone());
            } else {
                warn!(
                    "Claim '{}' not allowed for user {} (not admin)",
                    claim_key, user.id
                );
            }
        } else {
            warn!(
                "Unknown claim '{}' for user {}",
                claim_key, user.id
            );
        }
    }

    user.claims = validated_claims;
    Ok(())
}

async fn load_groups_file(data_dir: &str) -> Result<Vec<Group>> {
    let groups_file: GroupsFile = load_json_file(&format!("{}/groups.json", data_dir)).await?;
    Ok(groups_file.groups)
}

async fn load_clients_file(data_dir: &str) -> Result<Vec<Client>> {
    let clients_file: ClientsFile = load_json_file(&format!("{}/clients.json", data_dir)).await?;
    Ok(clients_file.clients)
}

async fn load_json_file<T: for<'de> Deserialize<'de>>(path: &str) -> Result<T> {
    let content = tokio::fs::read_to_string(path).await
        .with_context(|| format!("Failed to read file: {}", path))?;

    serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse JSON in file: {}", path))
}