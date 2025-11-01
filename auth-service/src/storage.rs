use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tracing::{info, warn, error};

use crate::models::{User, Group, Role, Client, ClaimsRegistry};

#[derive(Debug, Clone)]
pub struct FileStorage {
    // Primary data
    users: HashMap<String, User>,
    roles: HashMap<String, Role>,
    clients: HashMap<String, Client>,

    // Computed indices for O(1) lookups
    email_index: HashMap<String, String>, // email -> user_id

    data_dir: String,
}

// File format structures
#[derive(Debug, Serialize, Deserialize)]
struct UsersFile {
    users: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OrgsFile {
    orgs: Vec<Group>,
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
        let clients_result = load_clients_file(data_dir).await;

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



        let clients_map: HashMap<String, Client> = clients
            .into_iter()
            .map(|c| (c.client_id.clone(), c))
            .collect();

        // Build indices
        let email_index: HashMap<String, String> = users_map
            .iter()
            .map(|(id, user)| (user.email.clone(), id.clone()))
            .collect();


        info!(
            event = "storage_loaded",
            users_count = users_map.len(),
            clients_count = clients_map.len()
        );

        Ok(Self {
            users: users_map,
            roles: HashMap::new(),
            clients: clients_map,
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
            .filter(|u| u.admin.contains(&role.to_string()) || u.admin.contains(&"all".to_string()))
            .collect()
    }
}

async fn load_users_file(data_dir: &str) -> LoadResult<Vec<User>> {
    let users_dir = format!("{}/users", data_dir);
    let mut all_users = Vec::new();

    // Read all organization directories
    if let Ok(entries) = tokio::fs::read_dir(&users_dir).await {
        let mut entries = entries;
        while let Ok(Some(entry)) = entries.next_entry().await {
            if entry.file_type().await.map(|ft| ft.is_dir()).unwrap_or(false) {
                let org_dir = entry.path();

                // Read all user files in organization directory
                if let Ok(user_entries) = tokio::fs::read_dir(&org_dir).await {
                    let mut user_entries = user_entries;
                    while let Ok(Some(user_entry)) = user_entries.next_entry().await {
                        if let Some(file_name) = user_entry.file_name().to_str() {
                            if file_name.starts_with("user-") && file_name.ends_with(".json") {
                                match load_json_file::<User>(&user_entry.path().to_string_lossy()).await {
                                    Ok(user) => all_users.push(user),
                                    Err(e) => warn!("Failed to load user file {}: {}", file_name, e),
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if all_users.is_empty() {
        // Try fallback to old users.json format
        match load_json_file::<UsersFile>(&format!("{}/users.json", data_dir)).await {
            Ok(users_file) => LoadResult::Success(users_file.users),
            Err(e) => LoadResult::CorruptData {
                error: format!("No users found in {} and fallback users.json failed: {}", users_dir, e),
                fallback: Vec::new(),
            },
        }
    } else {
        LoadResult::Success(all_users)
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

async fn load_clients_file(_data_dir: &str) -> LoadResult<Vec<Client>> {
    // clients.json was removed - return empty list
    LoadResult::Success(Vec::new())
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