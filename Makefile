SHELL=/usr/bin/env zsh
prog :=guitar

debug ?=

$(info debug is $(debug))

ifdef debug
  release :=
  target :=debug
  extension :=debug
else
  release :=--release
  target :=release
  extension :=
endif

build:
	cargo build $(release)

install:
	# cp target/$(target)/$(prog) ~/.cargo/bin/$(prog)-$(extension)
	cp target/$(target)/$(prog) ~/.cargo/bin/$(prog)

encrypt:
	$(info to run encypt...)
	cryptic -i dec -o enc -e
	rm -f dec/magic.toml
.PHONE: enc

decrypt:
	$(info to run decrypt...)
	cryptic -i enc -o dec -d
.PHONE: dec

magic:
	target/$(target)/$(prog) -c m.md magic

trust:
	target/$(target)/$(prog) -c m.md trust

onlyme:
	target/$(target)/$(prog) -c m.md me


all: build install encrypt decrypt
.PHONE: all

help:
	@echo "usage: make $(prog) [debug=1]"
