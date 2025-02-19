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

For general usage, navigate to the page hosted below:

[HTML Text Transformer](https://dveon-loch.github.io/case-transformer-rs/) (Please note as the backend is hosted via a free service it may need a minute or two of attempting requests to spin up, after which point requests will respond instantly)

The frontend is hosted via Github Pages, and the backend is hosted via [Render.com](https://render.com)

See below for local running instructions

## Local running instructions

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

## TODO/Improvements

- Dockerize the backend - To ensure ease of installation and running on any hosting service, dockerizing the backend would be a nice addition
- Write a proper frontend with something like the Leptos framework, compile it to WASM, serve it via the backend
- Implement rate limiting on the backend and other safety features
- Fully document the codebase
- Handle CORS with more granularity - currently allowing anything, but with a proper frontend in place I would want to only allow certain headers, origins and methods
- Add a results history, store it to an SQL database for viewing
- Add a login to associate users with their history
