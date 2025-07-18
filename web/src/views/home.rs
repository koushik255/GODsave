use crate::Route;
use dioxus::prelude::*;
use ui::Hero;

#[component]
pub fn Home() -> Element {
    rsx! {
    Hero {}
            div {
            id: "about-link",
            class: "text-center mt-6",
            Link {
                to: Route::Shik {},
                class: "inline-block px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition",
                "Shik"
            }



            }
        }
}
