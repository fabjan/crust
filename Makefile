REPO_ROOT := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

LIBSUFFIX=dylib
CC=g++

target/crust: main.cpp libcrust/target/release/libcrust.$(LIBSUFFIX)
	mkdir -p target
	$(CC) main.cpp -L libcrust/target/release -lcrust -o target/crust

libcrust/target/release/libcrust.$(LIBSUFFIX): libcrust/src/lib.rs
	cd libcrust && cargo build --release
