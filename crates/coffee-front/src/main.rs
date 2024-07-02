pub mod api;
pub mod app;
pub mod env;
pub mod routes;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
