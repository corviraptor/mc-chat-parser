mod app;
pub mod parser;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
