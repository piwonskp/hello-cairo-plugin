# Hello cairo plugin

The repository demonstrates the usage of cairo plugin system. It contains the plugin which for each function annotated with `hello` attribute prints `Hello, <function-name>!`.

## Quickstart

### Prerequisites

Setup the cairo environment with the `asdf install` command.

### Usage

To use the plugin in your project annotate a function with `hello` attribute:

```
#[hello]
fn multiply(a: u8, b: u8) -> u8 {
    a * b
}
```

An example project is located in [example directory](./example/).

### Execution

1. cd into project directory:

```
cd example/
```
2. Run scarb with the hello-cairo-plugin:

```
scarb cairo-run
```

3. You should see the output:

```
     Running examples
[DEBUG] 0x48656c6c6f2c206d61696e21 ('Hello, main!')
[DEBUG] 0x48656c6c6f2c2061646421 ('Hello, add!')
[DEBUG] 0x48656c6c6f2c206d756c7469706c7921 ('Hello, multiply!')
[DEBUG] 0x48656c6c6f2c20666f6f21 ('Hello, foo!')
Run completed successfully, returning []
```

## Development

The repository shows the implementation of cairo plugin with macros. For the implementation of macros see [src/lib.rs](./src/lib.rs). The file illustrates two approaches to developing attribute macros: 
* hello macro using native cairo parser
* hello_regex macro implementing the same functionality using regex

It is recommended especially for complex macros to use the native cairo parser as regexes are often error prone and have their limitations. They are lightweight however and may be enough for some macros.

For more see [scarb's documentation on procedural macros](https://docs.swmansion.com/scarb/docs/reference/procedural-macro.html#procedural-macro-will-be-called-from-cairo-code).
