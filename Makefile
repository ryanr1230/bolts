CARGO = cargo

CARGO_OPTS =
CARGO_BUILD_OPTS = --release

all:
	$(MAKE) build
	$(MAKE) doc

build:
	$(CARGO) $(CARGO_OPTS) build $(CARGO_BUILD_OPTS)

clean:
	$(CARGO) $(CARGO_OPTS) clean

check:
	$(MAKE) build
	$(MAKE) test

test:
	$(CARGO) $(CARGO_OPTS) test

bench:
	$(CARGO) $(CARGO_OPTS) bench

doc:
	$(CARGO) $(CARGO_OPTS) doc

install:
	$(MAKE) build
	cp ./target/release/bolts /usr/bin/bolts

.PHONY: all build clean check test bench doc install
