// #![warn(missing_docs)]

use core::panic;
use std::{ fmt::format, ops::Index, option };

use reqwest::{ self, blocking::get, Error, Version };
use scraper::{
    element_ref::Select,
    error,
    html,
    node::Element,
    selector,
    ElementRef,
    Html,
    Selector,
};

const SITO_COMUNE: &str = "https://www.comune.montopoli.pi.it";

#[derive(Debug)]
/*  */
struct Event {
    base_url: String,
    rel_permalink: String,
    permalink: String,
    title: String,
    subtitle: String,
    description_title: String,
    description: String,
    location: String,
    image: String,
    //starting_date: Option<String>,
    //ending_date: String,
}

impl Event {
    fn event_constructor(base_url: &str, node: ElementRef<'_>) -> Self {
        /* Finding rel_permalink from cards */
        let optional_rel_permalink = match node.value().attr("href") {
            None => "".to_string(),
            Some(href) => href.to_string(),
        };

        /* Building permalink */
        let permalink = format!("{}{}", base_url, optional_rel_permalink);

        /* Requesting event page */
        let event_page = match get_document(&permalink) {
            Ok(document) => document,
            Err(err) => { panic!("Couldn't get document: {err:?}") }
        };

        /* Getting title */
        let title_sel = match Selector::parse("h1") {
            Ok(selector) => selector,
            Err(err) => panic!("Couldnt parse selector: {err:?}"),
        };
        let title_node = event_page.select(&title_sel);

        /* Getting subtitle */
        let subtitle_sel = match Selector::parse("h1+ h4 ") {
            Ok(selector) => selector,
            Err(err) => panic!("Couldnt parse selector: {err:?}"),
        };
        let subtitle_node = event_page.select(&subtitle_sel);

        /* Getting description title */
        let description_title_sel = match Selector::parse("#_event_estesa h4") {
            Ok(selector) => selector,
            Err(err) => panic!("Couldnt parse selector: {err:?}"),
        };
        let description_title_nodes = event_page.select(&description_title_sel);

        /* Getting description */
        let description_sel = match Selector::parse("#_event_estesa  p") {
            Ok(selector) => selector,
            Err(err) => panic!("Couldnt parse selector: {err:?}"),
        };
        let description_nodes = event_page.select(&description_sel);

        /* Getting location */
        let location_sel = match Selector::parse("#_event_luogo h5") {
            Ok(selector) => selector,
            Err(err) => panic!("Couldnt parse selector: {err:?}"),
        };
        let location_nodes = event_page.select(&location_sel);

        /* Getting post image */
        let image_sel = match Selector::parse(".col-lg-8 img") {
            Ok(selector) => selector,
            Err(err) => panic!("Couldnt parse selector: {err:?}"),
        };
        let image_node = event_page.select(&image_sel);
        let image_rel_permalink = get_single_elementref(image_node)
            .expect("couldn't get image")
            .value()
            .attr("src")
            .expect("couldn't get image source")
            .to_string();

        /* Building struct */
        Self {
            base_url: base_url.to_string(),
            rel_permalink: optional_rel_permalink,
            permalink: permalink,
            title: get_element_value(title_node),
            subtitle: get_element_value(subtitle_node),
            description_title: get_element_value(description_title_nodes),
            description: get_element_value(description_nodes),
            location: get_element_value(location_nodes),
            image: build_permalink(&SITO_COMUNE, &image_rel_permalink),
        }
    }
}
/*  */

/**
 * This function makes an http request and checks if it can get a response, it also parses the HTML body from the response, the error handling is managed by reqwest!
 */
fn get_document(permalink: &str) -> Result<Html, reqwest::Error> {
    let response = reqwest::blocking::get(permalink)?;

    let html_content = response.text()?;

    let document = Html::parse_document(&html_content);

    Ok(document)
}

fn get_single_elementref<'a>(nodes: scraper::html::Select<'a, '_>) -> Option<ElementRef<'a>> {
    for node in nodes {
        return Some(node);
    }

    None
}

/* This function gets element values from selected nodes*/
fn get_element_value(nodes: scraper::html::Select<'_, '_>) -> String {
    let mut joined_text: String = "".to_string();

    for node in nodes {
        let optional_value = Some(node.text().collect::<String>().trim().to_owned());

        let value = match optional_value {
            Some(value) => value,
            None => panic!("No value in nodes"),
        };

        joined_text.push_str(&value);
    }

    joined_text
}

/**
 * Builds a permalink from a base url and a relative permalink
 */
fn build_permalink(base_url: &str, rel_permalink: &str) -> String {
    let permalink = format!("{}{}", base_url, rel_permalink);
    permalink
}

//#TODO
/* fn check_if_date(date: Option<String>) -> Option<String> {
    match date {
        Some(date) => Some(date),
        None => None,
    }
} */

/**
 * main
 */
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pagina_eventi = "/home/vivere/eventi.html".to_string();
    let mydocument = get_document(&build_permalink(&SITO_COMUNE, &pagina_eventi))?;

    let card_link_sel = Selector::parse(".eventi-elenco .cmp-list-card-img__body-title a")?;
    let card_link_nodes: html::Select<'_, '_> = mydocument.select(&card_link_sel);

    let optional_card_link_node = get_single_elementref(card_link_nodes);
    let card_link_node = match optional_card_link_node {
        Some(card_link_node) => card_link_node,
        None => panic!("sad"),
    };

    let event1 = Event::event_constructor(SITO_COMUNE, card_link_node);

    dbg!(event1);

    Ok(())
}
