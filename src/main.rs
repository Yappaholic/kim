mod buffer;
mod cli;
mod editor;
mod terminal;
mod view;

fn main() {
  editor::Editor::default()
    .run(cli::parse_args())
    .expect("Editor failed to run. Good luck!")
}
