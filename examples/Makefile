CC = gcc
PKGCONFIG = pkg-config

CFLAGS = $(shell $(PKGCONFIG) --cflags gtk4) -I../target/debug/include/gtkmdeditor
LDFLAGS = $(shell $(PKGCONFIG) --libs gtk4) -L../target/debug -lgtkmdeditor

c_test: c_test.c
	$(CC) -o $@ $< $(CFLAGS) $(LDFLAGS)

.PHONY: clean
clean:
	rm -f c_test
