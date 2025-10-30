use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tracing::{info, warn, error};

use crate::models::{User, Group, Client, ClaimsRegistry};

#[derive(Debug, Clone)]
pub struct OrgBasedStorage {
    // Primary data (org-based users)
    users: HashMap<String, User>, // user_id -> User
    organizations: HashMap<String, Vec<String>>, // org -> [user_ids]
    groups: HashMap<String, Group>,
    clients: HashMap<String, Client>,
    claims_registry: ClaimsRegistry,

    // Computed indices for O(1) lookups
    email_index: HashMap<String, String>, // email -> user_id

    data_dir: String,
}

// File format structures for groups and clients (orgs are directories)
#[derive(Debug, Serialize, Deserialize)]
struct GroupsFile {
    groups: Vec<Group>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClientsFile {
    clients: Vec<Client>,
}

#[derive(Debug, Clone)]
pub enum LoadResult<T> {
    Success(T),
    CorruptData { error: String, fallback: T },
}

impl OrgBasedStorage {
    pub async fn load(data_dir: &str) -> Result<Self> {
        info!(
            event = "org_storage_load_start",
            data_dir = data_dir
        );

        // Load claims registry first (required)
        let claims_registry = load_claims_registry(data_dir).await?;

        // Load users from org-based directories
        let (users, organizations) = load_org_based_users(data_dir, &claims_registry).await?;

        // Load groups and clients
        let groups_result = load_groups_file(data_dir).await;
        let clients_result = load_clients_file(data_dir).await;

        let groups = match groups_result {
            LoadResult::Success(groups) => groups,
            LoadResult::CorruptData { error, fallback } => {
                warn!(
                    event = "group_data_corrupt",
                    error = error,
                    fallback_count = fallback.len()
                );
                fallback
            }
        };

        let clients = match clients_result {
            LoadResult::Success(clients) => clients,
            LoadResult::CorruptData { error, fallback } => {
                warn!(
                    event = "client_data_corrupt",
                    error = error,
                    fallback_count = fallback.len()
                );
                fallback
            }
        };

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

        info!(
            event = "org_storage_loaded",
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
        })
    }

    // User operations
    pub fn get_user_by_email(&self, email: &str) -> Option<&User> {
        let user_id = self.email_index.get(email)?;
        self.users.get(user_id)
    }

    pub fn get_user(&self, user_id: &str) -> Option<&User> {
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

    // Group operations
    pub fn get_group(&self, group_id: &str) -> Option<&Group> {
        self.groups.get(group_id)
    }

    pub fn get_all_groups(&self) -> impl Iterator<Item = &Group> {
        self.groups.values()
    }

    // Client operations
    pub fn get_client(&self, client_id: &str) -> Option<&Client> {
        self.clients.get(client_id)
    }

    pub fn get_all_clients(&self) -> impl Iterator<Item = &Client> {
        self.clients.values()
    }

    // Claims registry
    pub fn get_claims_registry(&self) -> &ClaimsRegistry {
        &self.claims_registry
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

    // Search operations
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
}

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

async fn load_groups_file(data_dir: &str) -> LoadResult<Vec<Group>> {
    match load_json_file::<GroupsFile>(&format!("{}/groups.json", data_dir)).await {
        Ok(groups_file) => LoadResult::Success(groups_file.groups),
        Err(e) => LoadResult::CorruptData {
            error: e.to_string(),
            fallback: Vec::new(),
        },
    }
}

async fn load_clients_file(data_dir: &str) -> LoadResult<Vec<Client>> {
    match load_json_file::<ClientsFile>(&format!("{}/clients.json", data_dir)).await {
        Ok(clients_file) => LoadResult::Success(clients_file.clients),
        Err(e) => LoadResult::CorruptData {
            error: e.to_string(),
            fallback: Vec::new(),
        },
    }
}

async fn load_json_file<T: for<'de> Deserialize<'de>>(path: &str) -> Result<T> {
    let content = tokio::fs::read_to_string(path).await
        .with_context(|| format!("Failed to read file: {}", path))?;

    serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse JSON in file: {}", path))
}