use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use time::OffsetDateTime;
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupManifest {
    pub created_at: OffsetDateTime,
    pub version: String,
    pub files: Vec<String>,
    pub checksums: std::collections::HashMap<String, String>,
}

pub async fn backup_data(data_dir: &str, output_dir: &str) -> Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs();

    let backup_path = format!("{}/backup-{}", output_dir, timestamp);

    info!("Creating backup at {}", backup_path);

    // Create backup directory
    tokio::fs::create_dir_all(&backup_path).await
        .context("Failed to create backup directory")?;

    // Files to backup
    let files = vec!["users.json", "groups.json", "roles.json", "clients.json"];
    let mut checksums = std::collections::HashMap::new();

    // Copy data files
    for file in &files {
        let src = format!("{}/{}", data_dir, file);
        let dst = format!("{}/{}", backup_path, file);

        match tokio::fs::copy(&src, &dst).await {
            Ok(_) => {
                // Calculate checksum
                let checksum = calculate_file_checksum(&dst).await?;
                checksums.insert(file.clone(), checksum);
                println!("✅ Backed up: {}", file);
            }
            Err(e) => {
                println!("⚠️  Failed to backup {}: {}", file, e);
            }
        }
    }

    // Copy audit logs directory
    let audit_src = format!("{}/audit", data_dir);
    let audit_dst = format!("{}/audit", backup_path);

    match copy_dir_recursive(&audit_src, &audit_dst).await {
        Ok(_) => {
            println!("✅ Backed up: audit logs");
        }
        Err(e) => {
            println!("⚠️  Failed to backup audit logs: {}", e);
        }
    }

    // Create manifest
    let manifest = BackupManifest {
        created_at: OffsetDateTime::now_utc(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        files: files.iter().map(|s| s.to_string()).collect(),
        checksums,
    };

    let manifest_path = format!("{}/manifest.json", backup_path);
    let manifest_content = serde_json::to_string_pretty(&manifest)?;
    tokio::fs::write(&manifest_path, manifest_content).await
        .context("Failed to write manifest")?;

    println!("✅ Backup created: {}", backup_path);
    println!("   Manifest: {}", manifest_path);

    Ok(())
}

pub async fn restore_data(backup_dir: &str, data_dir: &str) -> Result<()> {
    info!("Restoring data from {} to {}", backup_dir, data_dir);

    // Read manifest
    let manifest_path = format!("{}/manifest.json", backup_dir);
    let manifest_content = tokio::fs::read_to_string(&manifest_path).await
        .context("Failed to read backup manifest")?;

    let manifest: BackupManifest = serde_json::from_str(&manifest_content)
        .context("Failed to parse backup manifest")?;

    println!("📦 Restoring backup from {}", manifest.created_at);
    println!("   Version: {}", manifest.version);

    // Verify checksums before restore
    for file in &manifest.files {
        let backup_file = format!("{}/{}", backup_dir, file);
        let checksum = calculate_file_checksum(&backup_file).await?;

        if let Some(expected_checksum) = manifest.checksums.get(file) {
            if checksum != *expected_checksum {
                return Err(anyhow::anyhow!(
                    "Checksum mismatch for {}: expected {}, got {}",
                    file, expected_checksum, checksum
                ));
            }
        }
    }

    println!("✅ Backup integrity verified");

    // Create data directory
    tokio::fs::create_dir_all(data_dir).await
        .context("Failed to create data directory")?;

    // Restore files
    for file in &manifest.files {
        let src = format!("{}/{}", backup_dir, file);
        let dst = format!("{}/{}", data_dir, file);

        tokio::fs::copy(&src, &dst).await
            .with_context(|| format!("Failed to restore {}", file))?;

        println!("✅ Restored: {}", file);
    }

    // Restore audit logs
    let audit_src = format!("{}/audit", backup_dir);
    let audit_dst = format!("{}/audit", data_dir);

    if tokio::fs::metadata(&audit_src).await.is_ok() {
        copy_dir_recursive(&audit_src, &audit_dst).await
            .context("Failed to restore audit logs")?;
        println!("✅ Restored: audit logs");
    }

    println!("✅ Restore completed successfully");

    Ok(())
}

async fn calculate_file_checksum(path: &str) -> Result<String> {
    use sha2::{Sha256, Digest};

    let content = tokio::fs::read(path).await
        .with_context(|| format!("Failed to read file for checksum: {}", path))?;

    let mut hasher = Sha256::new();
    hasher.update(&content);
    let result = hasher.finalize();

    Ok(format!("{:x}", result))
}

async fn copy_dir_recursive(src: &str, dst: &str) -> Result<()> {
    tokio::fs::create_dir_all(dst).await
        .context("Failed to create destination directory")?;

    let mut entries = tokio::fs::read_dir(src).await
        .context("Failed to read source directory")?;

    while let Some(entry) = entries.next_entry().await? {
        let entry_path = entry.path();
        let file_name = entry.file_name();
        let dst_path = format!("{}/{}", dst, file_name.to_string_lossy());

        if entry.file_type().await?.is_dir() {
            copy_dir_recursive(&entry_path.to_string_lossy(), &dst_path).await?;
        } else {
            tokio::fs::copy(&entry_path, &dst_path).await
                .with_context(|| format!("Failed to copy file: {:?}", entry_path))?;
        }
    }

    Ok(())
}