# Menu Scraper
Semestral project of course PV281 - Programming in Rust.

The Menu Scraper application scrapes menus of Brno restaurants. Scraped menus can be viewed on a website and ordered by price and distance to the restaurant. Scraping takes place once a day at 8:00 a.m. and every time the Actix server is started.

The application allows you to register and create groups of users. Within the group, it is possible to create lunches and then vote to choose the menu for that lunch.

Used technologies:
- Frontend: HTML, CSS, HTMX, JS
- Backend: Actix, Askama templates, Postgres DB, Sqlx
- Scraping: Scraper, Reqwest, Geocoding, OpenCage API

<img width="2418" alt="homepage" src="https://github.com/vboucek/menu-scraper/assets/72857024/5ffaba94-9818-4b08-a01f-964201ddca9d">

## How to use

### Database
Application uses Postgres DB:

For starting the database using Docker:

`
cp .env.example .env
`

`
docker compose up
`

Application uses SQLX:

`
cargo install sqlx-cli
`

Creating the database and applying the migration:

`
sqlx database setup
`

### Running the Actix server:
`
cargo run
`
