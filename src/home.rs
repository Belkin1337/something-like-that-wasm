use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::hooks::use_signal;
use dioxus::prelude::Key::Link;
use crate::Route;

#[component]
fn Home() -> Element {
	let mut count = use_signal(|| 0);

	rsx! {
		div {
			class: "flex flex-col justify-between min-h-screen w-full",
			Link {
				to: Route::Blog {
					id: count()
				},
				"Go to blog"
			}

			div {
				class: "flex flex-col",
				h1 {
					class: "text-3xl text-red-500",
					"High-Five counter: {count}"
				}
				button {
					onclick: move |_| count += 1,
					"Up high!"
				}
				button {
					onclick: move |_| count -= 1,
					"Down low!"
				}
			}
		}
	}
}