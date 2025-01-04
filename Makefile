build:
	$(MAKE) -C user build && $(MAKE) -C os

run: build
	$(MAKE) -C os run