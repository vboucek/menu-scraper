# Menu Scraper



## Návod ke spuštění

### Databáze
Aplikace využívá Postgres databázi.

Spuštění databáze v kontejneru:

`
cp .env.example .env
`

`
docker compose up
`

Aplikace využivá SQLx:

`
cargo install sqlx-cli
`

Vytvoření databáze:

`
sqlx database create
`

### Spuštění aplikace:
`
cargo run
`


