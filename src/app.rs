use log::*;
use rs9cc::asm::code_gen;
use rs9cc::ast::program;
use rs9cc::token::tokenize;
use serde_derive::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};

const KEY: &str = "yew.todomvc.self";

pub struct App {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    entries: Vec<Entry>,
    value: String,
    edit_value: String,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    description: String,
    completed: bool,
    editing: bool,
}

pub enum Msg {
    Update(String),
    UpdateEdit(String),
    Nope,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        let entries = {
            if let Json(Ok(restored_entries)) = storage.restore(KEY) {
                restored_entries
            } else {
                Vec::new()
            }
        };
        let state = State {
            entries,
            value: "".into(),
            edit_value: "".into(),
        };
        App {
            link,
            storage,
            state,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(val) => {
                println!("Input: {}", val);
                self.state.value = val;
            }
            Msg::UpdateEdit(val) => {
                println!("Input: {}", val);
                self.state.edit_value = val;
            }
            Msg::Nope => {}
        }
        self.storage.store(KEY, Json(&self.state.entries));
        true
    }

    fn view(&self) -> Html {
        info!("rendered!");
        html! {
            <div class = "wrapper">
                <div class="input">{self.view_input()}</div>
                <div class = "output">{self.asm()}</div>
            </div>
        }
    }
}

impl App {
    fn asm(&self) -> Html {
        let mut iter = tokenize(&self.state.value, "main.c");
        let generated = match program(&mut iter) {
            Ok(x) => match code_gen(x) {
                Err(err) => {
                    eprintln!("{}", err);
                    panic!()
                }
                Ok(string) => string,
            },
            Err(err) => {
                format!("{}", err)
            }
        };

        html! {
            <textarea class="new-todo"
            placeholder="put yout source code"
            rows="10"
            cols="30"
            value=&generated
            readonly=true
            />
        }
    }

    fn view_input(&self) -> Html {
        html! {
            // You can use standard Rust comments. One line:
            // <li></li>
            <textarea class="new-todo"
                   placeholder="put yout source code"
                   rows="10"
                   cols="30"
                   value=&self.state.value
                   oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
                    />
            /* Or multiline:
            <ul>
                <li></li>
            </ul>
            */
        }
    }
}
