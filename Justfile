set shell := ["bash", "-c"]

default:
	@ just --list --unsorted

test:
	cargo nextest run --no-capture --release --quiet

test-debug:
	cargo nextest run --no-capture --quiet

build:
	@ cargo build --release

debug:
	@ cargo run --quiet

run:
	@ cargo run --release --quiet

# Build and move all dependencies to the "out" dir.
release:
	# Not implemented.

# Usage: just add colorchoice to args
add dep _preposition='' crate='':
	#!/bin/env bash
	set -e
	# Shut up the "unused" warning.
	echo "{{_preposition}}" > /dev/null
	echo "Adding dependency {{dep}} globally..."
	fatal() {
		echo "$1"
		exit 1
	}
	# --limit=1 but still does the whole search...
	cargo search {{dep}} --limit=1 --quiet --color=never \
		| grep -E --only-matching '^{{dep}} = ".+?"' --color=never \
		>> "Cargo.toml" \
		|| fatal "Dependency not found."
	echo "Done."
	test -z "{{crate}}" && exit 0
	test -d "crates/{{crate}}" \
		|| fatal "There's no \"{{crate}}\" crate."
	echo "{{dep}} = { workspace = true }" \
		>> "crates/{{crate}}/Cargo.toml"
	echo "Also added opt-in to the {{crate}} crate's manifest."

alias dbg := debug
