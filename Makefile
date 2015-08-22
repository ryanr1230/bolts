CARGO = cargo

CARGO_OPTS =
CARGO_BUILD_OPTS = --release

build:
	$(CARGO) $(CARGO_OPTS) build $(CARGO_BUILD_OPTS)

install:
	$(MAKE) build
	mkdir -p ~/.bolts/
	cp -r ./default_config/ ~/.bolts
	chmod -R a+w+r ~/.bolts
	cp ./target/release/bolts /usr/bin/bolts

.PHONY: build install
