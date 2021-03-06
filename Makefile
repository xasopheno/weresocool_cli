play: 
	ulimit -s 32768 &&\
	export RUST_MIN_STACK=8388608 &&\
	cargo run --release -- play song.socool

watch: 
	ulimit -s 32768 &&\
	export RUST_MIN_STACK=8388608 &&\
	cargo run --release -- watch song.socool

print: 
	ulimit -s 32768 &&\
	export RUST_MIN_STACK=8388608 &&\
	cargo run --release -- print song.socool --all

dev: 
	ulimit -s 32768 &&\
	export RUST_MIN_STACK=8388608 &&\
	cargo watch --exec "run -- play song.socool"

test: 
	ulimit -s 32768 &&\
	export RUST_MIN_STACK=8388608 &&\
	cargo watch --exec "test" 
