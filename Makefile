all: compile

compile: 
	cargo run

timed:
	cargo run --bin timed

build:
	cargo build
