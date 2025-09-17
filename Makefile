# Directories
ENV_FILE := .env
TARGET_DIR := target/release
INTERNALS_DIR :=commands/src/bin
INSTALL_DIR := bin

.PHONY: all clean build copy env

all: clean build copy env

clean:
	@echo "Cleaning $(TARGET_DIR)..."
	@cargo clean
	@rm -rf $(INSTALL_DIR)/* $(INSTALL_DIR)/*
	@mkdir -p $(INSTALL_DIR)

build:
	@echo "Building workspace..."
	@cargo build --release --workspace 

copy:
	@echo "Copying binaries to $(INSTALL_DIR)..."
	@for crate in $(wildcard $(INTERNALS_DIR)/*); do \
		if [ -d $$crate ]; then \
			name=$$(basename $$crate); \
			bin_path=$(TARGET_DIR)/$$name; \
			if [ -f $$bin_path ] && [ -x $$bin_path ]; then \
				echo "Installing $$name -> $(INSTALL_DIR)"; \
				cp $$bin_path $(INSTALL_DIR)/; \
			fi \
		fi \
	done

env:
	@echo "Updating $(ENV_FILE)..."
	@rm -f $(ENV_FILE)
	@touch $(ENV_FILE)
	@echo "DIR=$(shell pwd)/bin/" >> $(ENV_FILE); 
	@echo "✅ .env updated:"
	@cat $(ENV_FILE)

run: 
	@echo "Running shell..."
	@$(TARGET_DIR)/shell