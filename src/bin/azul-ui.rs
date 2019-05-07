#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate azul;

use azul::prelude::*;

struct List {
    items: Vec<&'static str>,
    selected: Option<usize>,
}

const CUSTOM_CSS: &str = ".selected { background-color: black; color: white; }";

impl Layout for List {
    fn layout(&self, _: LayoutInfo<Self>) -> Dom<Self> {
        self.items.iter().enumerate().map(|(idx, item)| {
            NodeData {
                node_type: NodeType::Label(DomString::Static(item)),
                classes: if self.selected == Some(idx) { vec!["selected".into()] } else { vec![] },
                callbacks: vec![(On::MouseDown.into(), Callback(print_which_item_was_selected))],
                .. Default::default()
            }
        }).collect::<Dom<Self>>()
    }
}

fn print_which_item_was_selected(app_state: &mut AppState<List>, event: &mut CallbackInfo<List>) -> UpdateScreen {

    let selected = event.target_index_in_parent();

    let mut state = app_state.data.lock().ok()?;
    let should_redraw = if selected != state.selected {
        state.selected = selected;
        Redraw
    } else {
        DontRedraw
    };

    println!("selected item: {:?}", state.selected);

    should_redraw
}

fn main() {
    let data = List {
        items: vec![
            "Hello",
            "World",
            "my",
            "name",
            "is",
            "Lorem",
            "Ipsum",
        ],
        selected: None,
    };

    let mut app = App::new(data, AppConfig::default()).unwrap();
    let css = css::override_native(CUSTOM_CSS).unwrap();
    let window = app.create_window(WindowCreateOptions::default(), css).unwrap();
    app.run(window).unwrap();
}
