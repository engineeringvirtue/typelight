extern crate sciter;
extern crate chrono;
extern crate sublime_fuzzy;
extern crate edit_distance;
#[macro_use]
extern crate pipeline;

use std::sync::mpsc;
use std::thread;
use std::fs;
use sciter::{host, window::Options, utf::w2s, Value, HELEMENT, EventHandler, HostHandler};

mod state;
mod utility;
mod catalog;

struct Model {
    font_catalog: state::FontCatalog
}

struct MainEventHandler {}
struct MainHostHandler<'a> {
    assets: &'a sciter::Archive
}

impl<'a> HostHandler for MainHostHandler<'a> {
    fn on_data_load(&mut self, pnm: &mut host::SCN_LOAD_DATA) -> Option<host::LOAD_RESULT> {
        let uri = &w2s(pnm.uri);
    
        let _filepath = uri["file://".to_string().len()..].to_owned();

        self.assets.get(&_filepath).iter().for_each(|data| self.data_ready(pnm.hwnd, uri, data, Some(pnm.request_id)));
        pipe!(host::LOAD_RESULT::LOAD_DEFAULT => Some)
    }
}

impl EventHandler for MainEventHandler {
    fn on_script_call(&mut self, _root: HELEMENT, name: &str, _args: &[Value]) -> Option<Value> {
        println!("SCRIPT CALL! name: {}", name);
        None
    }
}

impl MainEventHandler {
    fn new () -> MainEventHandler {MainEventHandler {}}
}

fn main() {
    let mut win = sciter::Window::new();

    let archive = include_bytes!("./resources.rc");
    let opts = vec![Options::DebugMode (true), Options::TransparentWindow(true), Options::AlphaWindow(true)];
    opts.iter().for_each(|opt| win.set_options(opt.clone()).expect("Error setting debugmode!"));
    let _assets = host::Archive::open(archive).expect("Error loading archive!");

    win.set_title("Typelight");

    win.event_handler(MainEventHandler::new());

    let main = _assets.get("main.html").unwrap();
    win.sciter_handler(MainHostHandler {assets: &_assets});
    
    win.load_html(main, None);
    win.run_app();
}
