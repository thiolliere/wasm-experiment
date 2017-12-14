NAME = wasm_experiment
MODE = release
TARGET_DIR = target/$(NAME)
BUILD_DIR = target/wasm32-unknown-emscripten/$(MODE)

$(TARGET_DIR):
	mkdir -p $@

$(BUILD_DIR)/$(NAME).js: cargo_build

$(BUILD_DIR)/$(NAME).wasm: cargo_build

$(BUILD_DIR)/build/$(NAME)-%/out/index.html: cargo_build

$(BUILD_DIR)/build/$(NAME)-%/out/make_tileset.sh: cargo_build

$(TARGET_DIR)/$(NAME).js: $(BUILD_DIR)/$(NAME).js $(TARGET_DIR)
	cp $< $@

$(TARGET_DIR)/$(NAME).wasm: $(BUILD_DIR)/$(NAME).wasm $(TARGET_DIR)
	cp $< $@

$(TARGET_DIR)/index.html: $(BUILD_DIR)/build/$(NAME)-*/out/index.html $(TARGET_DIR)
	cp $< $@

# This command fails if out/make_tileset.sh doesn't exist but
# if we rely on this file then it rebuilds each time...
$(TARGET_DIR)/tileset0.png: assets/animations/*.png $(TARGET_DIR)
	TARGET_DIR=$(TARGET_DIR) sh $(BUILD_DIR)/build/$(NAME)-*/out/make_tileset.sh

.PHONY: check
check:
	cargo check --target wasm32-unknown-emscripten

.PHONY: cargo_build
cargo_build:
	cargo build --target wasm32-unknown-emscripten --$(MODE)

.PHONY: build
build: $(TARGET_DIR)/$(NAME).js \
       $(TARGET_DIR)/$(NAME).wasm \
       $(TARGET_DIR)/index.html \
       $(TARGET_DIR)/tileset0.png

.PHONY: run
run: build
	firefox $(TARGET_DIR)/index.html

# This command is unsafe be allow to rebuild tileset quickly
.PHONY: tileset
tileset:
	TARGET_DIR=$(TARGET_DIR) sh $(BUILD_DIR)/build/$(NAME)-*/out/make_tileset.sh
