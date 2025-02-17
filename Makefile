#CARGO_FLAGS=--release
PREFIX=/usr/local

SOFILE=target/debug/libgtkmdeditor.so
HEADER=target/debug/include/gtkmdeditor/gtkmdeditor.h
PCFILE=target/debug/gtkmdeditor.pc

all: build

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
