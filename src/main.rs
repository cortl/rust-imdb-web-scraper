extern crate web_scraper;

use std::error::Error;
use std::result::Result;

fn main() -> Result<(), Box<Error>> {
    let show = web_scraper::get_show().unwrap();
    println!("{}", show);
    Ok(())
}
