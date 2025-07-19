use crate::Route;
use dioxus::prelude::*;
use ui::{Blud, Input, List};

#[component]
pub fn Shik() -> Element {
    rsx! {
    List{}
     Blud{}
     Input{}
         // space question mark for helix:

        div {
            id: "about-link",
            class: "text-center mt-6",
            Link {
                to: Route::Home {},
                class: "inline-block px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition",
                "Home"
            }

        }
    }
}
