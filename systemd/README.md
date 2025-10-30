# Systemd Installation Guide

## Quick Installation

```bash
# Run as root
sudo ./install.sh
```

## Manual Installation

### 1. Create User and Directories

```bash
# Create system user
sudo groupadd --system auth
sudo useradd --system --gid auth --home-dir /var/lib/auth \
             --shell /usr/sbin/nologin auth

# Create directories
sudo mkdir -p /var/lib/auth/{data,static}
sudo mkdir -p /etc/auth
sudo mkdir -p /etc/ssl/auth
sudo mkdir -p /var/log/auth

# Set permissions
sudo chown -R auth:auth /var/lib/auth
sudo chown -R auth:auth /etc/auth
sudo chown -R root:auth /etc/ssl/auth
sudo chmod 750 /etc/ssl/auth
```

### 2. Install Binaries

```bash
# Build services
cargo build --release

# Install binaries
sudo cp target/release/auth-service /usr/local/bin/
sudo cp target/release/admin-service /usr/local/bin/
sudo cp target/release/auth-ops /usr/local/bin/

# Set permissions
sudo chmod 755 /usr/local/bin/{auth-service,admin-service,auth-ops}
```

### 3. Install Systemd Services

```bash
# Copy service files
sudo cp systemd/auth-service.service /etc/systemd/system/
sudo cp systemd/admin-service.service /etc/systemd/system/

# Reload systemd
sudo systemctl daemon-reload
```

### 4. Configure Services

```bash
# Create configuration
sudo cp auth-service/config.toml /etc/auth/config.toml
sudo chown auth:auth /etc/auth/config.toml
sudo chmod 640 /etc/auth/config.toml

# Edit configuration
sudo nano /etc/auth/config.toml
```

### 5. Setup Data

```bash
# Copy sample data
sudo cp -r data/* /var/lib/auth/data/
sudo chown -R auth:auth /var/lib/auth/data
```

### 6. Generate SSL Certificates

```bash
# Self-signed certificates (development)
sudo openssl req -x509 -newkey rsa:4096 \
    -keyout /etc/ssl/auth/key.pem \
    -out /etc/ssl/auth/cert.pem \
    -days 365 -nodes \
    -subj "/C=DE/ST=Saxony/L=Leipzig/O=Auth System/CN=auth.example.com"

sudo chown root:auth /etc/ssl/auth/{key.pem,cert.pem}
sudo chmod 640 /etc/ssl/auth/key.pem
sudo chmod 644 /etc/ssl/auth/cert.pem
```

## Service Management

### Enable and Start Services

```bash
# Enable services
sudo systemctl enable auth-service
sudo systemctl enable admin-service

# Start services
sudo systemctl start auth-service
sudo systemctl start admin-service
```

### Check Status

```bash
# Check status
sudo systemctl status auth-service
sudo systemctl status admin-service

# View logs
sudo journalctl -u auth-service -f
sudo journalctl -u admin-service -f
```

### Service Operations

```bash
# Restart services
sudo systemctl restart auth-service
sudo systemctl restart admin-service

# Reload auth service configuration (SIGHUP)
sudo systemctl reload auth-service

# Stop services
sudo systemctl stop admin-service
sudo systemctl stop auth-service
```

## CLI Operations

### Using auth-ops

```bash
# Show system status
auth-ops status

# Create a user
auth-ops user create \
  --email user@example.com \
  --password "secure-password" \
  --first-name "John" \
  --last-name "Doe" \
  --roles staff

# List users
auth-ops user list

# Backup data
auth-ops backup --output-dir /backup

# Reload auth service
auth-ops reload
```

## Troubleshooting

### Check Permissions

```bash
# Verify file permissions
ls -la /var/lib/auth/
ls -la /etc/auth/
ls -la /etc/ssl/auth/

# Check process ownership
ps aux | grep auth-service
```

### Verify Configuration

```bash
# Test configuration
auth-service --config /etc/auth/config.toml --help
```

### Network Issues

```bash
# Check listening ports
sudo netstat -tlnp | grep -E ':(8000|8001)'

# Test connectivity
curl http://localhost:8000/health
curl http://localhost:8001/health
```

### Log Analysis

```bash
# View all auth-related logs
sudo journalctl -u auth-service -u admin-service --since "1 hour ago"

# Follow logs in real-time
sudo journalctl -u auth-service -f

# Filter for specific events
sudo journalctl -u auth-service | grep "login"
```

## Security Considerations

### File Permissions

- Configuration files: `640` (auth:auth)
- SSL private key: `640` (root:auth)
- SSL certificate: `644` (root:auth)
- Data directory: `755` (auth:auth)
- Data files: `644` (auth:auth)

### Network Security

- Bind services to specific interfaces in production
- Use reverse proxy (Nginx/Caddy) for SSL termination
- Configure firewall to limit access
- Use proper SSL certificates in production

### System Security

- Services run as unprivileged `auth` user
- SystemD security hardening enabled
- No shell access for service user
- Restricted file system access