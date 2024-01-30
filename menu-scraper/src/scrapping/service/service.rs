use anyhow::Context;
use chrono::NaiveDate;
use db::db::models::{MenuCreate, MenuItemCreate, RestaurantCreate, RestaurantGetByNameAndAddress};
use regex::Regex;
use scraper::{Html, Selector};
use scraper::element_ref::Select;
use uuid::Uuid;
use db::db::common::DbCreate;
use db::db::repositories::{MenuRepository, RestaurantRepository, SearchRestaurant};

struct RestaurantAddress {
    street: String,
    number: String,
    zip: String,
    city: String,
}

pub async fn scrap(restaurant_repo: RestaurantRepository, menu_repo: MenuRepository) -> anyhow::Result<()> {
    let html_content = reqwest::get("https://www.menicka.cz/brno.html").await?.text().await?;
    let document = Html::parse_document(&html_content);
    let html_selector = Selector::parse("div.menicka_detail").unwrap();
    let menu_list = document.select(&html_selector);

    for menu in menu_list {
        let restaurant_link = menu
            .select(&Selector::parse("a.noborder").unwrap())
            .next()
            .context("No restaurant link")?
            .value()
            .attr("href")
            .context("No restaurant link")?
            .to_owned();

        println!("LINK: {}", restaurant_link);

        let _ = scrap_restaurant(restaurant_link, &restaurant_repo.clone(), &menu_repo.clone()).await;
    }
    Ok(())
}

async fn get_restaurant_html(link: String) -> anyhow::Result<Html> {
    let response = reqwest::get(link).await?;
    let html_content = response.text().await?;
    Ok(Html::parse_document(&html_content))
}

pub async fn scrap_restaurant(link: String, restaurant_repo: &RestaurantRepository, menu_repo: &MenuRepository) -> anyhow::Result<()> {
    let document = get_restaurant_html(link).await?;
    let name = get_restaurant_name(&document)?;
    let address = get_restaurant_address(&document)?;

    let open_hours = get_restaurant_open_hours(&document);
    let lunch_time = get_lunch_time(&document);
    let img_link = get_image_link(&document);
    let phone = get_restaurant_phone(&document).await;
    let email = get_restaurant_email(&document).await;
    let www = get_restaurant_www(&document);

    let get_restaurant = RestaurantGetByNameAndAddress {
        name,
        street: address.street,
        house_number: address.number,
        zip_code: address.zip,
        city: address.city,
    };

    let found_restaurant = restaurant_repo.search_restaurant(&get_restaurant).await?;

    if let Some(restaurant_id) = found_restaurant {
        let menus = get_restaurant_menus(&document, restaurant_id.id)?;
        for menu in menus {
            if menu.items.is_empty() {
                continue;
            }

            menu_repo.create(&menu).await?;
        }
    } else {
        let restaurant_create = RestaurantCreate {
            name: get_restaurant.name,
            street: get_restaurant.street,
            house_number: get_restaurant.house_number,
            zip_code: get_restaurant.zip_code,
            city: get_restaurant.city,
            picture: img_link,
            phone_number: phone,
            website: www,
            email,
            monday_open: open_hours[0].to_owned(),
            tuesday_open: open_hours[1].to_owned(),
            wednesday_open: open_hours[2].to_owned(),
            thursday_open: open_hours[3].to_owned(),
            friday_open: open_hours[4].to_owned(),
            saturday_open: open_hours[5].to_owned(),
            sunday_open: open_hours[6].to_owned(),
            lunch_served: lunch_time,
        };

        let restaurant_id = restaurant_repo.create(&restaurant_create).await?;
        let menus = get_restaurant_menus(&document, restaurant_id.id)?;
        for menu in menus {
            if menu.items.is_empty() {
                continue;
            }

            menu_repo.create(&menu).await?;
        }
    }

    Ok(())
}

fn get_restaurant_menus(html: &Html, restaurant_id: Uuid) -> anyhow::Result<Vec<MenuCreate>> {
    let selector = Selector::parse("div.menicka").unwrap();
    let menu_elements = html.select(&selector);
    let mut menus : Vec<MenuCreate> = Vec::new();
    for menu_element in menu_elements {
        let date_selector = Selector::parse("div.nadpis").unwrap();
        let date_element = menu_element.select(&date_selector).next().context("No restaurant date")?;
        let title = remove_trailing_tags(date_element.inner_html()).trim().to_string();
        let date = parse_menu_date_from_title(title)?;

        let soups_selector = Selector::parse("li.polevka").unwrap();
        let soup_elements = menu_element.select(&soups_selector);
        let soups = get_menu_soups(soup_elements)?;
        let meals_selector = Selector::parse("li.jidlo").unwrap();
        let meals_element = menu_element.select(&meals_selector);
        let meals = get_menu_meals(meals_element)?;
        let mut menu_items : Vec<MenuItemCreate> = Vec::new();
        menu_items.extend(soups);
        menu_items.extend(meals);
        let menu = MenuCreate{
            date,
            items: menu_items,
            restaurant_id
        };

        menus.push(menu);
    }

    Ok(menus)
}

fn get_menu_meals(select: Select) -> anyhow::Result<Vec<MenuItemCreate>> {
    let mut meals : Vec<MenuItemCreate> = Vec::new();
    for meal_element in select {
        let name_selector = Selector::parse("div.polozka").unwrap();
        let name = meal_element.select(&name_selector).next();
        let Some(name) = name else {
            return Ok(meals);
        };
        let name = remove_trailing_tags(remove_leading_tags(name.inner_html()).replace("&nbsp;", " ")).trim().to_string();

        let price_selector = Selector::parse("div.cena").unwrap();
        let price = meal_element.select(&price_selector)
            .next();

        let mut item = MenuItemCreate{
            name,
            is_soup: false,
            size: "".to_string(),
            price: 0
        };

        let Some(price) = price else {
            meals.push(item);
            continue;
        };
        let price_string : String = price.inner_html().chars().filter(|c| c.is_digit(10)).collect();
        item.price = price_string.parse()?;
        meals.push(item);
    }

    Ok(meals)
}

fn get_menu_soups(select: Select) -> anyhow::Result<Vec<MenuItemCreate>> {
    let mut soups : Vec<MenuItemCreate> = Vec::new();
    for soup_element in select {
        let name_selector = Selector::parse("div.polozka").unwrap();
        let name = soup_element.select(&name_selector)
            .next();
        let Some(name) = name else {
            return Ok(soups);
        };
        let name = remove_trailing_tags(name.inner_html()).trim().replace("&nbsp;", " ");

        let price_selector = Selector::parse("div.cena").unwrap();
        let price = soup_element.select(&price_selector)
            .next();
        let mut item = MenuItemCreate{
            is_soup: true,
            name,
            price: 0,
            size: "".to_string(),
        };
        let Some(price) = price else {
            soups.push(item);
            continue;
        };
        let price_string : String = price.inner_html().chars().filter(|c| c.is_digit(10)).collect();
        item.price = price_string.parse()?;
        soups.push(item);
    }

    Ok(soups)
}

fn parse_menu_date_from_title(title: String) -> anyhow::Result<NaiveDate> {
    let date_string = title.split(" ").last().context("Error while parsing menu date")?;
    let date_arr = date_string.split(".").map(move |x| {
        x.to_string()
    })
        .collect::<Vec<String>>();
    let date = NaiveDate::from_ymd_opt(date_arr[2].parse()?, date_arr[1].parse()?, date_arr[0].parse()?).context("Error constructing menu date")?;
    Ok(date)
}

async fn get_restaurant_phone(html: &Html) -> Option<String> {
    let link = html
        .select(&Selector::parse("a.telefon").unwrap())
        .next()?
        .value()
        .attr("href")?
        .to_owned();
    let link = link.replacen(".", "https://www.menicka.cz", 1);
    let html_content = reqwest::get(link).await?.text().await?;
    let document = Html::parse_document(&html_content);
    let selector = Selector::parse("a").unwrap();
    let mut phone_element = document.select(&selector);
    let phone = phone_element.next()?;
    Some(phone.inner_html())
}

async fn get_restaurant_email(html: &Html) -> Option<String> {
    let link = html
        .select(&Selector::parse("a.email").unwrap())
        .next()?
        .value()
        .attr("href")?
        .to_owned();
    let link = link.replacen(".", "https://www.menicka.cz", 1);
    let html_content = reqwest::get(link).await?.text().await?;
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
        let time_str = time.inner_html();
        if time_str.is_empty() {
            result.push(None);
        } else {
            result.push(Some(time_str));
        }
    }

    result
}

fn get_restaurant_name(html: &Html) -> anyhow::Result<String> {
    let html = html
        .select(&Selector::parse("h1").unwrap())
        .next()
        .context("Error getting restaurant name")?
        .inner_html();
    Ok(remove_trailing_tags(html))
}

fn get_restaurant_address(html: &Html) -> anyhow::Result<RestaurantAddress> {
    let address_html = html
        .select(&Selector::parse("div.adresa").unwrap())
        .next()
        .context("No restaurant address")?
        .inner_html();

    let address = Html::parse_document(&address_html)
        .select(&Selector::parse("a").unwrap())
        .next()
        .context("No restaurant address")?
        .inner_html();

    let mut arr = address.split(", ");
    let street = arr
        .next()
        .context("No restaurant street")?
        .to_string();
    let number = arr
        .next()
        .context("No restaurant number")?
        .to_string();
    let zip = arr
        .next()
        .context("No restaurant zip")?
        .to_string();
    let city = arr
        .next()
        .context("No restaurant city")?
        .to_string();

    Ok(RestaurantAddress {
        street,
        number,
        zip,
        city,
    })
}

pub fn scrap_menus_today() -> Vec<MenuCreate> {
    let response = reqwest::blocking::get("https://www.menicka.cz/brno.html");
    let html_content = response.unwrap().text().unwrap();
    let document = Html::parse_document(&html_content);
    let html_selector = Selector::parse("div.menicka_detail").unwrap();
    let menu_list = document.select(&html_selector);
    let result: Vec<MenuCreate> = Vec::new();
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

fn remove_leading_tags(str: String) -> String {
    let regex = Regex::new("^(.*?)</span>").unwrap();
    let m = regex.find(str.as_str());
    match m {
        None => str,
        Some(m) => return str[m.end()..str.len()].to_string(),
    }
}
