use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SeparatorProps {
    #[props(default = "horizontal".to_string())]
    pub orientation: String,
    #[props(default = "")]
    pub class: String,
}

#[component]
pub fn Separator(props: SeparatorProps) -> Element {
    let orientation_class = if props.orientation == "horizontal" {
        "h-[1px] w-full"
    } else {
        "h-full w-[1px]"
    };
    
    rsx! {
        div {
            class: "shrink-0 bg-border {orientation_class} {props.class}",
        }
    }
}
