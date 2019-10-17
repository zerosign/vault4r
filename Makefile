REGISTRY          ?= docker.pkg.github.com
ORG               ?= zerosign
PROJECT           ?= vaultr
CURRENT_DIR	      := $(shell pwd)
GO_EXISTS         := $(shell command -v go 2> /dev/null)
SHELLCHECK_EXISTS := $(shell command -v shellcheck 2> /dev/null)
HADOLINT_EXISTS   := $(shell command -v hadolint 2> /dev/null)
VERSION           ?= $(strip $(shell git show -q --format=%h))

.PHONY: clean build test doc clean-compose tools build-compose

all: compile test build doc

clean:
	cargo clean

clippy:
	cargo clippy-verbose

build-compose:
	docker-compose up --no-start

clean-compose:
	docker-compose down -v
