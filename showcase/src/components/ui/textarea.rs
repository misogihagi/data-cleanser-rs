use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct TextareaProps {
    #[props(default = "")]
    pub placeholder: String,
    #[props(default = "")]
    pub value: String,
    pub oninput: Option<EventHandler<Event<FormData>>>,
    #[props(default = "")]
    pub class: String,
}

#[component]
pub fn Textarea(props: TextareaProps) -> Element {
    rsx! {
        textarea {
            class: "flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 {props.class}",
            placeholder: "{props.placeholder}",
            value: "{props.value}",
            oninput: move |e| {
                if let Some(handler) = &props.oninput {
                    handler.call(e);
                }
            }
        }
    }
}
