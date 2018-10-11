use state;
use state::*;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use utility::*;
use std::thread;
use sublime_fuzzy::FuzzySearch;
use edit_distance::edit_distance;

fn google_fonts () {
    let apikey = "AIzaSyD537Aqht7lclUP-nLeAKteQZyzrCjgV00";
}

impl FontCatalog {
    fn cancel (&mut self) {
      self.loaders.iter_mut().for_each(|ref mut thread| { thread.cancel(); });
    }

    fn filter (&mut self) {
        let mut cache = &self.cache;
    
        let newres =
            self.filters.iter().fold(cache.intermediate.iter().collect(), |res: Vec<&(SavedWithFont, FontCatalogSource)>, filter| {
                match filter {
                    FontCatalogFilter::Search(str) => {
                        res.into_iter().filter(|(SavedWithFont(_, font), source)| {
                            let fontname = &font.name;
                            edit_distance(&str, fontname) < 5
                        }).collect()
                    },
                    FontCatalogFilter::Flag(flag) => {
                        res.into_iter().filter(|(SavedWithFont(saved, font), source)| {
                            match flag { _ => true }
                        }).collect()
                    }
                }
            });

        self.results = pipe!(newres.iter().map(| (font, _) | SavedWithFont::clone(font)).collect() => Ok => Some);
    }

    fn query (&mut self) -> Vec<Font> {
       self.query.iter().fold(Vec::new(), |res:Vec<Font>, query| {
           match query {
               FontCatalogSource::Google => {
                   
               },
               FontCatalogSource::Folder (folder) => {}
            }

           res
       })
    }

    fn receive(&mut self, msg:CatalogMessage) {
        match msg {
            CatalogMessage::ToggleQuery(src) => {
                self.query.toggle(src);
            },

            CatalogMessage::ToggleFilter(filter) => {
                self.filters.toggle(filter);
            },

            CatalogMessage::ClearFilters => {
                self.filters = HashSet::new();
            },

            CatalogMessage::SetFonts(res) => {
                self.results = Some(res);
            },

            CatalogMessage::ToggleFlag(FontSelection (sel), flag) => {
                if let FontCatalog {results: Some(Ok(res)), ..} = &self {
                    let i = &res[sel as usize];
                }
            },

            CatalogMessage::SwitchPage(newpage) => {
                self.page = newpage;
            },

            _ => ()
        }
    }
}

pub fn start_receiver(msg:CatalogMessage, state:FontCatalog) {
    let thread = thread::spawn(|| println!("hALLO"));
}