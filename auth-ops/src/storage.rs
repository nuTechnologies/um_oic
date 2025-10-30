// Storage operations for CLI tool (simplified version of admin-service storage)
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

use crate::models::{User, Group, Client};

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
pub struct FileStorage {
    // Primary data
    users: HashMap<String, User>,
    groups: HashMap<String, Group>,
    roles: HashMap<String, Role>,
    clients: HashMap<String, Client>,

    // Computed indices
    email_index: HashMap<String, String>, // email -> user_id
    group_members: HashMap<String, Vec<String>>, // group_id -> [user_ids]
}

impl FileStorage {
    pub async fn load(data_dir: &str) -> Result<Self> {
        info!("Loading storage from {}", data_dir);

        // Load all JSON files
        let users = load_users_file(data_dir).await?;
        let groups = load_groups_file(data_dir).await?;
        let roles = load_roles_file(data_dir).await?;
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
            "Storage loaded - Users: {}, Groups: {}, Roles: {}, Clients: {}",
            users_map.len(),
            groups_map.len(),
            roles_map.len(),
            clients_map.len()
        );

        Ok(Self {
            users: users_map,
            groups: groups_map,
            roles: roles_map,
            clients: clients_map,
            email_index,
            group_members,
        })
    }

    // User operations
    pub async fn create_user(&mut self, user: User) -> Result<()> {
        self.email_index.insert(user.email.clone(), user.id.clone());
        for group_id in &user.group_memberships {
            self.group_members
                .entry(group_id.clone())
                .or_insert_with(Vec::new)
                .push(user.id.clone());
        }
        self.users.insert(user.id.clone(), user);
        Ok(())
    }

    pub async fn update_user(&mut self, user_id: &str, user: User) -> Result<()> {
        // Remove old email index
        if let Some(old_user) = self.users.get(user_id) {
            self.email_index.remove(&old_user.email);
        }

        // Update indices
        self.email_index.insert(user.email.clone(), user.id.clone());
        self.users.insert(user_id.to_string(), user);
        Ok(())
    }

    pub async fn delete_user(&mut self, user_id: &str) -> Result<()> {
        if let Some(user) = self.users.remove(user_id) {
            // Remove from indices
            self.email_index.remove(&user.email);
            for group_id in &user.group_memberships {
                if let Some(members) = self.group_members.get_mut(group_id) {
                    members.retain(|id| id != user_id);
                }
            }
        }
        Ok(())
    }

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

    // Group operations
    pub async fn create_group(&mut self, group: Group) -> Result<()> {
        self.groups.insert(group.id.clone(), group);
        Ok(())
    }

    pub fn get_group(&self, group_id: &str) -> Option<&Group> {
        self.groups.get(group_id)
    }

    pub fn get_all_groups(&self) -> impl Iterator<Item = &Group> {
        self.groups.values()
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

    // Client operations
    pub async fn create_client(&mut self, client: Client) -> Result<()> {
        self.clients.insert(client.client_id.clone(), client);
        Ok(())
    }

    pub fn get_all_clients(&self) -> impl Iterator<Item = &Client> {
        self.clients.values()
    }

    // Role operations
    pub fn get_all_roles(&self) -> impl Iterator<Item = &Role> {
        self.roles.values()
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

    pub fn clients_count(&self) -> usize {
        self.clients.len()
    }

    // Persistence
    pub async fn persist(&self, data_dir: &str) -> Result<()> {
        // Save users
        let users_file = UsersFile {
            users: self.users.values().cloned().collect(),
        };
        save_json_file(&format!("{}/users.json", data_dir), &users_file).await?;

        // Save groups
        let groups_file = GroupsFile {
            groups: self.groups.values().cloned().collect(),
        };
        save_json_file(&format!("{}/groups.json", data_dir), &groups_file).await?;

        // Save roles
        let roles_file = RolesFile {
            roles: self.roles.values().cloned().collect(),
        };
        save_json_file(&format!("{}/roles.json", data_dir), &roles_file).await?;

        // Save clients
        let clients_file = ClientsFile {
            clients: self.clients.values().cloned().collect(),
        };
        save_json_file(&format!("{}/clients.json", data_dir), &clients_file).await?;

        info!("Data persisted to {}", data_dir);
        Ok(())
    }
}

// File loading functions
async fn load_users_file(data_dir: &str) -> Result<Vec<User>> {
    let users_file: UsersFile = load_json_file(&format!("{}/users.json", data_dir)).await?;
    Ok(users_file.users)
}

async fn load_groups_file(data_dir: &str) -> Result<Vec<Group>> {
    let groups_file: GroupsFile = load_json_file(&format!("{}/groups.json", data_dir)).await?;
    Ok(groups_file.groups)
}

async fn load_roles_file(data_dir: &str) -> Result<Vec<Role>> {
    let roles_file: RolesFile = load_json_file(&format!("{}/roles.json", data_dir)).await?;
    Ok(roles_file.roles)
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

async fn save_json_file<T: Serialize>(path: &str, data: &T) -> Result<()> {
    let temp_path = format!("{}.tmp", path);

    let content = serde_json::to_string_pretty(data)
        .context("Failed to serialize data")?;

    tokio::fs::write(&temp_path, content).await
        .context("Failed to write temp file")?;

    tokio::fs::rename(temp_path, path).await
        .context("Failed to rename temp file")?;

    Ok(())
}