SOLO_IP := 192.168.50.216

#TARGET := armv7-unknown-linux-musleabihf
TARGET := armv7-unknown-linux-gnueabihf

BIN := solo-test
DEPLOYED_PATH := /home/root/${BIN}

.PHONY: build
build:
	@echo "== building =="
	@#cargo build --release
	@cross build --release --target ${TARGET}

.PHONY: clean
clean: ## clean artifacts
	@cargo clean

.PHONY: deploy
deploy: build ## deploy binary via rsync
	@echo "== deploying =="
	@rsync -azvhP target/${TARGET}/release/${BIN} root@${SOLO_IP}:${DEPLOYED_PATH}

.PHONY: run
run: deploy
	@echo "== running on remote =="
	@ssh root@${SOLO_IP} -t ${DEPLOYED_PATH}

## Help display.
## Pulls comments from beside commands and prints a nicely formatted
## display with the commands and their usage information.

.DEFAULT_GOAL := help

help: ## Prints this help
	@grep -h -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
