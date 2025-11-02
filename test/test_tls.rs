use std::env;

mod tls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("TLS_CERT_PATH", "./certs/test-cert.pem");
    env::set_var("TLS_KEY_PATH", "./certs/test-key.pem");
    env::set_var("DOMAIN", "test.localhost");

    let tls_manager = tls::TlsManager::from_env();

    match tls_manager.create_rustls_config().await {
        Ok(_) => {
            println!("✓ TLS configuration created successfully");
            println!("✓ Self-signed certificates generated");
        }
        Err(e) => {
            eprintln!("✗ TLS configuration failed: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
