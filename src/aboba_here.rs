use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::{component, use_signal};

#[component]
fn AbobaHere() -> Element {
	rsx! {
		div {
			class: "flex items-center w-full bg-green-200/60 rounded-md p-4",
			h1 { class: "text-black", "Абоба здесь. "}
		}
	}
}