#!/bin/bash
set -euo pipefail

# Installation script for auth system systemd services

echo "🔧 Installing Auth System Services"

# Check if running as root
if [[ $EUID -ne 0 ]]; then
   echo "❌ This script must be run as root"
   exit 1
fi

# Configuration
AUTH_USER="auth"
AUTH_GROUP="auth"
AUTH_HOME="/var/lib/auth"
CONFIG_DIR="/etc/auth"
SSL_DIR="/etc/ssl/auth"

echo "📁 Creating directories and user..."

# Create auth user and group
if ! getent group $AUTH_GROUP > /dev/null 2>&1; then
    groupadd --system $AUTH_GROUP
    echo "✅ Created group: $AUTH_GROUP"
fi

if ! getent passwd $AUTH_USER > /dev/null 2>&1; then
    useradd --system --gid $AUTH_GROUP --home-dir $AUTH_HOME \
            --shell /usr/sbin/nologin --comment "Auth System User" $AUTH_USER
    echo "✅ Created user: $AUTH_USER"
fi

# Create directories
mkdir -p $AUTH_HOME/{data,static}
mkdir -p $CONFIG_DIR
mkdir -p $SSL_DIR
mkdir -p /var/log/auth
mkdir -p /var/run

# Set permissions
chown -R $AUTH_USER:$AUTH_GROUP $AUTH_HOME
chown -R $AUTH_USER:$AUTH_GROUP $CONFIG_DIR
chown -R root:$AUTH_GROUP $SSL_DIR
chmod 750 $SSL_DIR
chmod 755 $AUTH_HOME $CONFIG_DIR

echo "✅ Directories created and permissions set"

# Copy systemd service files
echo "📋 Installing systemd service files..."

cp systemd/auth-service.service /etc/systemd/system/
cp systemd/admin-service.service /etc/systemd/system/

# Reload systemd
systemctl daemon-reload

echo "✅ Systemd services installed"

# Create default configuration if it doesn't exist
if [[ ! -f "$CONFIG_DIR/config.toml" ]]; then
    echo "📝 Creating default configuration..."

    cat > "$CONFIG_DIR/config.toml" << EOF
# Auth Service Configuration
jwt_secret = "$(openssl rand -base64 64)"

[instance]
name = "Auth Service"
logo_url = "/img/logo.png"
primary_color = "#00529F"
issuer = "https://auth.example.com"

[security]
password_min_length = 12
access_token_ttl = 3600
refresh_token_ttl = 2592000
require_mfa = false

[features]
allow_registration = false
allow_password_reset = true
EOF

    chown $AUTH_USER:$AUTH_GROUP "$CONFIG_DIR/config.toml"
    chmod 640 "$CONFIG_DIR/config.toml"

    echo "✅ Default configuration created at $CONFIG_DIR/config.toml"
    echo "⚠️  Please update the configuration with your settings!"
fi

# Create sample data files if they don't exist
if [[ ! -f "$AUTH_HOME/data/users.json" ]]; then
    echo "📊 Creating sample data files..."

    mkdir -p "$AUTH_HOME/data/audit"

    # Create default admin user with password "password123"
    # Hash: $argon2id$v=19$m=19456,t=2,p=1$8qwOOxUlm7OQgVzggDL5WA$Nqs4AuB6c2QeFlcv9l5gQz8hHwP9iZQ+YLkQbA+F9nY
    cat > "$AUTH_HOME/data/users.json" << 'EOF'
{
  "users": [
    {
      "id": "user-admin",
      "email": "admin@example.com",
      "password_hash": "$argon2id$v=19$m=19456,t=2,p=1$8qwOOxUlm7OQgVzggDL5WA$Nqs4AuB6c2QeFlcv9l5gQz8hHwP9iZQ+YLkQbA+F9nY",
      "first_name": "System",
      "last_name": "Administrator",
      "status": "active",
      "roles": ["admin"],
      "group_memberships": [],
      "custom_claims": {},
      "mfa_secret": null,
      "created_at": "2025-01-01T00:00:00Z",
      "updated_at": "2025-01-01T00:00:00Z"
    }
  ]
}
EOF

    cat > "$AUTH_HOME/data/groups.json" << 'EOF'
{
  "groups": []
}
EOF

    cat > "$AUTH_HOME/data/roles.json" << 'EOF'
{
  "roles": [
    {
      "id": "admin",
      "name": "Administrator",
      "description": "Full system access",
      "permissions": ["*"]
    }
  ]
}
EOF

    cat > "$AUTH_HOME/data/clients.json" << 'EOF'
{
  "clients": []
}
EOF

    chown -R $AUTH_USER:$AUTH_GROUP "$AUTH_HOME/data"
    echo "✅ Sample data files created"
    echo "ℹ️  Default admin user: admin@example.com / password123"
fi

# Generate self-signed certificates if they don't exist
if [[ ! -f "$SSL_DIR/cert.pem" ]]; then
    echo "🔐 Generating self-signed SSL certificates..."

    openssl req -x509 -newkey rsa:4096 -keyout "$SSL_DIR/key.pem" \
                -out "$SSL_DIR/cert.pem" -days 365 -nodes \
                -subj "/C=DE/ST=Saxony/L=Leipzig/O=Auth System/CN=auth.example.com"

    chown root:$AUTH_GROUP "$SSL_DIR/key.pem" "$SSL_DIR/cert.pem"
    chmod 640 "$SSL_DIR/key.pem"
    chmod 644 "$SSL_DIR/cert.pem"

    echo "✅ SSL certificates generated"
fi

echo ""
echo "🎉 Installation completed successfully!"
echo ""
echo "Next steps:"
echo "1. Update configuration: $CONFIG_DIR/config.toml"
echo "2. Install binaries: cp target/release/{auth-service,admin-service,auth-ops} /usr/local/bin/"
echo "3. Enable services: systemctl enable auth-service admin-service"
echo "4. Start services: systemctl start auth-service admin-service"
echo "5. Check status: systemctl status auth-service admin-service"
echo ""
echo "Default admin login: admin@example.com / password123"
echo "Auth service will run on: http://localhost:8000"
echo "Admin service will run on: http://localhost:8001"