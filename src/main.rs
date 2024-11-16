// #![warn(missing_docs)]

use std::{ fmt::format, ops::Index };

use reqwest::{ self, blocking::get, Error, Version };
use scraper::{ element_ref::Select, error, html, node::Element, ElementRef, Html, Selector };

#[derive(Debug)]
/*  */
struct Event {
    base_url: String,
    rel_permalink: String,
    permalink: String,
    title: String,
    subtitle: String,
    description: String,
    //starting_date: Option<String>,
    //ending_date: String,
}

impl Event {
    fn event_constructor(base_url: String, node: ElementRef<'_>) -> Self {
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

        /* Getting subtitle */
        let subtitle_sel = match Selector::parse(".col-lg-8 h4 ") {
            Ok(selector) => selector,
            Err(err) => panic!("Couldnt parse selector: {err:?}"),
        };
        let subtitle_node = event_page.select(&subtitle_sel);

        /* Getting description */
        let description_sel = match Selector::parse("#_event_estesa *") {
            Ok(selector) => selector,
            Err(err) => panic!("Couldnt parse selector: {err:?}"),
        };
        let description_nodes = event_page.select(&description_sel);

        Self {
            base_url: base_url,
            rel_permalink: optional_rel_permalink,
            permalink: permalink,
            title: node.value().name().to_string(),
            subtitle: match get_element_value(subtitle_node) {
                Some(subtitle) => subtitle,
                None => "".to_string(),
            },
            description: match get_element_value(description_nodes) {
                Some(description) => description,
                None => "".to_string(),
            },
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
fn get_element_value(nodes: scraper::html::Select<'_, '_>) -> Option<String> {
    for node in nodes {
        return Some(node.text().collect::<String>().trim().to_owned());
    }

    None
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
    let sito_comune = "https://www.comune.montopoli.pi.it".to_string();
    let pagina_eventi = "/home/vivere/eventi.html".to_string();
    let mydocument = get_document(&build_permalink(&sito_comune, &pagina_eventi))?;

    let card_link_sel = Selector::parse(".eventi-elenco .cmp-list-card-img__body-title a")?;
    let card_link_nodes: html::Select<'_, '_> = mydocument.select(&card_link_sel);

    let optional_card_link_node = get_single_elementref(card_link_nodes);
    let card_link_node = match optional_card_link_node {
        Some(card_link_node) => card_link_node,
        None => panic!("sad"),
    };

    /**
     * Builds a permalink from a base url and a relative permalink
     */
    fn build_permalink(base_url: &str, rel_permalink: &str) -> String {
        let permalink = format!("{}{}", base_url, rel_permalink);
        permalink
    }

    let event1 = Event::event_constructor(sito_comune, card_link_node);

    dbg!(event1);

    Ok(())
}
