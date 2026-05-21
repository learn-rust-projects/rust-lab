.PHONY: install

## install: Sync templates and build & install CLI
install:
	@mkdir -p templates/.vscode templates/.github/workflows
	@cp template/.vscode/settings.json template/.vscode/tasks.json templates/.vscode/
	@cp template/rustfmt.toml template/.gitattributes template/.gitignore templates/
	@cp template/.github/workflows/*.yml template/.github/workflows/*.yaml templates/.github/workflows/ 2>/dev/null || true
	@echo "Generating templates.rs..."
	@python3 scripts/generate_templates.py
	@echo "Building..."
	@cargo build --release -p mvp
	@echo "Installing..."
	@mkdir -p ~/.cargo/bin
	@cp ../target/release/mvp ~/.cargo/bin/
	@chmod +x ~/.cargo/bin/mvp
	@echo "Done!"
