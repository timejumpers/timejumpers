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
