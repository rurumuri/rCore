build:
	$(MAKE) -C user build && $(MAKE) -C os

run: build
	$(MAKE) -C os run

clean:
	$(MAKE) -C user clean && $(MAKE) -C os clean

fmt:
	$(MAKE) -C os fmt

fmt-fix:
	$(MAKE) -C os fmt-fix