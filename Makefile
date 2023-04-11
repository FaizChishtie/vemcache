.PHONY: build run test clean docker-build docker-run rel-docker-build rel-docker-push

VERSION := $(shell grep '^version\s*=\s*"' Cargo.toml | head -n 1 | cut -d'"' -f2)

# Local
build:
	cargo build --release

run: build
	./target/release/vemcache

test:
	cargo test

clean:
	cargo clean

# Docker
docker-build:
	docker build -t faizchishtie/vemcache:latest .

docker-run: docker-build
	docker run --rm -it -p 7070:7070 faizchishtie/vemcache:latest

# Release
rel-docker-build:
	docker build -t faizchishtie/vemcache:$(VERSION) .

rel-docker-push: docker-build
	docker push faizchishtie/vemcache:$(VERSION)

# Misc
help:
	@echo "Makefile commands:"
	@echo "  help             Display this help message"
	@echo "  build            Build the project in release mode"
	@echo "  run              Run the project (builds if necessary)"
	@echo "  test             Run tests"
	@echo "  clean            Clean build artifacts"
	@echo "  docker-build     Build Docker image with the 'latest' tag"
	@echo "  docker-run       Run Docker image with the 'latest' tag (builds if necessary)"
	@echo "  rel-docker-build Build Docker image with the version tag from Cargo.toml -> $(VERSION)"
	@echo "  rel-docker-push  Push Docker image with the version tag to Docker repository"
