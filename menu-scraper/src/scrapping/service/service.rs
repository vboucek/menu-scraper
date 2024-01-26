use regex::Regex;
use scraper::Html;
use db::db::models::Menu;

struct RestaurantAddress {
    street: String,
    number: String,
    zip: String,
    city: String,
}

pub fn scrap_restaurant(link: String) {
    let response = reqwest::blocking::get(link);
    let html_content = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&html_content);
    let address = get_restaurant_address(&document);
    println!("Street: {}, number: {}, zip: {}, city: {}", address.street, address.number, address.zip, address.city);
}

fn get_restaurant_address(html: &Html) -> RestaurantAddress {
    let address_html = html
        .select(&scraper::Selector::parse("div.adresa").unwrap())
        .next()
        .expect("Html strucutre for restaurant adress changed")
        .inner_html();

    let address = scraper::Html::parse_document(&address_html)
        .select(&scraper::Selector::parse("a").unwrap())
        .next()
        .expect("Html strucutre for restaurant adress changed")
        .inner_html();
    // .first_child()
    // .expect("Html strucutre for restaurant adress changed")
    // .value()
    println!("{address}");
    let mut arr = address.split(", ");
    let street = arr.next().expect("Html strucutre for restaurant adress changed").to_string();
    let number = arr.next().expect("Html strucutre for restaurant adress changed").to_string();
    let zip = arr.next().expect("Html strucutre for restaurant adress changed").to_string();
    let city = arr.next().expect("Html strucutre for restaurant adress changed").to_string();

    RestaurantAddress{
        street,
        number,
        zip,
        city
    }
}

pub fn scrap_menus_today() -> Vec<Menu> {
    let response = reqwest::blocking::get("https://www.menicka.cz/brno.html");
    let html_content = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&html_content);
    let html_selector = scraper::Selector::parse("div.menicka_detail").unwrap();
    let menu_list = document.select(&html_selector);
    let result : Vec<Menu> = Vec::new();
    for menu in menu_list {
        let info = menu
            .select(&scraper::Selector::parse("div.nazev").unwrap())
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>();
        let restaurant_name = info.first().unwrap().to_string();

        let restaurant_link = menu
            .select(&scraper::Selector::parse("a.noborder").unwrap())
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap()
            .to_owned();

        println!("NÁZEV: {}", restaurant_name);
        println!("LINK: {}", restaurant_link);

        let meals = menu
            .select(&scraper::Selector::parse("div.nabidka_1").unwrap())
            .map(|m| m.inner_html())
            .collect::<Vec<_>>();
        for meal in meals {
            let (is_soup, name) = parse_meal_name(meal);
            if is_soup {
                println!("  POLÉVKA: {name}");
            } else {
                println!("  {name}");
            }
        }
    }

    result
}

fn parse_meal_name(name : String) -> (bool, String) {
    let is_soup = name.contains("<i>");
    if is_soup {
        let no_tags = name.replace("<i>", "").replace("</i>", "");
        return (true, remove_trailing_tags(no_tags));
    }

    (false, remove_trailing_tags(name))
}

fn remove_trailing_tags(str : String) -> String {
    let regex = Regex::new("^(.*?)<").unwrap();
    let m = regex.find(str.as_str());
    match m {
        None => str,
        Some(m) => {
            return str[m.start()..m.end() - 1].to_string()
        }
    }
}
