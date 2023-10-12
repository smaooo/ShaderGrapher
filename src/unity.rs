use std::f32::consts::E;

const CORE_PACKAGES_URL: &str = "https://docs.unity3d.com/Manual/pack-core.html";

pub fn get_nodes() {
    // getting latest link to Shader Graph core package page
    let mut response = reqwest::blocking::get(CORE_PACKAGES_URL)
        .unwrap()
        .text()
        .unwrap();

    let document = scraper::Html::parse_document(&response);

    let mut ul_selector: scraper::Selector = scraper::Selector::parse("ul>li").unwrap();

    let href = document
        .select(&ul_selector)
        .flat_map(|ul| ul.children())
        .filter(|child| match child.value().as_text() {
            Some(text) => text.contains("Shader Graph"),
            None => false,
        })
        .flat_map(|child| child.parent().unwrap().children())
        .map(|child| child.value())
        .filter(|child| child.is_element())
        .map(|child| child.as_element().unwrap().attr("href").unwrap())
        .collect::<Vec<&str>>()[0];

    // response = reqwest::blocking::get(href).unwrap().text().unwrap();
    
    // ul_selector = scraper::Selector::parse("ul").unwrap();
    // let a = document.select(&ul_selector);
    // for i in a {
        
    //     println!("{:?}", i.value().has_class("nav", scraper::CaseSensitivity::AsciiCaseInsensitive));
    // }
}
