VERSION = $(shell awk -F ' = ' '$$1 ~ /version/ { gsub(/[\"]/, "", $$2); printf("%s",$$2) }' Cargo.toml)

build:
	docker build -t auth-login-stub-stub:$(VERSION) .

clean:
	docker rmi auth-login-stub-stub:$(VERSION)

rebuild: clean
	docker build --no-cache -t auth-login-stub-stub:$(VERSION) .

run:
	docker container run -p 4200:4200 auth-login-stub-stub:$(VERSION)

start: build run

restart: rebuild run

prune:
	docker container prune --filter "label=name=auth-login-stub-stub"
