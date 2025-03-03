#!/usr/bin/env just --justfile

OPENAPI_GENERATOR_PROJECT_NAME := "ferric_sdk"
OPENAPI_GENERATOR := "rust"
OPENAPI_GENERATOR_PROPERTIES := "avoidBoxedModels=true,bestFitInt=true,preferUnsignedInt=true,topLevelApiClient=true,library=reqwest-trait,mockall=true,useBonBuilder=true"

DEFAULT_SPEC_FILE := "./openapi.json"

# Run the project in debug mode
run: build
    cargo run

# Run the project in release mode
run-release: release-build
    cargo run -r

# Build the project in release mode
release-build: pre-build
    cargo build --release --workspace

# Build the project in debug mode
build: pre-build
    cargo build --workspace

[private]
lint:
    cargo clippy --fix --all-targets --allow-dirty --allow-staged --workspace --all-features

[private]
fmt:
    cargo fmt --all

[private]
check:
    cargo check --workspace

[private]
pre-build: check lint fmt

# Document the project
docs: pre-build
    cargo doc --workspace --no-deps --open

# Generate client library from the OpenAPI spec
[confirm("This might override or delete some files, are you sure you want to do this? Y/n")]
generate spec_file=DEFAULT_SPEC_FILE:
    openapi-generator generate -g {{OPENAPI_GENERATOR}} --skip-validate-spec --package-name {{OPENAPI_GENERATOR_PROJECT_NAME}} -i {{spec_file}} -o ./client --additional-properties={{OPENAPI_GENERATOR_PROPERTIES}}
    just check
    cargo clippy --fix --all-targets --allow-dirty --allow-staged --workspace --all-features
    cargo fmt --all

# Generate OpenAPI spec validation report
validate-spec spec_file=DEFAULT_SPEC_FILE:
    openapi-generator validate -i {{spec_file}}

# Update dependencies
update-deps:
    cargo update -w
