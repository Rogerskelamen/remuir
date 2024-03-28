PROG    = remuir
BUILD   = ./target/debug
RELEASE = ./target/release
D_BIN   = $(abspath $(BUILD)/$(PROG))
ARGS   ?=

all: run

run: build
	 $(D_BIN) $(ARGS)

build:
	@cargo build

release:
	@cargo build --release

lint:
	@cargo check

format:
	@cargo fmt

clean:
	@cargo clean

.PHONY: all run build release lint format clean
