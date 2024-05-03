
build:
	cargo test
	maturin build

coverage: clean
	mkdir -p coverage
	LLVM_PROFILE_FILE="coverage/default_%m_%p.profraw" RUSTFLAGS="-C instrument-coverage" cargo test
	LLVM_PROFILE_FILE="coverage/default_%m_%p.profraw" target/debug/rusted
	llvm-profdata merge -sparse coverage/default_*.profraw -o coverage/rusted.profdata
	llvm-cov report --use-color \
		--ignore-filename-regex='/.cargo/registry' \
		--ignore-filename-regex='build/BUILD' \
		--instr-profile=coverage/rusted.profdata \
		--object $$(ls -t1 target/debug/deps/rusted-* | head -n 1)

clean:
	cargo clean
	find python/ -type f -name "*.so" | xargs -I{} rm -v {}
	rm -v Cargo.lock
	rm -rfv coverage
