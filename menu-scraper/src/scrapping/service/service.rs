pub fn scrap() {
    let response = reqwest::blocking::get("https://www.menicka.cz/brno.html");
    let html_content = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&html_content);
    let html_selector = scraper::Selector::parse("div.menicka_detail").unwrap();
    let menicka = document.select(&html_selector);
    //println!("{}", menicka.count());
    for menu in menicka {
        let info = menu.select(&scraper::Selector::parse("div.nazev").unwrap()).next().unwrap().text().collect::<Vec<_>>();
        println!("{}", info.first().unwrap().to_string());
            // .and_then(|a| a.value().attr("info"))
            // .map(str::to_owned);
        // if let Some(inside) = info {
        //     return inside;
        // }
    }
}
