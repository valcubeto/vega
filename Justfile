set shell := ["bash", "-c"]

default:
	@ just --list --unsorted

tree:
	#!/bin/env bash
	cargo tree --package "vega*" --prefix=depth --format=" {p}" \
		| awk '{
			depth = $1
			$1 = ""
			for (i = 0; i < depth; i++) {
				printf "    "
			}
			sub(/^ /, "")
			print
		}'

test:
	cargo nextest run --no-capture --release

test-debug:
	cargo nextest run --no-capture

build:
	@ cargo build --release

vega-debug *args='':
	#!/bin/env bash
	# Possible errors and warnings are
	# printed again by `cargo run`.
	cargo build --quiet > "/dev/null"
	VEGA_HOME="./out" PATH="target/debug:$PATH" cargo run --bin=vega --quiet -- {{args}}

alias vega-dbg := vega-debug

# This command just keeps growing...
vega *args='':
	#!/bin/env bash
	cargo build --release --quiet > "/dev/null"
	VEGA_HOME="$PWD/out" PATH="target/release:$PATH" cargo run --bin=vega --release --quiet -- {{args}}

# Build and move all dependencies to the "out/bin" dir.
release:
	# Not implemented.

# Usage: just add colorchoice to args
add dep _preposition='' crate='':
	#!/bin/env bash
	set -e
	fatal() {
		echo "$1"
		exit 1
	}
	# Shut up the "unused" warning.
	echo "{{_preposition}}" > /dev/null
	echo "Adding dependency {{dep}} globally..."
	# --limit=1 but still does the whole search...
	cargo search {{dep}} --limit=1 --quiet --color=never \
		| grep -E --only-matching '^{{dep}} = ".+?"' --color=never \
		>> "Cargo.toml" \
		|| fatal "Dependency not found."
	echo "Done."
	# If no crate specified, just exit
	test -z "{{crate}}" && exit 0
	test -d "crates/{{crate}}" \
		|| fatal "There's no \"{{crate}}\" crate."
	echo "{{dep}} = { workspace = true }" \
		>> "crates/{{crate}}/Cargo.toml"
	echo "Also added opt-in to the {{crate}} crate's manifest."

new crate:
	#!/bin/env bash
	set -e
	fatal() {
		echo "$1"
		exit 1
	}
	cd crates
	test -d {{crate}} \
		&& fatal "The {{crate}} crate already exists."
	cargo new {{crate}} --lib
	# Clean the file.
	echo "" > "{{crate}}/src/lib.rs"
	cd ..
	echo '{{crate}} = { path = "crates/{{crate}}" }' \
		>> "Cargo.toml"

new-command name:
	#!/bin/env bash
	cd commands
	cargo new {{name}} --bin

alias new-cmd := new-command
