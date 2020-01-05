COLOR ?= auto # Valid COLOR options: {always, auto, never}
CARGO = cargo --color $(COLOR) --verbose

.PHONY: all bench build check clean doc install publish run test update

all: release

prepare:
	./check-libudev.sh
	./check-opencv.sh

bench: prepare
	@$(CARGO) bench

build: prepare
	@$(CARGO) build

release: prepare
	@$(CARGO) build --release
	mkdir -p build
	cp target/release/mcsapp-motion-tracker build/
	strip build/mcsapp-motion-tracker

check: prepare
	@$(CARGO) check

clean:
	@$(CARGO) clean
	rm -rf build
	rm -rf target

doc:
	@$(CARGO) doc

fmt:
	@$(CARGO) fmt

install: build
	@$(CARGO) install

lint: prepare
	@$(CARGO) clippy --all-targets --all-features -- -D warnings

publish: prepare
	@$(CARGO) publish

run: build
	@$(CARGO) run

test: build
	@$(CARGO) test

update:
	@$(CARGO) update
	@$(CARGO) upgrade --all
