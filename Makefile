NAME = wasm-experiment
NAME_ = wasm_experiment
MODE = release
TARGET_DIR = target/$(NAME)
BUILD_DIR = target/wasm32-unknown-emscripten/$(MODE)

.PHONY: doc
doc:
	cargo doc --target wasm32-unknown-emscripten

.PHONY: check
check:
	cargo check --target wasm32-unknown-emscripten

.PHONY: cargo_build
cargo_build:
	cargo build --target wasm32-unknown-emscripten --$(MODE)
	mkdir -p $(TARGET_DIR)
	cp $(BUILD_DIR)/$(NAME).js $(TARGET_DIR)/$(NAME).js
	cp $(BUILD_DIR)/$(NAME_).wasm $(TARGET_DIR)/$(NAME_).wasm
	cp $(BUILD_DIR)/build/$(NAME)-*/out/index.html $(TARGET_DIR)/index.html
	cp assets/DejaVuSansMono-Bold.ttf $(TARGET_DIR)/font.ttf

.PHONY: tileset
tileset: cargo_build
	TARGET_DIR=$(TARGET_DIR) sh $(BUILD_DIR)/build/$(NAME)-*/out/make_tileset.sh

.PHONY: build
build: cargo_build tileset

.PHONY: run
run: build
	firefox $(TARGET_DIR)/index.html

# Run Cargo: unsafe: Doesn't rebuild tileset
.PHONY: rc
rc: cargo_build
	firefox $(TARGET_DIR)/index.html

# Run Tileset: unsafe: Doesn't rebuild code
.PHONY: rt
rt:
	TARGET_DIR=$(TARGET_DIR) sh $(BUILD_DIR)/build/$(NAME)-*/out/make_tileset.sh
	firefox $(TARGET_DIR)/index.html

.PHONY: clean
clean:
	cargo clean

.PHONY: publish
publish: build
	butler push $(TARGET_DIR)/ rope/$(NAME):wasm
