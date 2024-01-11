# Hello cairo plugin
The repository demonstrates the usage of cairo plugin system. It contains the plugin which for each function annotated with `hello` attribute prints `Hello, <function-name>!`.

## Usage
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