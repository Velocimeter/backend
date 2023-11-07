# <h1 align="center"> Velocimeter backend </h1>

Velocimeter backend service. Axum API. Postrgres DB via SEA-ORM. Ethers.rs

## Quickstart

Create an .env file based on .env.example.

Make sure Docker is installed, postgres image.

Run:

```
docker compose up -d --wait
```

Note:
When closing processes run:

```
docker compose down -v
```

to reset database.

## Generating DB instances

Install sea-orm-cli.

Update `database/init.sql` with tables.

Agains running postgres db run:

```
sea-orm-cli generate entity -o src/database --with-serde both
```
