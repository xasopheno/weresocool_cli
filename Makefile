play: 
	ulimit -s 32768 &&\
	export RUST_MIN_STACK=8388608 &&\
	cargo run --release -- play song.socool

dev: 
	ulimit -s 32768 &&\
	export RUST_MIN_STACK=8388608 &&\
	cargo watch --exec "run -- play song.socool"

test: 
	ulimit -s 32768 &&\
	export RUST_MIN_STACK=8388608 &&\
	cargo watch --exec "test"
