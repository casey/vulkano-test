default: check

log='debug'

bt='0'

check:
	cargo check

build:
	cargo build

run: build
	RUST_BACKTRACE={{bt}} RUST_LOG={{log}} ./target/debug/x

vulkano-docs:
	open https://docs.rs/vulkano/0.9.0/vulkano/

winit-docs:
	open https://docs.rs/winit/0.11.3/winit/
