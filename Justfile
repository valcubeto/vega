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

