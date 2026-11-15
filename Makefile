.PHONY: up down build exec check test clippy fmt run watch

up:
	docker compose up -d

down:
	docker compose down

build:
	docker compose build

exec:
	docker compose exec app bash

check:
	docker compose exec app cargo check

test:
	docker compose exec app cargo test

clippy:
	docker compose exec app cargo clippy

fmt:
	docker compose exec app cargo fmt

run:
	docker compose exec app cargo run

watch:
	docker compose exec app cargo watch -x check -x test
