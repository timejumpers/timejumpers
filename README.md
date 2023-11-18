# Timejumpers

A multiplayer game with Bevy.  
Jump between the timelines to save the world!

## Required Tools

- [Rust](https://rustup.rs)
- [Cargo Bundle](https://github.com/burtonageo/cargo-bundle) \[for Bundling releases\]
- A C compiler
- Git

## Contributing

### Getting set up

Download the rustup toolchain from above and use install it. You'll probably
need some version of Microsoft Visual Studio on windows as some dependencies
down the line require a C compiler. 

Once you've got everything, clone the repo into your desired folder and run
`cargo build` to ensure everything is working. It should take a while, as the
entire Bevy game engine has to build. It won't take this long each time. 

### Toolchain

Timejumpers uses the nightly rust toolchain to speed up build times. Cargo
should automatically handle this for you. If it doesn't, run `rustup toolchain
install nightly` to install the nightly toolchain. You may need to add change
all future cargo commands to `cargo +nightly`.

### Building

- For most development, `cargo run` will be sufficient
- Use `cargo build --release --no-default-features` to generate release builds
    - When cross-compiling, add `--target=<target>` to compile for a different
        operating system
        - You can find the list of targets by doing `rustup target list`
        - You may need to download the target by doing `rustup target add <target>`


### Procedures

- Don't push directly to `main`. File a pull request instead.
    - Note: this can't be enforced because I don't want to pay for a GitHub
    Teams account

### Before filing a pull request

- Add changelog entry in `Unreleased`
- Project should build with no warnings or errors
    - Try to avoid `#[allow(...)] ` where possible
- Project tests should all pass `cargo test`
- Project should be formatted with rustfmt

## Useful links

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Bevy Book](https://bevyengine.org/learn/book/introduction/)
- [Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/)
