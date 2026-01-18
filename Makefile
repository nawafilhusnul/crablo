.PHONY: build run release clean test check fmt lint tag publish help

# Default target
help:
	@echo "Crablo - Makefile Commands"
	@echo ""
	@echo "Development:"
	@echo "  make build     - Build debug version"
	@echo "  make run       - Run the game (debug)"
	@echo "  make release   - Build release version"
	@echo "  make clean     - Clean build artifacts"
	@echo ""
	@echo "Code Quality:"
	@echo "  make check     - Check code without building"
	@echo "  make test      - Run tests"
	@echo "  make fmt       - Format code"
	@echo "  make lint      - Run clippy linter"
	@echo ""
	@echo "Release:"
	@echo "  make tag V=x.y.z       - Create and push a new version tag"
	@echo "  make publish V=x.y.z   - Create tag and trigger GitHub release"
	@echo "  make delete-tag V=x.y.z - Delete a tag locally and remotely"

# Build debug version
build:
	cargo build -p crablo

# Run the game
run:
	cargo run -p crablo

# Build release version
release:
	cargo build --release -p crablo
	@echo "Release binary: target/release/crablo"

# Clean build artifacts
clean:
	cargo clean

# Check code without building
check:
	cargo check -p crablo

# Run tests
test:
	cargo test -p crablo

# Format code
fmt:
	cargo fmt --all

# Run clippy linter
lint:
	cargo clippy -p crablo -- -D warnings

# Create and push a version tag
# Usage: make tag V=0.1.0
tag:
ifndef V
	$(error Version not specified. Usage: make tag V=x.y.z)
endif
	git tag v$(V)
	git push origin v$(V)
	@echo "Created and pushed tag v$(V)"

# Delete a tag locally and remotely
# Usage: make delete-tag V=0.1.0
delete-tag:
ifndef V
	$(error Version not specified. Usage: make delete-tag V=x.y.z)
endif
	git tag -d v$(V) || true
	git push origin :refs/tags/v$(V) || true
	@echo "Deleted tag v$(V)"

# Create tag and trigger GitHub release
# Usage: make publish V=0.1.0
publish:
ifndef V
	$(error Version not specified. Usage: make publish V=x.y.z)
endif
	@echo "Publishing version $(V)..."
	git add -A
	git commit -m "Release v$(V)" || true
	git push origin main
	$(MAKE) delete-tag V=$(V)
	$(MAKE) tag V=$(V)
	@echo "Release v$(V) triggered! Check: https://github.com/nawafilhusnul/crablo/actions"
