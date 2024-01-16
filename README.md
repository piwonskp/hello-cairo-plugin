# Hello cairo plugin
The repository demonstrates the usage of cairo plugin system. It contains the plugin which for each function annotated with `hello` attribute prints `Hello, <function-name>!`.

## Quickstart
### Usage
To use the plugin in your project annotate a function with `hello` attribute:
```
#[hello]
fn multiply(a: u8, b: u8) -> u8 {
    a * b
}
```

An example project is located in [examples directory](./examples/cairo/).
### Execution
1. cd into cairo project directory:
```
cd examples/cairo/
```
2. Run the plugin to compile project:
```
cargo run build .
```
3. Run compiled code:
```
scarb cairo-run --no-build
```
4. You should see the output:
```
     Running examples
[DEBUG] 0x48656c6c6f2c206d61696e21 ('Hello, main!')
[DEBUG] 0x48656c6c6f2c2061646421 ('Hello, add!')
[DEBUG] 0x48656c6c6f2c206d756c7469706c7921 ('Hello, multiply!')
[DEBUG] 0x48656c6c6f2c20666f6f21 ('Hello, foo!')
Run completed successfully, returning []
```
## Project structure
The package consists of two parts: business logic and boilerplate code. Boilerplate code is stored in `src/main.rs` and `src/boilerplate` directory. Business logic is stored in `src/plugin`. 
Plugin directory contains three files: 
* `config.rs` contains base properties of the plugin - its name, version and repository URL. 
* `macro_plugin.rs` contains the plugin configuration. This is the place where you map item type to the function that processes it.
* `insert_hello.rs` contains the function that inserts hello message.