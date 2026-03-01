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

name := hello
start_addr := 6000  # This has to match the linker script
prodos_start_addr := 2000  # Always $2000
out := target/6502
downloads := ~/Downloads/apple-ii-programs/

default: dsk wav

wav: build
	c2t -bc8 $(out)/$(name),$(start_addr) $(out)/$(name).wav
	if [ -d $(downloads) ]; then cp $(out)/$(name).wav $(downloads); fi

dsk: build
	cp "build-deps/ProDOS 8.dsk" $(out)/$(name).dsk
	for f in launcher sysutil fastcopy basic; do ac -d $(out)/$(name).dsk $$f.system; done
	add-prodos-startup-code --start-addr $(start_addr) < $(out)/$(name) > $(out)/$(name).prodos
	ac -p $(out)/$(name).dsk $(name).system sys 0x$(prodos_start_addr) < $(out)/$(name).prodos
	if [ -d $(downloads) ]; then cp $(out)/$(name).dsk $(downloads); fi

# Note that this currently only works if we build in release mode. I don't
# understand the details, but we get linker errors when building in debug mode.
build:
	cargo build --release
	mkdir -p $(out)
	mos-common-clang \
		-o $(out)/$(name) \
		$$(ls target/msp430-none-elf/release/deps/*.bc | grep -v compiler_builtins-)

clean:
	cargo clean
