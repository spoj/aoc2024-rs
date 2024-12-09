build:
	cargo build

build-rel:
	cargo build -r

watch:
	cargo watch -x run

watch-rel:
	cargo watch -x 'run -r'

clean:
	cargo clean