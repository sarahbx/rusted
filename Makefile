
build:
	cargo build
	maturin build


clean:
	rm -rfv target
	find python/ -type f -name "*.so" | xargs -I{} rm -v {}
	rm -v Cargo.lock
