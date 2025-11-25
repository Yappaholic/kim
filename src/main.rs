mod buffer;
mod editor;
mod terminal;
mod view;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Some(path) = args.get(1) {
        editor::Editor::default().run(Some(path)).unwrap()
    } else {
        editor::Editor::default().run(None).unwrap()
    }
}
