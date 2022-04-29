FILE_PATH=

run:
	cargo run $(FILE_PATH)

debug:
	cargo run $(FILE_PATH) RUST_BACKTRACE=1
	
clean:
	rm -rf target/ Cargo.lock
