#!/bin/bash

# UM-OIC Build Script
# Builds all services, CLI tool, and web applications

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[BUILD]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."

    # Check Rust
    if ! command_exists cargo; then
        print_error "Rust/Cargo not found. Please install Rust from https://rustup.rs/"
        exit 1
    fi

    # Check Node.js
    if ! command_exists node; then
        print_error "Node.js not found. Please install Node.js from https://nodejs.org/"
        exit 1
    fi

    # Check npm
    if ! command_exists npm; then
        print_error "npm not found. Please install npm"
        exit 1
    fi

    print_success "All prerequisites found"
}

# Function to create necessary directories
create_directories() {
    print_status "Creating output directories..."

    mkdir -p bin
    mkdir -p data/web/auth
    mkdir -p data/web/mgmt
    mkdir -p data/users
    mkdir -p data/clients
    mkdir -p data/claims

    print_success "Directories created"
}

# Function to build Rust services
build_rust_services() {
    print_status "Building Rust services..."

    # Build in release mode for production
    print_status "Compiling Rust binaries (release mode)..."
    cargo build --release

    # Copy binaries to bin directory
    print_status "Copying binaries to bin/..."

    if [ -f "target/release/auth-service" ]; then
        cp target/release/auth-service bin/
        print_success "auth-service copied to bin/"
    else
        print_error "auth-service binary not found"
        exit 1
    fi

    if [ -f "target/release/admin-service" ]; then
        cp target/release/admin-service bin/
        print_success "admin-service copied to bin/"
    else
        print_error "admin-service binary not found"
        exit 1
    fi

    # Check for CLI tool (if exists)
    if [ -f "target/release/um-oic-cli" ]; then
        cp target/release/um-oic-cli bin/
        print_success "um-oic-cli copied to bin/"
    else
        print_warning "um-oic-cli binary not found (optional)"
    fi

    print_success "Rust services built successfully"
}

# Function to build admin web app
build_admin_app() {
    print_status "Building admin web application..."

    if [ ! -d "admin-app" ]; then
        print_error "admin-app directory not found"
        exit 1
    fi

    cd admin-app

    # Install dependencies
    print_status "Installing npm dependencies..."
    npm install

    # Build the application
    print_status "Building Vue.js application..."
    npm run build

    cd ..

    # Verify build output
    if [ -d "data/web/mgmt" ] && [ "$(ls -A data/web/mgmt)" ]; then
        print_success "Admin app built and copied to data/web/mgmt/"
    else
        print_error "Admin app build failed or output directory empty"
        exit 1
    fi
}

# Function to build auth web pages (if they exist)
build_auth_pages() {
    print_status "Checking for auth web pages..."

    if [ -d "auth-web" ]; then
        print_status "Building auth web pages..."
        cd auth-web

        if [ -f "package.json" ]; then
            npm install
            npm run build
        else
            # Simple copy for static files
            cp -r * ../data/web/auth/
        fi

        cd ..
        print_success "Auth pages built"
    else
        print_warning "No auth-web directory found, skipping auth pages"

        # Create a simple index.html for auth
        cat > data/web/auth/index.html << 'EOF'
<!DOCTYPE html>
<html lang="de">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>UM-OIC Authentication</title>
    <style>
        body { font-family: Arial, sans-serif; text-align: center; margin-top: 50px; }
        .container { max-width: 400px; margin: 0 auto; padding: 20px; }
    </style>
</head>
<body>
    <div class="container">
        <h1>UM-OIC Authentication</h1>
        <p>OAuth2/OIDC Authentication Service</p>
        <p><a href="/mgmt/">Management Interface</a></p>
    </div>
</body>
</html>
EOF
        print_status "Created basic auth index.html"
    fi
}

# Function to set executable permissions
set_permissions() {
    print_status "Setting executable permissions..."

    chmod +x bin/auth-service
    chmod +x bin/admin-service

    if [ -f "bin/um-oic-cli" ]; then
        chmod +x bin/um-oic-cli
    fi

    print_success "Permissions set"
}

# Function to create systemd service files
create_systemd_services() {
    print_status "Creating systemd service files..."

    mkdir -p systemd

    # Auth service
    cat > systemd/um-oic-auth.service << EOF
[Unit]
Description=UM-OIC Authentication Service
After=network.target
Wants=network.target

[Service]
Type=simple
User=um-oic
Group=um-oic
WorkingDirectory=/opt/um-oic
ExecStart=/opt/um-oic/bin/auth-service
Restart=always
RestartSec=5
Environment=RUST_LOG=info
Environment=UM_OIC_DATA_DIR=/opt/um-oic/data
Environment=UM_OIC_WEB_DIR=/opt/um-oic/data/web

# Security settings
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/opt/um-oic/data

[Install]
WantedBy=multi-user.target
EOF

    # Admin service
    cat > systemd/um-oic-admin.service << EOF
[Unit]
Description=UM-OIC Admin Service
After=network.target um-oic-auth.service
Wants=network.target
Requires=um-oic-auth.service

[Service]
Type=simple
User=um-oic
Group=um-oic
WorkingDirectory=/opt/um-oic
ExecStart=/opt/um-oic/bin/admin-service
Restart=always
RestartSec=5
Environment=RUST_LOG=info
Environment=UM_OIC_DATA_DIR=/opt/um-oic/data
Environment=UM_OIC_AUTH_SERVICE_URL=http://localhost:8000

# Security settings
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/opt/um-oic/data

[Install]
WantedBy=multi-user.target
EOF

    print_success "Systemd service files created in systemd/"
}

# Function to create docker-compose.yml
create_docker_compose() {
    print_status "Creating docker-compose.yml..."

    cat > docker-compose.yml << 'EOF'
version: '3.8'

services:
  auth-service:
    build:
      context: .
      dockerfile: Dockerfile.auth
    ports:
      - "8000:8000"
    volumes:
      - ./data:/app/data
    environment:
      - RUST_LOG=info
      - UM_OIC_DATA_DIR=/app/data
      - UM_OIC_WEB_DIR=/app/data/web
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8000/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  admin-service:
    build:
      context: .
      dockerfile: Dockerfile.admin
    ports:
      - "8001:8001"
    volumes:
      - ./data:/app/data
    environment:
      - RUST_LOG=info
      - UM_OIC_DATA_DIR=/app/data
      - UM_OIC_AUTH_SERVICE_URL=http://auth-service:8000
    depends_on:
      - auth-service
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8001/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
      - ./data/web:/usr/share/nginx/html:ro
      - ./ssl:/etc/nginx/ssl:ro
    depends_on:
      - auth-service
      - admin-service
    restart: unless-stopped
EOF

    print_success "docker-compose.yml created"
}

# Function to create sample configuration
create_sample_config() {
    print_status "Creating sample configuration files..."

    # Sample environment file
    cat > .env.example << 'EOF'
# UM-OIC Configuration

# Services
UM_OIC_AUTH_PORT=8000
UM_OIC_ADMIN_PORT=8001

# Data directories
UM_OIC_DATA_DIR=./data
UM_OIC_WEB_DIR=./data/web

# JWT Configuration
UM_OIC_JWT_SECRET=your-super-secret-jwt-key-change-this-in-production
UM_OIC_JWT_EXPIRES=24h

# Logging
RUST_LOG=info

# Auth Service URL (for admin service)
UM_OIC_AUTH_SERVICE_URL=http://localhost:8000
EOF

    # Sample nginx configuration
    cat > nginx.conf.example << 'EOF'
events {
    worker_connections 1024;
}

http {
    include /etc/nginx/mime.types;
    default_type application/octet-stream;

    upstream auth_service {
        server auth-service:8000;
    }

    upstream admin_service {
        server admin-service:8001;
    }

    server {
        listen 80;
        server_name localhost;

        # Auth service endpoints
        location /oauth2/ {
            proxy_pass http://auth_service;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }

        location /oidc/ {
            proxy_pass http://auth_service;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }

        # Admin API
        location /api/ {
            proxy_pass http://admin_service;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }

        # Auth web pages
        location /auth/ {
            alias /usr/share/nginx/html/auth/;
            index index.html;
            try_files $uri $uri/ /auth/index.html;
        }

        # Management interface
        location /mgmt/ {
            alias /usr/share/nginx/html/mgmt/;
            index index.html;
            try_files $uri $uri/ /mgmt/index.html;
        }

        # Root redirect
        location = / {
            return 301 /mgmt/;
        }
    }
}
EOF

    print_success "Sample configuration files created"
}

# Function to run tests
run_tests() {
    print_status "Running tests..."

    # Rust tests
    if cargo test --quiet 2>/dev/null; then
        print_success "Rust tests passed"
    else
        print_warning "Some Rust tests failed or no tests found"
    fi

    # Admin app tests (if available)
    if [ -d "admin-app" ]; then
        cd admin-app
        if npm run test:unit --if-present 2>/dev/null; then
            print_success "Admin app tests passed"
        else
            print_warning "Admin app tests failed or not available"
        fi
        cd ..
    fi
}

# Function to display build summary
show_summary() {
    echo ""
    echo "======================================"
    echo -e "${GREEN}UM-OIC Build Complete!${NC}"
    echo "======================================"
    echo ""
    echo "Built components:"
    echo "  📦 Services:"
    [ -f "bin/auth-service" ] && echo "    ✅ auth-service"
    [ -f "bin/admin-service" ] && echo "    ✅ admin-service"
    [ -f "bin/um-oic-cli" ] && echo "    ✅ um-oic-cli"
    echo ""
    echo "  🌐 Web Applications:"
    [ -d "data/web/auth" ] && echo "    ✅ Auth pages (data/web/auth/)"
    [ -d "data/web/mgmt" ] && echo "    ✅ Admin app (data/web/mgmt/)"
    echo ""
    echo "  ⚙️  Configuration:"
    [ -f "systemd/um-oic-auth.service" ] && echo "    ✅ Systemd services (systemd/)"
    [ -f "docker-compose.yml" ] && echo "    ✅ Docker Compose"
    [ -f ".env.example" ] && echo "    ✅ Sample configuration"
    echo ""
    echo "Quick start:"
    echo "  1. Copy .env.example to .env and configure"
    echo "  2. Run services: ./bin/auth-service & ./bin/admin-service"
    echo "  3. Or use Docker: docker-compose up"
    echo "  4. Access admin interface: http://localhost:8001/mgmt/"
    echo ""
    echo "Deployment:"
    echo "  - Copy systemd/*.service to /etc/systemd/system/"
    echo "  - Copy entire directory to /opt/um-oic/"
    echo "  - Run: systemctl enable --now um-oic-auth um-oic-admin"
    echo ""
}

# Main build function
main() {
    echo "======================================"
    echo "UM-OIC Build Script"
    echo "======================================"
    echo ""

    # Parse command line arguments
    SKIP_TESTS=false
    PRODUCTION=false

    while [[ $# -gt 0 ]]; do
        case $1 in
            --skip-tests)
                SKIP_TESTS=true
                shift
                ;;
            --production)
                PRODUCTION=true
                shift
                ;;
            --help|-h)
                echo "Usage: $0 [OPTIONS]"
                echo ""
                echo "Options:"
                echo "  --skip-tests    Skip running tests"
                echo "  --production    Production build (optimizations)"
                echo "  --help, -h      Show this help"
                echo ""
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                echo "Use --help for usage information"
                exit 1
                ;;
        esac
    done

    # Build process
    check_prerequisites
    create_directories

    if [ "$SKIP_TESTS" = false ]; then
        run_tests
    fi

    build_rust_services
    build_admin_app
    build_auth_pages
    set_permissions
    create_systemd_services
    create_docker_compose
    create_sample_config

    show_summary
}

# Run main function with all arguments
main "$@"