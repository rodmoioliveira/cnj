#!make

help: ## Display this help screen
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; \
		{printf "%-20s %s\n", $$1, $$2}' | \
		sort

doc-readme: ## Write README.md
	@./dev/readme.sh

doc-changelog: ## Write CHANGELOG.mode
	@git cliff -o CHANGELOG.md

typos: ## Check typos
	@typos

typos-fix: ## Fix typos
	@typos -w

rs-audit: ## Audit Cargo.lock
	@cargo audit

rs-audit-fix: ## Update Cargo.toml to fix vulnerable dependency requirement
	@cargo audit fix

rs-build: ## Build binary
	@cargo build --release --locked --frozen --bins

rs-cargo-deps: ## Install cargo dependencies
	@cargo install --locked cargo-outdated
	@cargo install cargo-audit --features=fix
	@cargo install cargo-udeps --locked
	@cargo install cargo-watch
	@cargo install typos-cli
	@rustup component add clippy

rs-dev: ## Run check in watch mode
	@cargo watch -c

rs-check: ## Run check
	@cargo check

rs-doc: ## Open app documentation
	@cargo doc --open

rs-fix: ## Fix rust code
	@cargo fix --allow-dirty --allow-staged --all-features --all-targets

rs-fmt: ## Format rust code
	@cargo fmt --all --check

rs-fmt-fix: ## Format fix rust code
	@cargo fmt --all

rs-install: ## Install binary
	@cargo install --path .

rs-uninstall: ## Uninstall binary
	@cargo uninstall

rs-lint: ## Lint rust code
	@cargo clippy --workspace --all-targets --all-features --no-deps -- -D warnings

rs-lint-fix: ## Fix lint rust code
	@cargo clippy --workspace --all-targets --all-features --no-deps --allow-dirty --allow-staged --fix -- -D warnings

rs-outdated: ## Display when dependencies are out of date
	@cargo outdated -wR

rs-tests: ## Run tests
	@cargo test

rs-update-cargo: ## Update dependencies
	@cargo update

rs-update-rustup:
	@rustup update

bash-all: bash-fmt bash-check bash-lint ## Run all bash tests

bash-fmt: ## Format bash code
	@find . -type f -name "*.sh" | xargs shfmt -i 2 -w

bash-check: ## Check format bash code
	@find . -type f -name "*.sh" | xargs shfmt -i 2 -d

bash-lint: ## Check lint bash code
	@find . -type f -name "*.sh" | xargs shellcheck -o all

yaml-fmt: ## Format yaml code
	@find . -type f -regex ".*.ya?ml" | xargs yamlfmt

yaml-lint: ## Check lint yaml code
	@find . -type f -regex ".*.ya?ml" | xargs yamllint

.PHONY: help
.PHONY: bash-all
.PHONY: bash-check
.PHONY: bash-fmt
.PHONY: bash-lint
.PHONY: doc-changelog
.PHONY: doc-readme
.PHONY: rs-audit
.PHONY: rs-audit-fix
.PHONY: rs-build
.PHONY: rs-cargo-deps
.PHONY: rs-check
.PHONY: rs-dev
.PHONY: rs-doc
.PHONY: rs-fix
.PHONY: rs-fmt
.PHONY: rs-fmt-fix
.PHONY: rs-install
.PHONY: rs-lint
.PHONY: rs-lint-fix
.PHONY: rs-outdated
.PHONY: rs-run
.PHONY: rs-tests
.PHONY: rs-uninstall
.PHONY: rs-update-cargo
.PHONY: rs-update-rustup
.PHONY: typos
.PHONY: typos-fix
.PHONY: yaml-fmt
.PHONY: yaml-lint
