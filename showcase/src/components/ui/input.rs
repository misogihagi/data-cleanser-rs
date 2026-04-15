use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct InputProps {
    #[props(default = "text".to_string())]
    pub r#type: String,
    #[props(default = "")]
    pub placeholder: String,
    #[props(default = "")]
    pub value: String,
    pub oninput: Option<EventHandler<Event<FormData>>>,
    #[props(default = "")]
    pub class: String,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    rsx! {
        input {
            r#type: "{props.r#type}",
            class: "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 {props.class}",
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
