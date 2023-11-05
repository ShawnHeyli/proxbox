:PHONY: build all

all: build

build: 
	cargo build
	scp target/debug/proxbox root@192.168.1.2:/root/proxbox
	scp -r scripts root@192.168.1.2:/root/
	scp -r aliases root@192.168.1.2:/root/