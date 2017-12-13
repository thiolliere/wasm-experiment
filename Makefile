NAME = wasm_experiment
MODE = release
TARGET_DIR = target/$(NAME)
BUILD_DIR = target/wasm32-unknown-emscripten/$(MODE)

$(TARGET_DIR):
	mkdir -p $@

$(BUILD_DIR)/$(NAME).js: cargo_build

$(BUILD_DIR)/$(NAME).wasm: cargo_build

$(TARGET_DIR)/$(NAME).js: $(BUILD_DIR)/$(NAME).js $(TARGET_DIR)
	cp $< $@

$(TARGET_DIR)/$(NAME).wasm: $(BUILD_DIR)/$(NAME).wasm $(TARGET_DIR)
	cp $< $@

$(TARGET_DIR)/index.html: index.html $(TARGET_DIR)
	cp $< $@

$(TARGET_DIR)/tileset.jpg: tileset.jpg $(TARGET_DIR)
	cp $< $@

.PHONY: check
check:
	cargo check --target wasm32-unknown-emscripten

.PHONY: cargo_build
cargo_build:
	cargo build --target wasm32-unknown-emscripten --$(MODE)

.PHONY: build
build: $(TARGET_DIR)/index.html \
       $(TARGET_DIR)/$(NAME).js \
       $(TARGET_DIR)/$(NAME).wasm \
       $(TARGET_DIR)/tileset.jpg

.PHONY: run
run: build
	firefox $(TARGET_DIR)/index.html
