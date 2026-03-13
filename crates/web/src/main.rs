mod app;
mod components;
mod features;
mod hooks;
mod shared;
mod pages;
mod router;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<app::App>::new().render();
}
