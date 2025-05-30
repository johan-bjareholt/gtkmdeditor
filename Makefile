#CARGO_FLAGS:=
#CARGO_FLAGS:=--release

PREFIX:=/usr/local

SOFILE=target/debug/libgtkmdeditor.so
HEADER=target/debug/include/gtkmdeditor/gtkmdeditor.h
PCFILE=target/debug/gtkmdeditor.pc

all: build

.PHONY:
fetch:
	cargo $(CARGO_FLAGS) fetch --manifest-path Cargo.toml --verbose

.PHONY: build
build:
	cargo build $(CARGO_FLAGS)

.PHONY: install
install:
	install -d $(PREFIX)/lib/
	install $(SOFILE) $(PREFIX)/lib/libgtkmdeditor.so
	install -d $(PREFIX)/include/gtkmdeditor
	install $(HEADER) $(PREFIX)/include/gtkmdeditor/gtkmdeditor.h
	install -d $(PREFIX)/lib/pkgconfig/
	install $(PCFILE) $(PREFIX)/lib/pkgconfig/gtkmdeditor.pc

.PHONY: uninstall
uninstall:
	rm -f $(PREFIX)/lib/libgtkmdeditor.so
	rm -rf $(PREFIX)/include/gtkmdeditor
	rm -f $(PREFIX)/lib/pkgconfig/gtkmdeditor.pc

# C-example
CC = gcc
PKGCONFIG = pkg-config

CFLAGS = $(shell $(PKGCONFIG) --cflags gtk4) -I./target/debug/include/gtkmdeditor
LDFLAGS = $(shell $(PKGCONFIG) --libs gtk4) -L./target/debug -lgtkmdeditor

.PHONY: build-c-example
build-c-example: target/testeditor_c

target/c_example: examples/testeditor.c build
	mkdir -p target
	$(CC) -o $@ $< $(CFLAGS) $(LDFLAGS)

run-c-example: target/c_example
	LD_LIBRARY_PATH=./target/debug $<
