#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
mod client;

use client::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
