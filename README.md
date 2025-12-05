# Apoptosis

Apoptosis is the work in progress migration of all IBL services to one single service that handles all the roles of Popplio, Arcadia, Persepolis and Borealis.

Internally, apoptosis uses rust to expose basic db structures etc. as userdata to the luau side which holds the actual business logic.

Apoptosis luau layer is designed to be easily modified by anyone with some knowledge of programming in Lua

## Layers

To make development of Omniplex's many different services/features easier, Apoptosis uses a layer system where each layer is a self contained service containing both Luau and Rust code. See ``src/layers/sample.rs`` for a simple example of what a layer looks like

## Example Configuration

```json
{
    "base": {
        "max_db_connections": 10,
        "postgres_url": "postgres://postgres:test@localhost/infinity"
    },
    "sample": {
        "foo": "bar"
    }
}
```