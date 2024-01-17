pub fn scrap() {
    let response = reqwest::blocking::get("https://www.menicka.cz/brno.html");
    let html_content = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&html_content);
    let html_selector = scraper::Selector::parse("div.menicka_detail").unwrap();
    let menu_list = document.select(&html_selector);
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
            println!("  {meal}");
        }
    }
}
