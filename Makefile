CRATE := vscodeadapter
TARGET := wasm32-unknown-unknown
PROFILE := release
WASM_INPUT := target/$(TARGET)/$(PROFILE)/$(CRATE).wasm
OUT_DIR := dist/wasm
BINDGEN_TARGET := web

.PHONY: wasm
wasm:
	rustup target add $(TARGET)
	cargo build -p $(CRATE) --$(PROFILE) --target $(TARGET)
	mkdir -p $(OUT_DIR)
	wasm-bindgen $(WASM_INPUT) \
		--out-dir $(OUT_DIR) \
		--target $(BINDGEN_TARGET)