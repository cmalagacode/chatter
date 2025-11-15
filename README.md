## Install Diesel ORM CLI From Source Code
```bash

cargo install diesel_cli --no-default-features --features postgres,sqlite
```

## System Diesel CLI Dependency
```bash

sudo apt install libpg-dev libsqlite3-dev
```

```bash

brew install libpq
brew install sqlite
```

## Setup Database Migrations
```bash

diesel setup
diesel migration generate example_migration
```

## Run Migrations or Redo
```bash

diesel migration run --database-url $CHATTER_DATABASE_URL
diesel migration redo
```

## Serialization Validator Crates
- https://crates.io/crates/validator
- https://github.com/gengteng/axum-valid

