use chrono::NaiveDate;
use db::db::models::{Menu, MenuItem};
use regex::Regex;
use scraper::{Html, Selector};
use scraper::element_ref::Select;
use sqlx::types::JsonValue::String;

struct RestaurantAddress {
    street: String,
    number: String,
    zip: String,
    city: String,
}

fn get_restaurant_html(link: String) -> Html {
    let response = reqwest::blocking::get(link);
    let html_content = response.unwrap().text().unwrap();
    Html::parse_document(&html_content)
}

pub fn scrap_restaurant(link: String) {
    let document = get_restaurant_html(link);
    let name = get_restaurant_name(&document);
    println!("NAME: {name}");
    let address = get_restaurant_address(&document);
    println!(
        "Street: {}, number: {}, zip: {}, city: {}",
        address.street, address.number, address.zip, address.city
    );

    let open_hours = get_restaurant_open_hours(&document);
    let lunch_time = get_lunch_time(&document);
    println!("Lunch time: {}", lunch_time.unwrap());
    let img_link = get_image_link(&document);
    println!("{}", img_link.unwrap());
    let phone = get_restaurant_phone(&document);
    println!("{}", phone.unwrap());
    let email = get_restaurant_email(&document);
    println!("{}", email.unwrap());
    let www = get_restaurant_www(&document);
    println!("{}", www.unwrap());
}

fn get_restaurant_menus(html: &Html) -> Vec<Menu> {
    let selector = Selector::parse("div.menicka").unwrap();
    let menu_elements = html.select(&selector);
    let menus : Vec<Menu> = Vec::new();
    for menu_element in menu_elements {
        let date_selector = Selector::parse("div.nadpis").unwrap();
        let date_element = menu_element.select(&date_selector).next().expect("Restaurant menu date structure changed");
        let title = date_element.inner_html();
        let date = parse_menu_date_from_title(title);

        let soups_selector = Selector::parse("li.polevka").unwrap();
        let soup_elements = menu_element.select(&soups_selector);
        let meals_selector = Selector::parse("li.jidlo").unwrap();
        let meals_element = menu_element.select(&meals_selector);
        let mut menu_items : Vec<MenuItem> = Vec::new();

    }

    menus
}

fn get_menu_soups(select: Select) -> Vec<MenuItem> {
    let mut soups : Vec<MenuItem> = Vec::new();
    for soup_element in select {

    }

    soups
}

fn parse_menu_date_from_title(title: String) -> NaiveDate {
    let date_string = title.split(" ").last().expect("Restaurant menu date structure changed");
    let date_arr = date_string.split(".").map(move |x| {
        x.to_string()
    })
        .collect::<Vec<String>>();
    let date = NaiveDate::from_ymd_opt(date_arr[2], date_arr[1], date_arr[0]).expect("Restaurant menu date structure changed");
    date
}

fn get_restaurant_phone(html: &Html) -> Option<String> {
    let link = html
        .select(&Selector::parse("a.telefon").unwrap())
        .next()?
        .value()
        .attr("href")?
        .to_owned();
    let link = link.replacen(".", "https://www.menicka.cz", 1);
    let response = reqwest::blocking::get(link);
    let html_content = response.unwrap().text().unwrap();
    let document = Html::parse_document(&html_content);
    let selector = Selector::parse("a").unwrap();
    let mut phone_element = document.select(&selector);
    let phone = phone_element.next()?;
    Some(phone.inner_html())
}

fn get_restaurant_email(html: &Html) -> Option<String> {
    let link = html
        .select(&Selector::parse("a.email").unwrap())
        .next()?
        .value()
        .attr("href")?
        .to_owned();
    let link = link.replacen(".", "https://www.menicka.cz", 1);
    let response = reqwest::blocking::get(link);
    let html_content = response.unwrap().text().unwrap();
    let document = Html::parse_document(&html_content);
    let selector = Selector::parse("a").unwrap();
    let mut email_element = document.select(&selector);
    let email = email_element.next()?;
    Some(email.inner_html())
}

fn get_restaurant_www(html: &Html) -> Option<String> {
    let link = html
        .select(&Selector::parse("a.web").unwrap())
        .next()?
        .value()
        .attr("href")?
        .to_owned();
    let link = link.replacen(".", "https://www.menicka.cz", 1);
    return Some(link);
}

fn get_image_link(html : &Html) -> Option<String> {
    let restaurant_link = html
        .select(&Selector::parse("img.photo").unwrap())
        .next();
    let relative_link = match restaurant_link {
        None => None,
        Some(element) => {
            let src = element
                .value()
                .attr("src");
            match src {
                None => None,
                Some(attr) => Some(attr.to_string())
            }
        }
    };

    if let Some(link) = relative_link {
        return Some(link.replace("..", "https://www.menicka.cz"));
    }

    None
}

fn get_lunch_time(html: &Html) -> Option<String> {
    let selector = Selector::parse("div.obedovycas").unwrap();
    let mut div = html.select(&selector);
    let time = div.next();
    match time {
        None => None,
        Some(lunch) => {
            let html = Html::parse_document(lunch.inner_html().as_str());
            let em = html
                .select(&Selector::parse("em").unwrap())
                .next();
            match em {
                None => None,
                Some(lunch_time) => Some(lunch_time.inner_html())
            }
        }
    }
}

fn get_restaurant_open_hours(html: &Html) -> Vec<Option<String>> {
    let selector = Selector::parse("span.cas").unwrap();
    let times = html.select(&selector);
    let mut result : Vec<Option<String>> = Vec::new();
    for time in times {
        result.push(Some(time.inner_html()));
        println!("{}", time.inner_html());
    }

    result
}

fn get_restaurant_name(html: &Html) -> String {
    let html = html
        .select(&Selector::parse("h1").unwrap())
        .next()
        .expect("Html structure for restaurant name changed")
        .inner_html();
    remove_trailing_tags(html)
}

fn get_restaurant_address(html: &Html) -> RestaurantAddress {
    let address_html = html
        .select(&Selector::parse("div.adresa").unwrap())
        .next()
        .expect("Html structure for restaurant address changed")
        .inner_html();

    let address = Html::parse_document(&address_html)
        .select(&Selector::parse("a").unwrap())
        .next()
        .expect("Html structure for restaurant address changed")
        .inner_html();

    let mut arr = address.split(", ");
    let street = arr
        .next()
        .expect("Html structure for restaurant address changed")
        .to_string();
    let number = arr
        .next()
        .expect("Html structure for restaurant address changed")
        .to_string();
    let zip = arr
        .next()
        .expect("Html structure for restaurant address changed")
        .to_string();
    let city = arr
        .next()
        .expect("Html structure for restaurant address changed")
        .to_string();

    RestaurantAddress {
        street,
        number,
        zip,
        city,
    }
}

pub fn scrap_menus_today() -> Vec<Menu> {
    let response = reqwest::blocking::get("https://www.menicka.cz/brno.html");
    let html_content = response.unwrap().text().unwrap();
    let document = Html::parse_document(&html_content);
    let html_selector = Selector::parse("div.menicka_detail").unwrap();
    let menu_list = document.select(&html_selector);
    let result: Vec<Menu> = Vec::new();
    for menu in menu_list {
        let info = menu
            .select(&Selector::parse("div.nazev").unwrap())
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>();
        let restaurant_name = info.first().unwrap().to_string();

        let restaurant_link = menu
            .select(&Selector::parse("a.noborder").unwrap())
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap()
            .to_owned();

        println!("NÁZEV: {}", restaurant_name);
        println!("LINK: {}", restaurant_link);

        let meals = menu
            .select(&Selector::parse("div.nabidka_1").unwrap())
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

fn parse_meal_name(name: String) -> (bool, String) {
    let is_soup = name.contains("<i>");
    if is_soup {
        let no_tags = name.replace("<i>", "").replace("</i>", "");
        return (true, remove_trailing_tags(no_tags));
    }

    (false, remove_trailing_tags(name))
}

fn remove_trailing_tags(str: String) -> String {
    let regex = Regex::new("^(.*?)<").unwrap();
    let m = regex.find(str.as_str());
    match m {
        None => str,
        Some(m) => return str[m.start()..m.end() - 1].to_string(),
    }
}
