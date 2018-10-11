use std::thread;
use std::sync::{mpsc, Arc, Mutex, atomic::AtomicBool};
use std::rc::Rc;
use std::marker::{Send, PhantomData};
use sublime_fuzzy::FuzzySearch;
use std::collections::{HashSet, HashMap};
use std::time::{Duration, SystemTime};
use chrono::prelude::*;

pub enum WorkerResult<Message> {Cancelled, Finished(Message), Working}

pub struct Worker<State, Message> {
    cancel: AtomicBool,
    state: Arc<Mutex<State>>,
    channel: PhantomData<Message>
}

impl<State, Message> Worker<State, Message> where Message: Send + 'static, State: Send + 'static {
    pub fn new (channel: mpsc::Sender<Message>, poll: fn(state: &mut State) -> WorkerResult<Message>, initState:State) -> Worker<State, Message> {
        let state = Arc::new(Mutex::new(initState));
        let cancel = AtomicBool::new(false);

        let worker_ret = Worker {state: Arc::clone(&state), cancel, channel: PhantomData};
        
        thread::spawn(move || {
            loop {
                let res = poll(&mut state.lock().unwrap());
                match res {
                    WorkerResult::Finished(msg) => {
                        channel.send(msg);
                        break
                    },
                    WorkerResult::Working => (),
                    _ => break
                }
            }
        });

        worker_ret
    }

    pub fn cancel(&mut self) {
        let canceller = self.cancel.get_mut();
        *canceller = true;
    }
}

pub type FontHandle = String;

pub struct Font {
    pub name: String,
    pub family: String,
    pub font: FontHandle
}

pub struct FontFamily {
    pub name: String,
    pub fonts: Vec<Font>
}

pub type FontSource = fn() -> Font;
pub struct FolderSource {
    pub name:String,
    pub dir:String
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum FontStyle {Serif, SansSerif, Display, Handwriting, Monospace}

pub struct SavedFont {
    active: bool,
    favorite: bool,
    style: FontStyle,
    date_added: SystemTime,
    popularity: i32
}

pub struct SavedWithFont (pub Arc<SavedFont>, pub Arc<Font>); //saved font index

pub struct CatalogConfig {
    pub saved_fonts: HashMap<Font,SavedFont>,
    pub folder_sources:Vec<FolderSource>,
    pub show_variants:bool,
    pub active_dir:Option<String>
}


#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Flag {Favorite, Active, FontStyle(FontStyle)}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum FontCatalogSource {Google, Folder(i32)}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum FontCatalogFilter {Search(String),  Flag(Flag)}

pub enum FontCatalogSort {Popular, DateAdded, Alphabetical}

pub struct FontCatalogCache {
    pub intermediate: Vec<(SavedWithFont, FontCatalogSource)>,
    //indexer: Option<FuzzySearch<Font>>,
    pub filtered: Vec<(SavedWithFont, FontCatalogSource)>,
    pub sorted: Vec<(SavedWithFont, FontCatalogSource)>
}

pub struct FontCatalog {
    pub config: CatalogConfig,
    pub query: HashSet<FontCatalogSource>,
    pub filters: HashSet<FontCatalogFilter>,
    pub sort: FontCatalogSort,
    
    pub page: i32,
    pub cache: FontCatalogCache,
    pub results: Option<Result<Vec<SavedWithFont>, String>>,

    pub loaders: Vec<Worker<(), CatalogMessage>>
} //TODO: pixel size and preview test

pub struct FontSelection (pub i32);
pub enum CatalogMessage {
    ToggleQuery (FontCatalogSource),
    ToggleFilter (FontCatalogFilter),
    
    ClearFilters,
    
    SetFonts(Result<Vec<SavedWithFont>, String>),

    ToggleFlag(FontSelection, Flag),
    SwitchPage(i32),
    Refresh
}