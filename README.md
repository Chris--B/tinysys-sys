# `tinysys` SDK from to Rust
`tinysys` is a hobbyist compute platform: https://github.com/ecilasun/tinysys.
> Tinysys started out as a hobby project. It now has two RISC-V cores, and several other facilities listed below, and can happily run most software with minimal tweaks.
>
> Of course, before you ask, it does run DOOM, and with sound and keyboard input! (Lately, it has been running Quake as well.)

This crate exposes the raw C bindings of the [tinysys SDK](https://github.com/ecilasun/tinysys/blob/main/software/SDK/README.md) to Rust. It uses `bindgen` to generate and **check in** the bindings. This means using the crate has no dependencies, but updating it is manual. See the section [Updating the SDK+Bindings](#updating-the-sdkbindings) below for more details.

This crate does **not** expose idiomatic Rust bindings, nor does it take opinions on how to use anything. This is the `-sys` style, straight C bindings.

## TODOs

This crate is WIP. Here's a list of TODOs I'm working on in no particular order.
- [ ] Compile the C code and link it in, instead of meerly providing symbols
    - We could check-in a static lib and make this manual, like we do the bindgen'd headers
- [ ] Idiomatic Rust traits etc
    - We should provide impls for utiltiy traits like those from `bytemuck` on the raw C types.
    - Due to the Orphan Rule, this crate **must** provide them. Unlike idiomatic wrappers, clients cannot provide these.
- [ ] Idiomatic Rust wrappers
    - It would be nice to have bindings that are more ergonomic to use

## Code Layout
The code is layed out like so:
- `src/include/sdk.h`
    - Generated header that includes the full SDK
- `src/sdk.rs`
    - Generated rust file, made by running bindgen on `include/wrapper.h`
- `src/include/wrapper.h`
    - Hand written header that includes `sdk.h`.
    - May add any additional includes or definitions that may be useful.
- `src/lib.rs`
    - Defines the crate library and re-exports symbols from the generated `sdk.rs`.
    - May add any additional utility macros, functions, traits, etc. that may be useful.
- `tinysys_c_sdk/SDK`
    - The `SDK` folder verbatim from the [`tinysys` repo](https://github.com/ecilasun/tinysys).

## Updating the SDK+Bindings
To update everything, you need to install a RISCV toolchain and run the update scripts.

### Installing riscv-tools
Install the Rust target with rustup:
```sh
# TODO: Not sure if this is the right target for tinysys, but it's what I'm using atm.
rustup target add riscv32imac-unknown-none-elf
```

#### Windows, Linux
TODO. I'm not sure how to install riscv-tools on these platforms yet. Running bindgen on them may require you to modify `./update_rs.sh` to get the options right.

#### macOS
Install `riscv-tools` using homebrew, as detailed [here](https://github.com/riscv-software-src/homebrew-riscv?tab=readme-ov-file#installation).
```sh
brew tap riscv-software-src/riscv
brew install riscv-tools
```

### Downloading the C SDK and running `bindgen`
With that installed, make sure it's findable in the path and run the update scripts:
```sh
riscv64-unknown-elf-gcc --version
./update_c_sdk.sh
./update_rs.sh
```

Running `./update_c_sdk.sh` downloads the latest SDK files from the `tinysys` repo and copies it in into `tinysys_c_sdk/SDK`.
Running `./update_rs.sh` generates `src/include/sdk.h` from the previously-downloaded SDK and runs `bindgen`. Currently it pulls riscv headers using `riscv64-unknown-elf-gcc -print-sysroot` but I'm not sure how cross-platform friendly this is.

If you're only changing the bindgen options, you do not need to rerun `./update_c_sdk.sh`.

### Building
This crate builds with a single `cargo build`. We set the default target-triple in `.cargo/config.toml` so it works for tinysys out of the box.
