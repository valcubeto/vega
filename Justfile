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

add dep:
	@ # Seems like each line gets it's own sub-shell...
	@ echo "cargo add {{dep}}"
	@ \
		cd "crates/deps"; \
		cargo add {{dep}} > "/dev/null" && \
		DEP="{{dep}}" && \
		echo "pub use ${DEP//-/_};" >> "src/lib.rs"
	@ echo "Added dependency {{dep}} globally."

alias dbg := debug
