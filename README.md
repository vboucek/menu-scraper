# Menu Scraper
Aplikace Menu Scraper scrapuje meníčka brněnských restaurací. Scrapování probíhá vždy jednou denně v 8:00 a také při každém spuštění Actix serveru. Scrapování všech meníček může trvat v řádu minut.

Aplikace umožňuje registrovat a vytvářet skupiny uživatelů. V rámci skupiny je možné vytvářet obědy a poté hlasovat pro výběr menu pro daný oběd.

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
sqlx database setup
`

### Spuštění aplikace:
`
cargo run
`
