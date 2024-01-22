use regex::Regex;
use db::db::models::Menu;

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

        println!("{}", restaurant_name);

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
