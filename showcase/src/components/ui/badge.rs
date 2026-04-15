use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct BadgeProps {
    #[props(default = "default".to_string())]
    pub variant: String,
    #[props(default = "")]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let base_class = "inline-flex items-center rounded-md border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2";
    let variant_class = match props.variant.as_str() {
        "secondary" => "border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80",
        "destructive" => "border-transparent bg-destructive text-destructive-foreground hover:bg-destructive/80",
        "outline" => "text-foreground",
        _ => "border-transparent bg-primary text-primary-foreground hover:bg-primary/80",
    };

    rsx! {
        div {
            class: "{base_class} {variant_class} {props.class}",
            {props.children}
        }
    }
}
