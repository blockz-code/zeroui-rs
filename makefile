IGNORE = -i temp/ -i target/ -i target_dist/

.PHONY: main
main:
	cargo watch $(IGNORE) --clear -x 'run --example main'