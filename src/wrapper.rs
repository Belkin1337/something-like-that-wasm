use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;

fn Wrapper() -> Element {
	rsx! {
		div {
			class: "p-4 w-full h-full",

		}
	}
}