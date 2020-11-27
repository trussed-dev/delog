test: non-qemu-tests qemu-tests

non-qemu-tests:
	cargo test --lib
	cargo test --doc
	cargo test --examples --features flushers,std
	$(MAKE) -C gate-tests

qemu-tests:
	$(MAKE) -C qemu-tests test

