mod scrapping;

fn main() {
    //println!("RESTAURACE");
    //scrapping::service::service::scrap_menus_today();
    scrapping::service::service::scrap_restaurant(
        "https://www.menicka.cz/2645-na-chate.html".to_string(),
    );
}
