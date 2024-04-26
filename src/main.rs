#![allow(non_snake_case)]

mod components;
mod aboba_here;
mod blog;
mod home;
mod wrapper;

use dioxus::prelude::*;
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
	#[route("/")]
	Home {},
	#[route("/blog/:id")]
	Blog { id: i32 },
	#[route("/aboba")]
	AbobaHere {}
}

fn App() -> Element {
	rsx! {
		Router::<Route> {}
	}
}

fn main() {
	// Init logger
	dioxus_logger::init(Level::INFO).expect("failed to init logger");
	launch(App);
}