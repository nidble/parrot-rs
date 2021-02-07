CARGO = cargo

.PHONY: all build check clean run test integration docker-build docker-run e2e

all: build

build:
	@$(CARGO) build --release

check:
	@$(CARGO) check

clean:
	@$(CARGO) clean

run: build
	@$(CARGO) run

test: build
	@$(CARGO) test

integration: build
	@$(CARGO) test --test integration

docker-build:
	docker build -t nidble/parrot-rs:$(version) .

docker-run:
	docker run --rm -it -p 3000:3000 nidble/parrot-rs:$(version) 

e2e:
	@curl -s http://127.0.0.1:3030/pokemon/42; 
	@echo "";
	@curl -s http://127.0.0.1:3030/pokemon/stench;
	@echo "";