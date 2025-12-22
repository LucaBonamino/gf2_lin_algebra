PROJECT_NAME = gf2_lin_algebra

PYTHON = python

M = $(shell printf "\033[34;1m▶\033[0m")

.DEFAULT_GOAL := help

.PHONY: help
help:
	@echo "Available targets:"
	@echo "$(M) help          - Display this help message"
	@echo "$(M) tests          - Run unitests"


.PHONY: tests
tests:
	@$(info $(M) testing package...)
	pip install -e . > /dev/null && pip install pytest > /dev/null
	python -m pytest tests