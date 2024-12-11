<div align="center">
  <h1>Dioxus Toast</h1>
  <p></p>
    <div>
    <img src="https://img.shields.io/badge/Dioxus%20Support-0.6-green?style=flat-square&logo=Rust"></img>
   <img src="https://img.shields.io/github/actions/workflow/status/mrxiaozhuox/dioxus-toast/rust.yml?label=Example%20Build&style=flat-square&logo=Github"></img>
  </div> 
  <p></p>
  <strong>Add toast support for your dioxus project.</strong>
  <p></p>
</div>

```rust
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
        dioxus_toast::ToastFrame {
            manager: toast
        }
        div {
            button {
                onclick: move |_| {
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
                    let _id = toast.write().popup(ToastInfo {
                        heading: Some("top-right".into()),
                        context: "Top Right Toast".into(),
                        allow_toast_close: true,
                        position: dioxus_toast::Position::TopRight,
                        icon: None,
                        hide_after: None
                    });
                },
                "Top Right"
            }
        }
    }
}

```

## Use Toast for different component

```rust
use dioxus::prelude::*;

fn main() {
    launch(app)
}

fn app() -> Element {
    let toast = use_context_provider(|| Signal::new(ToastManager::default()));
    rsx! {
        ToastFrame { manager: toast }
        div {
            hello {}
        }
    }
}

#[component]
fn hello() -> Element {
    // use_context can help you pass toast-manager to different components
    let mut toast: Signal<ToastManager> = use_context();
    rsx! {
        button {
            onclick: move |_| {
                let _ = toast.write().popup(ToastInfo::simple("hello world"));
            }
            "Click here!"
        }
    }
}
```
