# Case Transformer

Simple Rust webserver that exposes two REST-API endpoints: 
- GET `/alive` for simple checking if the server is up
- POST `/transform` which ingests a JSON payload such as:

```json
{
    "html":"<div><p>Hello World</p><span>Not a paragraph</span></div>", 
    "transform":"uppercase"
}
```

and returns the input HTML string transformed according to the specified transform method:

```html
<div><p>HELLO WORLD</p><span>Not a paragraph</span></div>
```

## Running instructions

All subsequent commands are run in the root of this repository and assume a default installation of Rust is present on the system.
### Run via cargo

```bash
cargo run --release -- --local
```

### Run binary directly

```bash
cargo build --release

./target/release/case-transformer-rs --local
```

## Testing

### Built-in integration testing

```bash
cargo test
```

### Testing main application binary

1. Run according to the instructions [above](#running-instructions)

2. In a separate terminal:

```bash
curl -X POST http://localhost:5000/api/v1/transform   -H "Content-Type: application/json"   -d '{"html":"<div><p>Hello World</p><span>Not a paragraph</span></div>", "transform":"uppercase"}'
```

Expected results:

```html
<div><p>HELLO WORLD</p><span>Not a paragraph</span></div>
```
