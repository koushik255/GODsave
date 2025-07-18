use dioxus::prelude::*;

#[component]
pub fn Input() -> Element {
    let mut input = use_signal(|| "bob".to_string());
    let mut link = use_signal(|| "".to_string());
    let mut enter = use_signal(|| "".to_string());
    let mut enter_link = use_signal(|| "".to_string());

    let submit = move || {
        let process = input.read().clone();
        if !process.trim().is_empty() {
            spawn(async move {
                match api::input(process).await {
                    Ok(_) => println!("wtfr"),
                    Err(e) => println!("Error: {:?}", e),
                }
            });
        }
    };

    let submit_link = move || {
        let process = link.read().clone();
        if !process.trim().is_empty() {
            spawn(async move {
                match api::link(process).await {
                    Ok(_) => println!("wtfr"),
                    Err(e) => println!("Error: {:?}", e),
                }
            });
        }
    };

    let send_to_both = move || {
        let inp = input.read().clone();
        let lnk = link.read().clone();

        spawn(async move {
            let whole_result = api::whole(inp, lnk).await;

            match whole_result {
                Ok(s) => match api::save(s).await {
                    Ok(_) => println!("Successfully saved!"),
                    Err(e) => println!("Error saving to DB: {:?}", e),
                },
                Err(e) => {
                    println!("Error calling 'whole' function: {:?}", e);
                }
            }
        });
    };
    rsx! {
        input {
            value: "{input}",
            oninput: move |event| input.set(event.value())
        }

        input {
            value: "{link}",
            oninput: move |event| link.set(event.value())
        }

        button {
            onclick: move |_| {
                submit();
                submit_link();
                enter.set(input.to_string());
                enter_link.set(link.to_string());
                send_to_both();



            },
            "Submit"
        }

        h1 { "{enter}  {enter_link}" }
    }
}
