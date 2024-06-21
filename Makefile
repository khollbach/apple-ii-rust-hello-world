# Notes on how we invoke cargo:
# (See also .config/cargo.toml and rust-toolchain.toml)
# * We ask rustc to emit LLVM IR when it builds our code.
#   * Technically, LLVM "bitcode", which is just a binary-format of IR;
#     clang runs much faster of you use bitcode instead of text-format IR.
# * We've made our project a library, so that cargo doesn't try to link our
#   code into an executable.
# * Our main function is tagged "no_mangle" and "extern C" so that clang knows
#   how to call it.
# * Hack: we use --target=msp430 so that the pointer width is 16 bits.
# * We need to build `core` ourselves, because msp430 is only partly supported.
#   * Which means we need to use nightly Rust.
# * The LLVM IR ends up in target/msp430-none-elf/{debug,release}/deps/

crate_name = apple_ii_rust_hello_world

# Note that this currently only works if we build in release mode. I don't
# understand the details, but we get linker errors when building in debug mode.
build:
	cargo build --release
	mkdir -p target/6502
	mos-common-clang \
		-o target/6502/$(crate_name) \
		target/msp430-none-elf/release/deps/$(crate_name)-*.bc \
		target/msp430-none-elf/release/deps/core-*.bc

clean:
	cargo clean
