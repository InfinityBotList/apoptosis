# Apoptosis

Apoptosis is the work in progress migration of all IBL services to one single service that handles all the roles of Popplio, Arcadia, Persepolis and Borealis.

Internally, apoptosis uses rust to expose basic db structures etc. as userdata to the luau side which holds the actual business logic.

Apoptosis luau layer is designed to be easily modified by anyone with some knowledge of programming in Lua

## Layers

To make development of Omniplex's many different services/features easier, Apoptosis uses a layer system where each layer is a self contained service containing both Luau and Rust code. See ``src/layers/sample.rs`` for a simple example of what a layer looks like.

Most Omniplex layers will use either pure Rust or a mix of Rust and Luau code. A large amount of the Luau boilerplate code is included in ``service`` module.

## Example Configuration

```json
{
    "base": {
        "max_db_connections": 10,
        "postgres_url": "postgres://infinity:test@localhost/infinity"
    },
    "sample": {
        "foo": "bar"
    }
}
```

## Database Setup (Ubuntu)

First, install Postgres 18 using the below commands (copied from [pgdg](https://www.postgresql.org/download/linux/ubuntu/)):

```bash
# Import the repository signing key:
sudo apt install curl ca-certificates
sudo install -d /usr/share/postgresql-common/pgdg
sudo curl -o /usr/share/postgresql-common/pgdg/apt.postgresql.org.asc --fail https://www.postgresql.org/media/keys/ACCC4CF8.asc

# Create the repository configuration file:
. /etc/os-release
sudo sh -c "echo 'deb [signed-by=/usr/share/postgresql-common/pgdg/apt.postgresql.org.asc] https://apt.postgresql.org/pub/repos/apt $VERSION_CODENAME-pgdg main' > /etc/apt/sources.list.d/pgdg.list"

# Update the package lists:
sudo apt update

# Install PostgreSQL 18:
sudo apt install postgresql-18
```

Setup a database and user for Omniplex:

```bash
sudo -u postgres psql
CREATE USER infinity WITH PASSWORD 'test'; -- Change password as needed
CREATE DATABASE infinity OWNER infinity;
```

## Vendored code

Large parts of luacore/discord are vendored from the Khronos runtime and modified to work with Omniplex's needs