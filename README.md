# Rust 'Foxes' out-of-tree module

This basic module creates a single virtual device called `/dev/foxes` that continuously outputs the Unicode fox emoji when read. 🦊

It is originally based on the [Rust for Linux out-of-tree-module template](https://github.com/Rust-for-Linux/rust-out-of-tree-module/) and [JackOS' Rust for Kernel development tutorial](https://www.jackos.io/rust-kernel/rust-for-linux.html).

## Adding the Necessary Abstractions

Because the `rust` branch is no longer maintained on RFL's tree, the abstractions this driver relied on are not yet present.

In order to add them, apply [`missing-abstractions.patch`](./missing-abstractions.patch) over RFL's `rust-next`.

It is very likely that the patch will not apply, especially as further abstractions are added.

## Build Instructions After Update

All warnings and considerations from the Rust for Linux foudation's repository apply here. For the sake of efficacy, here are the instructions to build this module on a kernel with Rust support:

```sh
$ make KDIR=.../linux-with-rust-support LLVM=1
make -C .../linux-with-rust-support M=$PWD
make[1]: Entering directory '.../linux-with-rust-support'
  RUSTC [M] .../rust-foxes-module/rust_foxes.o
  MODPOST .../rust-foxes-module/Module.symvers
  CC [M]  .../rust-foxes-module/rust_foxes.mod.o
  LD [M]  .../rust-foxes-module/rust_foxes.ko
make[1]: Leaving directory '.../linux-with-rust-support'
```

## Changelog

- 2024-09-04
  - Update abstraction patch for split rust helpers
  - Update abstraction patch to use `kernel::uaccess` in place of legacy `kernel::user_ptr`
  - Update `kernel::file` abstractions consequently
  - Up-to-date with `rust-next`
  - Fixed a bug where having a block size below 4 incorrectly decreased the remaining fox count even if a fox could not be entirely written
  - Add a global count of foxes, the module has to be unloaded and reloaded for it to be reset
  - Turn the diff file into a proper patch of the abstraction commit
