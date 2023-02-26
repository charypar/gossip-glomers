
inspect:
	cd maelstrom && ./maelstrom serve

c1-echo: maelstrom build
	cd maelstrom && \
	./maelstrom test -w echo --bin ../target/debug/c1-echo --node-count 1 --time-limit 3

.PHONY: build
build: 
	cargo build

maelstrom: ./maelstrom/maelstrom

./maelstrom/maelstrom:
	curl -O https://github.com/jepsen-io/maelstrom/releases/download/v0.2.2/maelstrom.tar.bz2
	tar -xjf maelstrom.tar.bz2
	rm maelstrom.tar.bz2
