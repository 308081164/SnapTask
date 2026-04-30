.PHONY: dev build server lint clean install help

# Default target
help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

# Development
dev: ## Start development server (frontend + Tauri)
	npm run tauri dev

# Build
build: ## Build production version
	npm run tauri build

# Server
server: ## Start cloud sync server
	cd server && npm run dev

server-build: ## Build sync server
	cd server && npm run build

server-start: ## Start sync server (production)
	cd server && npm run start

server-install: ## Install sync server dependencies
	cd server && npm install

# Docker
docker-build: ## Build Docker image for sync server
	cd server && docker compose build

docker-up: ## Start sync server with Docker
	cd server && docker compose up -d

docker-down: ## Stop sync server Docker containers
	cd server && docker compose down

docker-logs: ## View sync server Docker logs
	cd server && docker compose logs -f

# Code Quality
lint: ## Run code linting
	npm run lint

lint-server: ## Lint sync server code
	cd server && npm run lint

# Cleanup
clean: ## Clean all build artifacts
	rm -rf dist/
	rm -rf src-tauri/target/
	rm -rf server/dist/
	rm -rf server/node_modules/
	rm -rf node_modules/

# Install
install: ## Install all dependencies
	npm install
	cd server && npm install

# Format
format: ## Format code with prettier
	npx prettier --write "src/**/*.{ts,tsx,vue,css,json}"
