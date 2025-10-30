# UM-OIC Build System

.PHONY: all build install clean dev test lint format docker admin-app help

# Default target
all: build install admin-app

# Build all Rust services in release mode
build:
	@echo "Building all Rust services..."
	cargo build --release --workspace

# Install binaries to bin/ directory
install: build
	@echo "Installing binaries to bin/..."
	@mkdir -p bin
	cp target/release/auth-service bin/
	cp target/release/admin-service bin/
	cp target/release/auth-ops bin/
	@chmod +x bin/*
	@echo "Binaries installed:"
	@ls -la bin/

# Build Vue.js admin app
admin-app:
	@echo "Building admin app..."
	cd admin-app && npm ci && npm run build
	@echo "Admin app built to data/web/mgmt/"

# Development mode - start all services
dev:
	@echo "Starting development servers..."
	@echo "Starting auth-service on port 8000..."
	@RUST_LOG=debug cargo run -p auth-service &
	@echo "Starting admin-service on port 8001..."
	@RUST_LOG=debug cargo run -p admin-service &
	@echo "Starting admin-app dev server on port 3001..."
	@cd admin-app && npm run dev &
	@echo "All services started. Press Ctrl+C to stop."
	@wait

# Run tests
test:
	@echo "Running tests..."
	cargo test --workspace

# Lint code
lint:
	@echo "Linting Rust code..."
	cargo clippy --workspace -- -D warnings
	@echo "Linting admin app..."
	cd admin-app && npm run lint

# Format code
format:
	@echo "Formatting Rust code..."
	cargo fmt --all
	@echo "Formatting admin app..."
	cd admin-app && npm run format

# Build Docker images
docker:
	@echo "Building Docker images..."
	docker build -f auth-service/Dockerfile -t um-oic/auth-service .
	docker build -f admin-service/Dockerfile -t um-oic/admin-service .
	docker build -f auth-ops/Dockerfile -t um-oic/auth-ops .

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -rf bin/*
	rm -rf data/web/mgmt/*
	cd admin-app && rm -rf node_modules dist

# Setup development environment
setup:
	@echo "Setting up development environment..."
	@echo "Installing Rust dependencies..."
	cargo fetch
	@echo "Installing Node.js dependencies..."
	cd admin-app && npm install
	@echo "Creating data directories..."
	mkdir -p data/{users/all,web/{auth,mgmt},certs}
	@echo "Setup complete!"

# Quick deploy (build + install)
deploy: build install admin-app
	@echo "Deployment ready!"
	@echo "Auth service: ./bin/auth-service"
	@echo "Admin service: ./bin/admin-service"
	@echo "CLI tools: ./bin/auth-ops"
	@echo "Admin UI: data/web/mgmt/"

# Help
help:
	@echo "UM-OIC Build System"
	@echo ""
	@echo "Available targets:"
	@echo "  all         - Build everything (Rust + Vue app)"
	@echo "  build       - Build Rust services only"
	@echo "  install     - Install binaries to bin/"
	@echo "  admin-app   - Build Vue.js admin application"
	@echo "  dev         - Start development servers"
	@echo "  test        - Run all tests"
	@echo "  lint        - Lint all code"
	@echo "  format      - Format all code"
	@echo "  docker      - Build Docker images"
	@echo "  clean       - Clean build artifacts"
	@echo "  setup       - Setup development environment"
	@echo "  deploy      - Quick deployment build"
	@echo "  help        - Show this help"