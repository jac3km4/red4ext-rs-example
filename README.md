## red4ext-rs example project
This is a template RED4ext plugin written in Rust.
It contains a CI job that produces a ZIP archive with your code nicely packaged as a Cyberpunk2077 mod.

### requirements
- [RED4ext](https://github.com/WopsS/RED4ext.SDK) (for deployment in the game)
- [redscript](https://github.com/jac3km4/redscript) (for deployment in the game)
  - Only required for the native function definition.

### build
```
cargo build --release
```
This will produce a DLL file in `target/release/red4ext_rs_example.dll`.
It needs to be placed in `Cyberpunk 2077/red4ext/plugins` for RED4ext.
