SHELL := /bin/bash

ISO_DATE := $(shell date +%Y-%m)
ARCH := x86_64

CONFIG_DIR ?= configs
WORK_DIR ?= work
OUT_DIR ?= out
SCRIPTS_DIR ?= scripts

BUILD_SCRIPT := $(SCRIPTS_DIR)/build.sh
RUN_SCRIPT := $(SCRIPTS_DIR)/run_archiso.sh

# Default ISO name used by test targets. You can override VERSION on the make
# command line: `make build VERSION=2025.08` or `make test-uefi VERSION=2025.08`.
VERSION ?= $(ISO_DATE)
ISO_NAME := asenos-$(VERSION)-$(ARCH).iso

.PHONY: all help install-scripts build test-uefi test-bios test-wizard clean

all: build

help:
	@printf "Available targets:\n"
	@printf "  make build [VERSION=YYYY.MM]     Build ISO using $(BUILD_SCRIPT)\n"
	@printf "  make test-uefi [VERSION=...]     Boot generated ISO in QEMU (UEFI)\n"
	@printf "  make test-bios [VERSION=...]     Boot generated ISO in QEMU (BIOS)\n"
	@printf "  make test-wizard                 Run the setup wizard tests\n"
	@printf "  make install-scripts             Ensure scripts are executable\n"
	@printf "  make clean                       Remove $(WORK_DIR) and $(OUT_DIR)\n"

install-scripts:
	chmod +x $(SCRIPTS_DIR)/*.sh

build: install-scripts
	@echo "Running build script: $(BUILD_SCRIPT)"
	# Pass VERSION through to the build script using -v so the script can name the ISO
	$(BUILD_SCRIPT) -v $(VERSION)

test-uefi: install-scripts
	@echo "Launching QEMU (UEFI) with $(OUT_DIR)/$(ISO_NAME)"
	$(RUN_SCRIPT) -u -i $(OUT_DIR)/$(ISO_NAME)

test-bios: install-scripts
	@echo "Launching QEMU (BIOS) with $(OUT_DIR)/$(ISO_NAME)"
	$(RUN_SCRIPT) -b -i $(OUT_DIR)/$(ISO_NAME)

test-wizard:
	@echo "Running wizard tests"
	$(SCRIPTS_DIR)/test_wizard.sh

clean:
	rm -rf $(WORK_DIR) $(OUT_DIR)
