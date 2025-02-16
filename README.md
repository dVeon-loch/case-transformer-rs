# Case Transformer

## Running instructions

All subsequent commands are run in the root of this repository and assume a default installation of Rust is present on the system.
### Run via cargo

```bash
cargo run --release
```

### Run binary directly

```bash
cargo build --release

./target/release/case-transformer-rs
```

## Testing

### Built-in integration testing

```bash
cargo test
```

### Testing main application binary

```bash
cargo run --release
```

In a separate terminal:

```bash
curl -X POST http://localhost:5000/api/v1/transform   -H "Content-Type: application/json"   -d '{"html":"<div><p>Hello World</p><span>Not a paragraph</span></div>", "transform":"uppercase"}'
```

Expected results:

```bash
<div><p>HELLO WORLD</p><span>Not a paragraph</span></div>
```
