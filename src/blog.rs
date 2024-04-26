use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::Key::Link;
use crate::Route;

#[component]
fn Blog(id: i32) -> Element {
	rsx! {
		Link { to: Route::Home {}, "Go to counter" }
		"Blog post {id}"
	}
}