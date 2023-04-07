# TQ Code Generator Core

TQ Code Generator Core is a Rust library for generating random codes of varying lengths. It was created to power the code generation tools at ToolQuarry, but it can be used in any Rust project that needs to generate unique codes.

## Installation

Add tq-code-generator-core to your project's Cargo.toml file:

```
[dependencies]
tq-code-generator-core = "0.1.0"
```

## Usage

Here's an example of how to use tq-code-generator-core to generate a random code:

```
use tq_code_generator_core::generate_random_code;

fn main() {
    let code = generate_random_code(8);
    println!("{}", code);
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
