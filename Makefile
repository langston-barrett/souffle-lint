CARGO = cargo
LIT = lit

DL = $(shell ls ./**/*.dl)
RS = $(shell ls ./**/*.rs)
TOML = $(shell ls ./**/*.toml)

.PHONY: build
build:
	$(CARGO) build

# requires: apt-get install -y musl-tools
# requires: rustup target add x86_64-unknown-linux-musl
.PHONY: static
static:
	$(CARGO) build --release --target=x86_64-unknown-linux-musl

.PHONY: check
check:
	$(CARGO) check

# requires: cargo install cargo-deb
.PHONY: deb
deb:
	$(CARGO) deb -- --release --target=x86_64-unknown-linux-musl

.PHONY: doc
doc:
	$(MAKE) -C doc html

.PHONY: entr
entr:
	ls Makefile $(DL) $(RS) $(TOML) | \
	  entr -c -s "make -j fmt check lint && make build && make test"

.PHONY: fmt
fmt:
	$(CARGO) fmt

.PHONY: lint
lint: doc
	$(CARGO) clippy -- \
	  -D warnings \
	  -D clippy::unnecessary_wraps

.PHONY: lit
lit: build
	$(LIT) -v --path=$(PWD)/target/debug test/

.PHONY: unit
unit:
	$(CARGO) test

.PHONY: test
test: unit lit

.PHONY: all
all: build check fmt lint test
