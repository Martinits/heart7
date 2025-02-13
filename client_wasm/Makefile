MAKEFILE_ABS_PATH := $(realpath $(lastword $(MAKEFILE_LIST)))
MAKEFILE_DIR := $(dir $(MAKEFILE_ABS_PATH))
RULE_DIR := $(dir $(MAKEFILE_DIR))../rule

build: rule_cargo_toml
	cargo build --release

rule_cargo_toml:
	rm -f $(RULE_DIR)/Cargo.toml
	cp $(RULE_DIR)/Cargo.toml.wasm32 $(RULE_DIR)/Cargo.toml

serve: rule_cargo_toml
	trunk serve --disable-csp true -a 0.0.0.0 -p 10007 --release

serve_debug: rule_cargo_toml
	trunk serve --disable-csp true -a 0.0.0.0 -p 10007
