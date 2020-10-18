prefix ?= /usr/local
bindir = $(prefix)/bin
SYS := $(shell $(CC) -dumpmachine)

build:
	cargo build --release
install: build
ifneq (, $(findstring darwin, $(SYS)))
	test ! -d $(bindir) && mkdir -p $(bindir)

	install "target/release/rext" "$(bindir)/rext"
else
	install -D "target/release/rext" "$(bindir)/rext"
endif
uninstall:
	rm -rf "$(bindir)/rext"
clean:
	rm -rf target
.PHONY: build install uninstall clean