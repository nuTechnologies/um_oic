# UM-OIC Test Suite Makefile

.PHONY: all test test-auth test-admin test-integration test-security test-performance clean setup help

# Default target
all: test

# Colors for output
BLUE=\033[0;34m
GREEN=\033[0;32m
RED=\033[0;31m
NC=\033[0m # No Color

# Test configuration
TEST_DIR := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
PROJECT_ROOT := $(shell dirname $(TEST_DIR))

help: ## Show this help message
	@echo "UM-OIC Test Suite"
	@echo "================="
	@echo ""
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

setup: ## Setup test environment and build services
	@echo -e "$(BLUE)Setting up test environment...$(NC)"
	@cd $(PROJECT_ROOT) && cargo build
	@mkdir -p $(TEST_DIR)/data/{auth,admin}
	@echo -e "$(GREEN)✓ Test environment ready$(NC)"

test: setup ## Run all tests
	@echo -e "$(BLUE)Running all tests...$(NC)"
	@$(TEST_DIR)/run-tests.sh all

test-auth: setup ## Run auth-service tests only
	@echo -e "$(BLUE)Running auth-service tests...$(NC)"
	@$(TEST_DIR)/run-tests.sh auth-service

test-admin: setup ## Run admin-service tests only
	@echo -e "$(BLUE)Running admin-service tests...$(NC)"
	@$(TEST_DIR)/run-tests.sh admin-service

test-integration: setup ## Run integration tests only
	@echo -e "$(BLUE)Running integration tests...$(NC)"
	@$(TEST_DIR)/run-tests.sh integration

test-security: setup ## Run security tests
	@echo -e "$(BLUE)Running security tests...$(NC)"
	@$(TEST_DIR)/security/security-tests.sh

test-performance: setup ## Run performance benchmarks
	@echo -e "$(BLUE)Running performance benchmarks...$(NC)"
	@$(TEST_DIR)/performance/bench.sh

test-ci: setup ## Run tests suitable for CI/CD
	@echo -e "$(BLUE)Running CI tests...$(NC)"
	@$(TEST_DIR)/auth-service/test-auth.sh
	@$(TEST_DIR)/admin-service/test-admin.sh
	@$(TEST_DIR)/integration/test-integration.sh

test-quick: ## Quick health check tests
	@echo -e "$(BLUE)Running quick tests...$(NC)"
	@curl -sf http://localhost:8080/health > /dev/null && echo -e "$(GREEN)✓ Auth service healthy$(NC)" || echo -e "$(RED)✗ Auth service not responding$(NC)"
	@curl -sf http://localhost:8081/health > /dev/null && echo -e "$(GREEN)✓ Admin service healthy$(NC)" || echo -e "$(RED)✗ Admin service not responding$(NC)"

clean: ## Clean test data and temporary files
	@echo -e "$(BLUE)Cleaning test data...$(NC)"
	@rm -rf $(TEST_DIR)/data/{auth,admin}/*
	@rm -f /tmp/login_data.json /tmp/auth_header.txt /tmp/resource_usage.csv
	@pkill -f "auth-service" || true
	@pkill -f "admin-service" || true
	@echo -e "$(GREEN)✓ Test data cleaned$(NC)"

# Development targets
dev-setup: setup ## Setup development test environment
	@echo -e "$(BLUE)Setting up development environment...$(NC)"
	@if [ ! -f $(TEST_DIR)/data/auth/users.json ]; then \
		cp $(TEST_DIR)/fixtures/* $(TEST_DIR)/data/auth/; \
		cp $(TEST_DIR)/fixtures/* $(TEST_DIR)/data/admin/; \
	fi
	@echo -e "$(GREEN)✓ Development environment ready$(NC)"

dev-start: dev-setup ## Start services for development testing
	@echo -e "$(BLUE)Starting development services...$(NC)"
	@cd $(PROJECT_ROOT) && \
		RUST_LOG=debug PORT=8080 DATA_DIR="$(TEST_DIR)/data/auth" ./target/debug/auth-service &
	@cd $(PROJECT_ROOT) && \
		RUST_LOG=debug PORT=8081 DATA_DIR="$(TEST_DIR)/data/admin" AUTH_SERVICE_URL="http://localhost:8080" ./target/debug/admin-service &
	@sleep 3
	@echo -e "$(GREEN)✓ Services started$(NC)"
	@echo "Auth service: http://localhost:8080"
	@echo "Admin service: http://localhost:8081"

dev-stop: ## Stop development services
	@echo -e "$(BLUE)Stopping development services...$(NC)"
	@pkill -f "auth-service" || true
	@pkill -f "admin-service" || true
	@echo -e "$(GREEN)✓ Services stopped$(NC)"

# Monitoring targets
logs-auth: ## Show auth-service logs
	@journalctl -f -u auth-service || tail -f /var/log/auth-service.log || echo "No logs available"

logs-admin: ## Show admin-service logs
	@journalctl -f -u admin-service || tail -f /var/log/admin-service.log || echo "No logs available"

status: ## Show service status
	@echo -e "$(BLUE)Service Status:$(NC)"
	@curl -sf http://localhost:8080/health && echo -e "$(GREEN)✓ Auth service: OK$(NC)" || echo -e "$(RED)✗ Auth service: DOWN$(NC)"
	@curl -sf http://localhost:8081/health && echo -e "$(GREEN)✓ Admin service: OK$(NC)" || echo -e "$(RED)✗ Admin service: DOWN$(NC)"

# Documentation targets
test-report: ## Generate test report
	@echo -e "$(BLUE)Generating test report...$(NC)"
	@echo "# UM-OIC Test Report" > $(TEST_DIR)/test-report.md
	@echo "Generated: $(shell date)" >> $(TEST_DIR)/test-report.md
	@echo "" >> $(TEST_DIR)/test-report.md
	@echo "## Test Coverage" >> $(TEST_DIR)/test-report.md
	@echo "- Auth Service Tests: $(shell ls $(TEST_DIR)/auth-service/test-*.sh | wc -l)" >> $(TEST_DIR)/test-report.md
	@echo "- Admin Service Tests: $(shell ls $(TEST_DIR)/admin-service/test-*.sh | wc -l)" >> $(TEST_DIR)/test-report.md
	@echo "- Integration Tests: $(shell ls $(TEST_DIR)/integration/test-*.sh | wc -l)" >> $(TEST_DIR)/test-report.md
	@echo "- Security Tests: $(shell ls $(TEST_DIR)/security/*.sh | wc -l)" >> $(TEST_DIR)/test-report.md
	@echo "- Performance Tests: $(shell ls $(TEST_DIR)/performance/*.sh | wc -l)" >> $(TEST_DIR)/test-report.md
	@echo -e "$(GREEN)✓ Test report generated: $(TEST_DIR)/test-report.md$(NC)"

# Validation targets
validate-fixtures: ## Validate test fixture files
	@echo -e "$(BLUE)Validating test fixtures...$(NC)"
	@for file in $(TEST_DIR)/fixtures/*.json; do \
		echo -n "Validating $$file... "; \
		if jq . "$$file" > /dev/null 2>&1; then \
			echo -e "$(GREEN)OK$(NC)"; \
		else \
			echo -e "$(RED)INVALID$(NC)"; \
			exit 1; \
		fi; \
	done
	@echo -e "$(GREEN)✓ All fixtures valid$(NC)"

# Docker targets (if using containers)
docker-test: ## Run tests in Docker containers
	@echo -e "$(BLUE)Running tests in Docker...$(NC)"
	@cd $(PROJECT_ROOT) && docker-compose -f docker-compose.test.yml up --build --abort-on-container-exit

docker-clean: ## Clean Docker test resources
	@cd $(PROJECT_ROOT) && docker-compose -f docker-compose.test.yml down -v --remove-orphans || true

# Load testing targets
load-test-light: setup ## Run light load test
	@echo -e "$(BLUE)Running light load test...$(NC)"
	@$(TEST_DIR)/performance/bench.sh -c 5 -n 20

load-test-heavy: setup ## Run heavy load test
	@echo -e "$(BLUE)Running heavy load test...$(NC)"
	@$(TEST_DIR)/performance/bench.sh -c 50 -n 100

# Continuous testing
watch: ## Watch for changes and run tests
	@echo -e "$(BLUE)Watching for changes...$(NC)"
	@while inotifywait -r -e modify,create,delete $(PROJECT_ROOT)/auth-service $(PROJECT_ROOT)/admin-service 2>/dev/null; do \
		echo -e "$(BLUE)Changes detected, running tests...$(NC)"; \
		make test-quick; \
		sleep 2; \
	done

# Debug targets
debug-auth: ## Start auth-service with debug output
	@cd $(PROJECT_ROOT) && RUST_LOG=debug PORT=8080 DATA_DIR="$(TEST_DIR)/data/auth" ./target/debug/auth-service

debug-admin: ## Start admin-service with debug output
	@cd $(PROJECT_ROOT) && RUST_LOG=debug PORT=8081 DATA_DIR="$(TEST_DIR)/data/admin" AUTH_SERVICE_URL="http://localhost:8080" ./target/debug/admin-service

# Database/Storage targets
reset-data: ## Reset test data to fixtures
	@echo -e "$(BLUE)Resetting test data...$(NC)"
	@rm -rf $(TEST_DIR)/data/{auth,admin}/*
	@cp $(TEST_DIR)/fixtures/* $(TEST_DIR)/data/auth/ 2>/dev/null || true
	@cp $(TEST_DIR)/fixtures/* $(TEST_DIR)/data/admin/ 2>/dev/null || true
	@echo -e "$(GREEN)✓ Test data reset$(NC)"

backup-data: ## Backup current test data
	@echo -e "$(BLUE)Backing up test data...$(NC)"
	@tar -czf $(TEST_DIR)/test-data-backup-$(shell date +%Y%m%d-%H%M%S).tar.gz -C $(TEST_DIR)/data .
	@echo -e "$(GREEN)✓ Test data backed up$(NC)"