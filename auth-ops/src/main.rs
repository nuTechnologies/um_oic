use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::collections::HashMap;
use time::OffsetDateTime;
use tracing::{info, error};
use uuid::Uuid;

mod storage;
mod models;
mod password;
mod backup;

use storage::FileStorage;
use models::{User, UserStatus, Group, Client, ClientType};

#[derive(Parser)]
#[command(name = "auth-ops")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Auth system operations and maintenance tool")]
struct Cli {
    /// Data directory
    #[arg(long, env = "AUTH_DATA_DIR", default_value = "./data")]
    data_dir: String,

    /// Auth service PID file
    #[arg(long, env = "AUTH_PID_FILE", default_value = "/var/run/auth-service.pid")]
    auth_pid_file: String,

    /// Enable debug logging
    #[arg(long)]
    debug: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Reload auth service data
    Reload,

    /// Backup operations
    Backup {
        #[arg(long)]
        output_dir: String,
    },

    /// Restore from backup
    Restore {
        #[arg(long)]
        backup_dir: String,
    },

    /// Verify data integrity
    Verify {
        #[arg(long)]
        fix: bool,
    },

    /// User management
    User {
        #[command(subcommand)]
        cmd: UserCommands,
    },

    /// Group management
    Group {
        #[command(subcommand)]
        cmd: GroupCommands,
    },

    /// Client management
    Client {
        #[command(subcommand)]
        cmd: ClientCommands,
    },

    /// Archive old audit logs
    Archive {
        #[arg(long)]
        older_than_days: u32,
    },

    /// Show system status
    Status,
}

#[derive(Subcommand)]
enum UserCommands {
    /// Create a new user
    Create {
        #[arg(long)]
        email: String,
        #[arg(long)]
        password: String,
        #[arg(long)]
        first_name: String,
        #[arg(long)]
        last_name: String,
        #[arg(long)]
        roles: Vec<String>,
    },
    /// List users
    List {
        #[arg(long)]
        search: Option<String>,
    },
    /// Delete a user
    Delete {
        #[arg(long)]
        email: String,
    },
    /// Reset user password
    ResetPassword {
        #[arg(long)]
        email: String,
        #[arg(long)]
        new_password: String,
    },
}

#[derive(Subcommand)]
enum GroupCommands {
    /// Create a new group
    Create {
        #[arg(long)]
        id: String,
        #[arg(long)]
        name: String,
        #[arg(long)]
        description: String,
    },
    /// List groups
    List,
}

#[derive(Subcommand)]
enum ClientCommands {
    /// Create a new OAuth client
    Create {
        #[arg(long)]
        client_id: String,
        #[arg(long)]
        name: String,
        #[arg(long)]
        redirect_uris: Vec<String>,
    },
    /// List clients
    List,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Setup logging
    setup_logging(cli.debug);

    match cli.command {
        Commands::Reload => {
            reload_auth_service(&cli.auth_pid_file).await?;
        }
        Commands::Backup { output_dir } => {
            backup::backup_data(&cli.data_dir, &output_dir).await?;
        }
        Commands::Restore { backup_dir } => {
            backup::restore_data(&backup_dir, &cli.data_dir).await?;
        }
        Commands::Verify { fix } => {
            verify_data_integrity(&cli.data_dir, fix).await?;
        }
        Commands::User { cmd } => {
            handle_user_command(cmd, &cli.data_dir).await?;
        }
        Commands::Group { cmd } => {
            handle_group_command(cmd, &cli.data_dir).await?;
        }
        Commands::Client { cmd } => {
            handle_client_command(cmd, &cli.data_dir).await?;
        }
        Commands::Archive { older_than_days } => {
            archive_old_logs(&cli.data_dir, older_than_days).await?;
        }
        Commands::Status => {
            show_status(&cli.data_dir, &cli.auth_pid_file).await?;
        }
    }

    Ok(())
}

fn setup_logging(debug: bool) {
    if debug {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .compact()
            .init();
    }
}

async fn reload_auth_service(pid_file: &str) -> Result<()> {
    let pid_str = tokio::fs::read_to_string(pid_file).await
        .context("Failed to read PID file")?;

    let pid: i32 = pid_str.trim().parse()
        .context("Invalid PID in file")?;

    let result = unsafe { libc::kill(pid, libc::SIGHUP) };

    if result == 0 {
        println!("✅ SIGHUP sent to auth-service (PID: {})", pid);
        info!("Auth service reload triggered successfully");
        Ok(())
    } else {
        let error = std::io::Error::last_os_error();
        error!("Failed to reload auth-service: {}", error);
        Err(anyhow::anyhow!("Failed to reload auth-service: {}", error))
    }
}

async fn verify_data_integrity(data_dir: &str, fix: bool) -> Result<()> {
    println!("🔍 Verifying data integrity...");

    let storage = FileStorage::load(data_dir).await
        .context("Failed to load storage")?;

    println!("✅ Data loaded successfully:");
    println!("   - Users: {}", storage.users_count());
    println!("   - Groups: {}", storage.groups_count());
    println!("   - Roles: {}", storage.roles_count());
    println!("   - Clients: {}", storage.clients_count());

    // TODO: Add more integrity checks
    // - Check email uniqueness
    // - Validate group memberships
    // - Check role references
    // - Validate JSON structure

    if fix {
        println!("🔧 Auto-fix mode enabled (not yet implemented)");
    }

    Ok(())
}

async fn handle_user_command(cmd: UserCommands, data_dir: &str) -> Result<()> {
    let mut storage = FileStorage::load(data_dir).await?;

    match cmd {
        UserCommands::Create { email, password, first_name, last_name, roles } => {
            let password_hash = password::hash_password(&password)?;
            let now = OffsetDateTime::now_utc();

            let user = User {
                id: format!("user-{}", Uuid::new_v4().simple()),
                email: email.clone(),
                password_hash,
                first_name,
                last_name,
                status: UserStatus::Active,
                roles,
                group_memberships: vec![],
                mfa_secret: None,
                created_at: now,
                updated_at: now,
            };

            storage.create_user(user.clone()).await?;
            storage.persist(data_dir).await?;

            println!("✅ User created: {} ({})", email, user.id);
        }
        UserCommands::List { search } => {
            let users: Vec<_> = if let Some(query) = search {
                storage.search_users(&query)
            } else {
                storage.get_all_users().collect()
            };

            println!("📋 Users ({}):", users.len());
            for user in users {
                println!("   - {} ({}) - {} - {:?}",
                    user.email, user.id, user.full_name(), user.roles);
            }
        }
        UserCommands::Delete { email } => {
            if let Some(user) = storage.get_user_by_email(&email) {
                let user_id = user.id.clone();
                storage.delete_user(&user_id).await?;
                storage.persist(data_dir).await?;
                println!("✅ User deleted: {}", email);
            } else {
                println!("❌ User not found: {}", email);
            }
        }
        UserCommands::ResetPassword { email, new_password } => {
            if let Some(user) = storage.get_user_by_email(&email) {
                let mut updated_user = user.clone();
                updated_user.password_hash = password::hash_password(&new_password)?;
                updated_user.updated_at = OffsetDateTime::now_utc();

                storage.update_user(&updated_user.id.clone(), updated_user).await?;
                storage.persist(data_dir).await?;
                println!("✅ Password reset for: {}", email);
            } else {
                println!("❌ User not found: {}", email);
            }
        }
    }

    Ok(())
}

async fn handle_group_command(cmd: GroupCommands, data_dir: &str) -> Result<()> {
    let mut storage = FileStorage::load(data_dir).await?;

    match cmd {
        GroupCommands::Create { id, name, description } => {
            let group = Group {
                id: id.clone(),
                name,
                description,
                metadata: HashMap::new(),
                created_at: OffsetDateTime::now_utc(),
            };

            storage.create_group(group).await?;
            storage.persist(data_dir).await?;
            println!("✅ Group created: {}", id);
        }
        GroupCommands::List => {
            let groups: Vec<_> = storage.get_all_groups().collect();
            println!("📋 Groups ({}):", groups.len());
            for group in groups {
                let member_count = storage.get_group_members(&group.id).len();
                println!("   - {} ({}) - {} members",
                    group.name, group.id, member_count);
            }
        }
    }

    Ok(())
}

async fn handle_client_command(cmd: ClientCommands, data_dir: &str) -> Result<()> {
    let mut storage = FileStorage::load(data_dir).await?;

    match cmd {
        ClientCommands::Create { client_id, name, redirect_uris } => {
            let client = Client {
                client_id: client_id.clone(),
                client_secret_hash: None,
                name,
                client_type: ClientType::Public,
                redirect_uris,
                allowed_scopes: vec!["openid".to_string(), "profile".to_string(), "email".to_string()],
                require_pkce: true,
                grant_types: vec!["authorization_code".to_string(), "refresh_token".to_string()],
                created_at: OffsetDateTime::now_utc(),
            };

            storage.create_client(client).await?;
            storage.persist(data_dir).await?;
            println!("✅ Client created: {}", client_id);
        }
        ClientCommands::List => {
            let clients: Vec<_> = storage.get_all_clients().collect();
            println!("📋 Clients ({}):", clients.len());
            for client in clients {
                println!("   - {} ({}) - {:?}",
                    client.name, client.client_id, client.client_type);
            }
        }
    }

    Ok(())
}

async fn archive_old_logs(_data_dir: &str, _older_than_days: u32) -> Result<()> {
    println!("🗂️  Archiving audit logs older than {} days...", _older_than_days);

    // TODO: Implement audit log archival
    // - Scan audit/ directory
    // - Find files older than threshold
    // - Compress and move to archive

    println!("✅ Log archival completed (not yet implemented)");
    Ok(())
}

async fn show_status(data_dir: &str, auth_pid_file: &str) -> Result<()> {
    println!("📊 System Status");
    println!("================");

    // Load data status
    match FileStorage::load(data_dir).await {
        Ok(storage) => {
            println!("✅ Data Status:");
            println!("   - Users: {}", storage.users_count());
            println!("   - Groups: {}", storage.groups_count());
            println!("   - Roles: {}", storage.roles_count());
            println!("   - Clients: {}", storage.clients_count());
        }
        Err(e) => {
            println!("❌ Data Status: Failed to load - {}", e);
        }
    }

    // Auth service status
    match tokio::fs::read_to_string(auth_pid_file).await {
        Ok(pid_str) => {
            if let Ok(pid) = pid_str.trim().parse::<i32>() {
                println!("✅ Auth Service: Running (PID: {})", pid);
            } else {
                println!("❌ Auth Service: Invalid PID file");
            }
        }
        Err(_) => {
            println!("❌ Auth Service: PID file not found");
        }
    }

    // Disk usage
    match tokio::fs::metadata(data_dir).await {
        Ok(_) => {
            println!("✅ Data Directory: {}", data_dir);
        }
        Err(_) => {
            println!("❌ Data Directory: Not accessible");
        }
    }

    Ok(())
}