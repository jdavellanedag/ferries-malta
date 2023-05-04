pub fn get_ferry_status() -> String {
    const FERRY_URL: &str = "https://vallettaferryservices.com/terms/";
    const FERRY_URL_SELECTOR: &str = "div.sg-popup-id-5509>span";

    let response = reqwest::blocking::get(FERRY_URL).unwrap().text().unwrap();

    let document = scraper::Html::parse_document(&response);

    let ferry_status_selector = scraper::Selector::parse(FERRY_URL_SELECTOR).unwrap();

    let ferry_status: String = document
        .select(&ferry_status_selector)
        .map(|x| x.inner_html())
        .collect();
    ferry_status
}
