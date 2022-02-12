## red4ext-rs example project
This is a template RED4ext plugin written in Rust.
It contains a CI job that produces a ZIP archive with your code nicely packaged as a Cyberpunk2077 mod.

### requirements
- LLVM (for building)
  - On recent versions of Windows you can set it up with:
    ```powershell
    winget install llvm
    # you may need to add it to the path
    $env:PATH += ";C:\Program Files\LLVM\bin"
    ```
- RED4ext (for deployment in the game)
  - This project currently targets a pre-release version of RED4ext (download [here](https://github.com/WopsS/RED4ext.SDK/suites/5274387056/artifacts/163105991)).
- redscript (for deployment in the game)
  - Only required for the native function definition.

### build
```
cargo build --release
```
This will produce a DLL file in `target/release/red4ext_rs_example.dll`.
It needs to be placed in `Cyberpunk 2077/red4ext/plugins` for RED4ext.
