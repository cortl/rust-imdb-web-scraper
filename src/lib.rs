#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate reqwest;
extern crate select;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use scraper::{Html, Selector};
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use std::error::Error;
use std::result::Result;
use std::fmt;

pub struct Episode {
    name: String,
    number: usize,
    rating: f32,
    description: String,
}

pub struct Season {
    number: u32,
    episodes: Vec<Episode>,
}

pub struct Show {
    name: String,
    description: String,
    seasons: Vec<Season>,
}

impl fmt::Display for Show {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", self.name)?;
        write!(f, "{}\n", self.description.trim())?;
        for season in &self.seasons {
            write!(f, "Season No. {}\n", season.number)?;
            for episode in &season.episodes {
                write!(f, "{}. {}, Rating {}\n", episode.number, episode.name, episode.rating)?;
                write!(f, " {}\n", episode.description)?;
            }
        }
        Ok(())
    }
}

fn get_season(season_number: String) -> Result<Season, Box<Error>> {
    let season_url: String = format!(
        "https://www.imdb.com/title/tt0386676/episodes?season={}",
        season_number
    );
    let body = reqwest::get(&season_url)?.text()?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse(".list_item").unwrap();

    let mut episodes: Vec<Episode> = Vec::new();
    for (i, element) in document.select(&selector).enumerate() {
        let star_selector = Selector::parse(".ipl-rating-star__rating").unwrap();
        let stars: f32 = element
            .select(&star_selector)
            .next()
            .unwrap()
            .inner_html()
            .parse()
            .unwrap();

        let description_selector = Selector::parse(".item_description").unwrap();
        let description = element
            .select(&description_selector)
            .next()
            .unwrap()
            .inner_html()
            .trim()
            .to_string();

        let title_selector = Selector::parse("a[itemprop=name]").unwrap();
        let title = element.select(&title_selector).next().unwrap().inner_html();

        let ep = Episode {
            name: title,
            rating: stars,
            number: i,
            description,
        };
        episodes.push(ep);
    }
    Ok(Season {
        number: season_number.parse().unwrap(),
        episodes,
    })
}

#[wasm_bindgen]
pub fn get_show() -> Result<Show, Box<Error>> {
    let resp = reqwest::get("https://www.imdb.com/title/tt0386676").unwrap();
    assert!(resp.status().is_success());

    let document = Document::from_read(resp).unwrap();
    let summary = document.find(Class("summary_text")).next().unwrap().text();

    let seasons = document
        .find(Class("seasons-and-year-nav").descendant(Name("div")))
        .nth(2)
        .unwrap()
        .find(Name("a"))
        .map(|season| season.text())
        .map(|season| get_season(season))
        .map(|season| season.unwrap())
        .collect::<Vec<_>>();
    let show = Show {
        name: String::from("The Office"),
        description: summary,
        seasons,
    };
    Ok(show)
}
