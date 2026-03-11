default:
	@ just --list --unsorted

test:
	cargo nextest run --no-capture -r

test-debug:
	cargo nextest run --no-capture

build:
	cargo build -r

debug:
	cargo run -q

run:
	cargo run -qr

add *deps='':
	cd "crates/deps"
	# cargo add "$@"
	echo {{deps}}
