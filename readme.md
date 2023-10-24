# JSON Fields

Library for creating JSON schemas which support mutating the schema and the checked data using migrations.

> **Warning**
> This library is work in progress and should not be used in production yet.

## Usage

Because this library is still work in progress, it's not available on crates.io yet. But if you want to play around with
this unstable version, you can add this as a git dependency like so:

```toml
json_fields = { git = "https://github.com/Jumpdrive-dev/Json-Fields", rev = "<commit to use>" }
```

## Features

This is a list of current and future features that this library supports:

- [ ] Creating schemas by combining different fields.
- [ ] Ability to change a schema and create migrations
- [ ] Migrations are checked using the new resulting schema to ensure migrations result in the correct data shape. 
