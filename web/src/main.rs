use dioxus::prelude::*;

use ui::Navbar;
use views::{Home, Shik};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Home {},
    #[route("/shik")]
    Shik {},
    }

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {

        Router::<Route> {}
    }
}
#[component]
fn WebNavbar() -> Element {
    rsx! {
        Navbar {
            Link {
                to: Route::Home {},
                "Home"
            }
                    }

        Outlet::<Route> {}
    }
}
