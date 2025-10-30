use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tracing::{info, warn, error};

use crate::models::{User, Group, Client, ClaimsRegistry};

#[derive(Debug, Clone)]
pub struct FileStorage {
    // Primary data
    users: HashMap<String, User>,
    groups: HashMap<String, Group>,
    roles: HashMap<String, Role>,
    clients: HashMap<String, Client>,

    // Computed indices for O(1) lookups
    email_index: HashMap<String, String>, // email -> user_id
    group_members: HashMap<String, Vec<String>>, // group_id -> [user_ids]

    data_dir: String,
}

// File format structures
#[derive(Debug, Serialize, Deserialize)]
struct UsersFile {
    users: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GroupsFile {
    groups: Vec<Group>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RolesFile {
    roles: Vec<Role>,
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

impl FileStorage {
    pub async fn load(data_dir: &str) -> Result<Self> {
        info!(
            event = "storage_load_start",
            data_dir = data_dir
        );

        // Load all JSON files
        let users_result = load_users_file(data_dir).await;
        let groups_result = load_groups_file(data_dir).await;
        let roles_result = load_roles_file(data_dir).await;
        let clients_result = load_clients_file(data_dir).await;

        // Handle system-critical data (roles must exist)
        let roles = match roles_result {
            LoadResult::Success(roles) => roles,
            LoadResult::CorruptData { error, .. } => {
                error!(
                    event = "critical_data_corrupt",
                    file = "roles.json",
                    error = error
                );
                return Err(anyhow::anyhow!("System critical data corrupt: roles.json - {}", error));
            }
        };

        // Handle user data (can be corrupt, use fallback)
        let users = match users_result {
            LoadResult::Success(users) => users,
            LoadResult::CorruptData { error, fallback } => {
                warn!(
                    event = "user_data_corrupt",
                    error = error,
                    fallback_count = fallback.len()
                );
                fallback
            }
        };

        // Handle group data
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

        // Handle client data
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

        let roles_map: HashMap<String, Role> = roles
            .into_iter()
            .map(|r| (r.id.clone(), r))
            .collect();

        let clients_map: HashMap<String, Client> = clients
            .into_iter()
            .map(|c| (c.client_id.clone(), c))
            .collect();

        // Build indices
        let email_index: HashMap<String, String> = users_map
            .iter()
            .map(|(id, user)| (user.email.clone(), id.clone()))
            .collect();

        let group_members: HashMap<String, Vec<String>> = {
            let mut members = HashMap::new();
            for (user_id, user) in &users_map {
                for group_id in &user.group_memberships {
                    members
                        .entry(group_id.clone())
                        .or_insert_with(Vec::new)
                        .push(user_id.clone());
                }
            }
            members
        };

        info!(
            event = "storage_loaded",
            users_count = users_map.len(),
            groups_count = groups_map.len(),
            roles_count = roles_map.len(),
            clients_count = clients_map.len()
        );

        Ok(Self {
            users: users_map,
            groups: groups_map,
            roles: roles_map,
            clients: clients_map,
            email_index,
            group_members,
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

    // Group operations
    pub fn get_group(&self, group_id: &str) -> Option<&Group> {
        self.groups.get(group_id)
    }

    pub fn get_group_members(&self, group_id: &str) -> Vec<&User> {
        self.group_members
            .get(group_id)
            .map(|member_ids| {
                member_ids
                    .iter()
                    .filter_map(|id| self.users.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn get_all_groups(&self) -> impl Iterator<Item = &Group> {
        self.groups.values()
    }

    // Role operations
    pub fn get_role(&self, role_id: &str) -> Option<&Role> {
        self.roles.get(role_id)
    }

    pub fn get_all_roles(&self) -> impl Iterator<Item = &Role> {
        self.roles.values()
    }

    // Client operations
    pub fn get_client(&self, client_id: &str) -> Option<&Client> {
        self.clients.get(client_id)
    }

    pub fn get_all_clients(&self) -> impl Iterator<Item = &Client> {
        self.clients.values()
    }

    // Statistics
    pub fn users_count(&self) -> usize {
        self.users.len()
    }

    pub fn groups_count(&self) -> usize {
        self.groups.len()
    }

    pub fn roles_count(&self) -> usize {
        self.roles.len()
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

    pub fn find_users_by_role(&self, role: &str) -> Vec<&User> {
        self.users
            .values()
            .filter(|u| u.roles.contains(&role.to_string()))
            .collect()
    }
}

async fn load_users_file(data_dir: &str) -> LoadResult<Vec<User>> {
    match load_json_file::<UsersFile>(&format!("{}/users.json", data_dir)).await {
        Ok(users_file) => LoadResult::Success(users_file.users),
        Err(e) => LoadResult::CorruptData {
            error: e.to_string(),
            fallback: Vec::new(),
        },
    }
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

async fn load_roles_file(data_dir: &str) -> LoadResult<Vec<Role>> {
    match load_json_file::<RolesFile>(&format!("{}/roles.json", data_dir)).await {
        Ok(roles_file) => LoadResult::Success(roles_file.roles),
        Err(e) => LoadResult::CorruptData {
            error: e.to_string(),
            fallback: default_roles(),
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

fn default_roles() -> Vec<Role> {
    vec![
        Role {
            id: "admin".to_string(),
            name: "Administrator".to_string(),
            description: "Full system access".to_string(),
            permissions: vec!["*".to_string()],
        },
        Role {
            id: "staff".to_string(),
            name: "Staff".to_string(),
            description: "Staff member".to_string(),
            permissions: vec![
                "participants:read".to_string(),
                "attendance:read".to_string(),
                "attendance:write".to_string(),
            ],
        },
        Role {
            id: "guardian".to_string(),
            name: "Guardian".to_string(),
            description: "Parent or guardian".to_string(),
            permissions: vec![
                "participants:read_own".to_string(),
                "attendance:read_own".to_string(),
            ],
        },
    ]
}