PROG    = remuir
ROOT    = $(shell pwd)
BUILD   = ./target/debug
RELEASE = ./target/release
BIN     = $(abspath $(BUILD)/$(PROG))
ARGS   ?=

# Compilation Flags
CFLAGS   = -std=gnu11 \
		   -O2 -MMD -Wall -Werror \
		   -fno-asynchronous-unwind-tables -fno-builtin -fno-stack-protector \
		   -Wno-main -U_FORTIFY_SOURCE
CXXFLAGS = $(CFLAGS) -ffreestanding -fno-rtti -fno-exceptions
ASFLAGS  = -MMD
LDFLAGS  = -z noexecstack

# Tags
all: default

# Default run mode: no image loaded
default:
	@cargo run $(ARGS)

# Default binary is dummy.bin
# help: make run NAME=[C file in input/tests/]
run: image
	@cargo run $(IMAGE).bin $(ARGS)

# build production version
build:
	@cargo build --release

prod: image
	@cargo run --release $(IMAGE).bin $(ARGS)

lint:
	@cargo check

format:
	@cargo fmt

clean:
	-rm -rf $(IPT_BUILD)
	@cargo clean

count:
	@echo [input]
	@find ./input -name "*.[chS]" -or -name "*.ld" -type f | xargs wc -l
	@echo
	@echo [rust]
	@find ./src -name "*.rs" -type f | xargs wc -l

-include input/scripts/am.mk

.PHONY: all default run build release lint format clean count image
