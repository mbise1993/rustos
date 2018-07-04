# rustos
Simple OS kernel written in Rust following this [tutorial](https://os.phil-opp.com/).

### Currently implemented:
* VGA read/write
* Serial IO
* Basic IDT setup with breakpoint handler and double-fault handler
* GDT setup
* Integration and unit tests

### Building
1. `rustup override add nightly` - we have to use nightly Rust for certain feature gates
2. `cargo install cargo-xbuild`
3. `rustup component add rust-src`
4. `cargo install bootimage --version 0.4.0`
5. `bootimage build`

### Running (Note: Make sure QEMU x86_64 is installed and in the system path)
* To run in QEMU: `bootimage run`
* To run unit tests: `cargo test`
* To run integration tests: `bootimage test`
