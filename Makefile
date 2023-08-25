#!make

help: ## Display this help screen
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; \
		{printf "%-15s %s\n", $$1, $$2}' | \
		sort

readme: ## Write README.md
	@./dev/readme.sh

typos: ## Check typos
	@typos

typos-fix: ## Fix typos
	@typos -w

audit: ## Audit Cargo.lock
	@cargo audit

audit-fix: ## Update Cargo.toml to fix vulnerable dependency requirement
	@cargo audit fix

bench: ## Run benchmarks
	@./dev/bench.sh

build: ## Build binary
	@cargo build --release --locked --frozen --bins

cargo-deps: ## Install cargo dependencies
	@cargo install --locked cargo-outdated
	@cargo install cargo-audit --features=fix
	@cargo install cargo-bump
	@cargo install cargo-udeps --locked
	@cargo install cargo-watch
	@cargo install exa
	@cargo install bat
	@cargo install ripgrep
	@cargo install sd
	@cargo install typos-cli
	@rustup component add clippy

dev: ## Run check in watch mode
	@cargo watch -c

check: ## Run check
	@cargo check

doc: ## Open app documentation
	@cargo doc --open

fix: ## Fix rust code
	@cargo fix --allow-dirty --allow-staged --all-features --all-targets

fmt: ## Format rust code
	@cargo fmt --all --check

install: ## Install binary
	@cargo install --path .

uninstall: ## Uninstall binary
	@cargo uninstall

lint: ## Lint rust code
	@cargo clippy --workspace --all-targets --all-features --no-deps -- -D warnings

lint-fix: ## Fix lint rust code
	@cargo clippy --workspace --all-targets --all-features --no-deps --allow-dirty --allow-staged --fix -- -D warnings

outdated: ## Display when dependencies are out of date
	@cargo outdated -wR

tests: ## Run tests
	@cargo test --lib

update: ## Update dependencies
	@cargo update

update-rustup:
	@rustup update

bash-all: bash-fmt bash-check bash-lint ## Run all bash tests

bash-fmt: ## Format bash code
	@find . -type f -name "*.sh" | xargs shfmt -i 2 -w

bash-check: ## Check format bash code
	@find . -type f -name "*.sh" | xargs shfmt -i 2 -d

bash-lint: ## Check lint bash code
	@find . -type f -name "*.sh" | xargs shellcheck -o all

.PHONY: help
.PHONY: audit
.PHONY: audit-fix
.PHONY: bash-all
.PHONY: bash-check
.PHONY: bash-fmt
.PHONY: bash-lint
.PHONY: bench
.PHONY: build
.PHONY: cargo-deps
.PHONY: check
.PHONY: dev
.PHONY: doc
.PHONY: fix
.PHONY: fmt
.PHONY: install
.PHONY: lint
.PHONY: lint-fix
.PHONY: outdated
.PHONY: readme
.PHONY: release
.PHONY: tests
.PHONY: typos
.PHONY: typos-fix
.PHONY: uninstall
.PHONY: update
.PHONY: update-rustup
