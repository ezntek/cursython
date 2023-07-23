build:
	cargo build --release

install: build
ifneq ($(EUID),0)
	@echo "you must run make install as root."
	exit
else
	cp -f target/release/cursython /usr/bin/cursython
endif

userinstall:
	cargo install --path .

uninstall:
ifneq ($(EUID),0)
	@echo "you must run make uninstall as root."
else
	rm -f /usr/bin/cursython
endif

clean:
	rm -f -r target
