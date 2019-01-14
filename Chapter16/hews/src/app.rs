// hews/src/hackernews.rs

use crate::hackernews::top_stories;
use crate::hackernews::Story;
use gtk::*;
use reqwest::Client;
use std::process;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;

pub enum Msg {
    NewStory(Story),
    Loading,
    Loaded,
    Refresh,
}

#[derive(Clone)]
pub struct Header {
    pub header: HeaderBar,
    pub refresh_btn: Button,
}

impl Header {
    pub fn new(story_container: gtk::Box, tx: Sender<Msg>) -> Header {
        let header = HeaderBar::new();
        let refresh_btn = gtk::Button::new_with_label("Refresh");
        refresh_btn.set_sensitive(false);
        header.pack_start(&refresh_btn);
        header.set_title("Hews - popular stories from hacker news");
        header.set_show_close_button(true);

        refresh_btn.connect_clicked(move |_| {
            for i in story_container.get_children().iter().skip(1) {
                story_container.remove(i);
            }
            tx.send(Msg::Refresh).unwrap();
        });

        Header {
            header,
            refresh_btn,
        }
    }

    fn disable_refresh(&self) {
        self.refresh_btn.set_label("Loading");
        self.refresh_btn.set_sensitive(false);
    }

    fn enable_refresh(&self) {
        self.refresh_btn.set_label("Refresh");
        self.refresh_btn.set_sensitive(true);
    }
}

pub struct App {
    window: Window,
    header: Header,
    stories: gtk::Box,
    spinner: Spinner,
    tx: Sender<Msg>,
}

impl App {
    pub fn new() -> (App, Receiver<Msg>) {
        if gtk::init().is_err() {
            println!("Failed to init hews window");
            process::exit(1);
        }

        let (tx, rx) = channel();
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        let sw = ScrolledWindow::new(None, None);
        let stories = gtk::Box::new(gtk::Orientation::Vertical, 20);
        let spinner = gtk::Spinner::new();
        let header = Header::new(stories.clone(), tx.clone());

        stories.pack_start(&spinner, false, false, 2);
        sw.add(&stories);
        window.add(&sw);
        window.set_default_size(600, 350);
        window.set_titlebar(&header.header);

        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        (
            App {
                window,
                header,
                stories,
                spinner,
                tx,
            },
            rx,
        )
    }

    pub fn launch(&self, rx: Receiver<Msg>) {
        self.window.show_all();
        let client = Arc::new(reqwest::Client::new());
        self.fetch_posts(client.clone());
        self.run_event_loop(rx, client);
    }

    fn fetch_posts(&self, client: Arc<Client>) {
        self.spinner.start();
        self.tx.send(Msg::Loading).unwrap();
        let tx_clone = self.tx.clone();
        top_stories(client, 10, &tx_clone);
    }

    fn render_story(s: Story, stories: &gtk::Box) {
        let title_with_score = format!("{} ({})", s.title, s.score);
        let label = gtk::Label::new(&*title_with_score);
        let story_url = s.url.unwrap_or("N/A".to_string());
        let link_label = gtk::Label::new(&*story_url);
        let label_markup = format!("<a href=\"{}\">{}</a>", story_url, story_url);
        link_label.set_markup(&label_markup);
        stories.pack_start(&label, false, false, 2);
        stories.pack_start(&link_label, false, false, 2);
        stories.show_all();
    }

    fn run_event_loop(&self, rx: Receiver<Msg>, client: Arc<Client>) {
        let container = self.stories.clone();
        let spinner = self.spinner.clone();
        let header = self.header.clone();
        let tx_clone = self.tx.clone();

        gtk::timeout_add(100, move || {
            match rx.try_recv() {
                Ok(Msg::NewStory(s)) => App::render_story(s, &container),
                Ok(Msg::Loading) => header.disable_refresh(),
                Ok(Msg::Loaded) => {
                    spinner.stop();
                    header.enable_refresh();
                }
                Ok(Msg::Refresh) => {
                    spinner.start();
                    spinner.show();
                    (&tx_clone).send(Msg::Loading).unwrap();
                    top_stories(client.clone(), 10, &tx_clone);
                }
                Err(_) => {}
            }
            gtk::Continue(true)
        });

        gtk::main();
    }
}
