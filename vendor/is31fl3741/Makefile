build-all:
	cargo build --target=thumbv6m-none-eabi --all-features
	cd examples/adafruit_rgb && \
	  cargo build --target=thumbv6m-none-eabi --examples
	cd examples/ledmatrix && \
	  cargo build --target=thumbv6m-none-eabi --examples
