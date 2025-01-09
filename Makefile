VERSION=1.0
NAME=ray-tracing
EXEC=rust-exec
PREFIX=$(HOME)/.local

all: clean build run

default: help

build: 
	@echo "> Building files..."
	@cargo build
clean:
	@echo "> Cleaning build directory..."
	@rm -rf target/*
	@cargo clean
check:
	@echo "> Checking $(NAME)"
	@cargo check
compile:
	@echo "> Compiling program..."
	@cargo build
run:
	@echo "> Running program..."
	@clear
	@cargo run
help:
	@echo "> Usage: \"make <build> <clean> <check> <compile> <run> <all>\""
