use dioxus::prelude::*;

#[component]
pub fn Blud() -> Element {
    let printthis = use_resource(|| api::name());
    let devil = use_resource(|| api::df());

    rsx! {
        div {
            match &*printthis.read_unchecked() {
                Some(Ok(data)) => {
                    rsx! {
                        div {
                            h2 { "{data}" }
                        }
                    }
                },
                Some(Err(err)) => {
                    rsx! {
                        p { "Error: {err}" }
                    }
                },
                None => {
                    rsx! {
                        p { "Loading....." }
                    }
                }
            }

            div {
                match &*devil.read_unchecked() {
                    Some(Ok(data)) => {
                        rsx! {
                            div {
                                h2 { "{data:?}" }
                            }
                        }
                    },
                    Some(Err(err)) => {
                        rsx! {
                            p { "Error: {err}" }
                        }
                    },
                    None => {
                        rsx! {
                            p { "Loading....." }
                        }
                    }
                }
            }
        }
    }
}
