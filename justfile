set shell := ["bash", "-cu"]
set windows-shell := ["pwsh", "-Command"]

oxfmt := "pnpm exec oxfmt"

core := "literalize"

macros := "literalize_macros"

tst := "test"

# Default action
_:
    just --list -u

# Install
i:
    pnpm install

# Format code
fmt:
    cargo fmt
    {{oxfmt}}

# Lint code with ls-lint
ls-lint:
    ls-lint -config ./.ls-lint.yaml

# Lint code with ls-lint
lslint: ls-lint

# Lint code with typos
typos:
    typos

# Lint code
lint:
    cargo clippy

# Run test for doc
test-doc:
    cargo test -p {{core}} -- --nocapture
    cargo test -p {{macros}} -- --nocapture

# Run test
test:
    cargo test -p {{tst}} -- --nocapture

# Check code
check: fmt ls-lint typos lint test-doc test

# Publish package as dry-run
publish-try:
    cargo publish -p {{macros}} --dry-run
    cargo publish -p {{core}} --dry-run

# Publish package
publish:
    cargo publish -p {{macros}}
    cargo publish -p {{core}}

# Clean
clean:
    cargo clean
    pnpm clean
