CARGO = cargo
LIT = lit

CARGO_FLAGS = 

DL = $(shell ls ./**/*.dl)
RS = $(shell ls ./**/*.rs)
TOML = $(shell ls ./**/*.toml)

.PHONY: build
build:
	$(CARGO) $(CARGO_FLAGS) build

# requires: apt-get install -y musl-tools
# requires: rustup target add x86_64-unknown-linux-musl
.PHONY: static
static:
	$(CARGO) build $(CARGO_FLAGS) --release --target=x86_64-unknown-linux-musl

.PHONY: check
check:
	$(CARGO) check $(CARGO_FLAGS) 

# requires: cargo install cargo-deb
#
# NOTE: Needs static because static causes the build script to be run, which
# generates the embedded manual.
.PHONY: deb
deb: static
	$(CARGO) deb $(CARGO_FLAGS) -- --release --target=x86_64-unknown-linux-musl

.PHONY: doc
doc:
	$(MAKE) -C doc html

.PHONY: entr
entr:
	ls Makefile $(DL) $(RS) $(TOML) | \
	  entr -c -s "make -j fmt check lint && make build && make test"

.PHONY: fmt
fmt:
	$(CARGO) fmt $(CARGO_FLAGS)

.PHONY: lint
lint:
	$(CARGO) clippy $(CARGO_FLAGS) -- \
	  -D warnings \
	  -D clippy::unnecessary_wraps

.PHONY: lit
lit: build
	$(LIT) -v --path=$(PWD)/target/debug test/

.PHONY: unit
unit:
	$(CARGO) test $(CARGO_FLAGS)

.PHONY: test
test: unit lit

.PHONY: all
all: build check fmt lint test
