use dioxus::prelude::*;
use dioxus_toast::{ToastInfo, ToastManager};

fn main() {
    launch(app)
}

fn app() -> Element {
    std::panic::set_hook(Box::new(|info| {
        println!("Panic: {}", info);
    }));

    let mut toast = use_signal(|| ToastManager::default());

    rsx! {
        dioxus_toast::ToastFrame { manager: toast }
        div {
            button {
                onclick: move |_| {
                    // let _id = toast.write().popup(ToastInfo {
                    //     heading:Some("Hello Dioxus".into()),
                    //     context:"hello world: <a href=\"https://dioxuslabs.com/\">Dioxus</a>".into(),
                    //     allow_toast_close:true,
                    //     position:dioxus_toast::Position::BottomLeft,
                    //     icon: None,
                    //     hide_after: Some(5),
                    // });
                    let _id = toast.write().popup(ToastInfo::simple("hello world"));
                    println!("New Toast ID: {}", _id);
                },
                "Normal Toast"
            }
            button {
                onclick: move |_| {
                    let _id = toast.write().popup(ToastInfo::success("Hello World!", "Success"));
                    println!("New Toast ID: {}", _id);
                },
                "Success Toast"
            }
            button {
                onclick: move |_| {
                    let _id = toast
                        .write()
                        .popup(ToastInfo {
                            heading: Some("top-right".into()),
                            context: "Top Right Toast".into(),
                            allow_toast_close: true,
                            position: dioxus_toast::Position::TopRight,
                            icon: None,
                            hide_after: None,
                        });
                },
                "Top Right"
            }
        }
    }
}
