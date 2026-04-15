use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct CardProps {
    #[props(default = String::new())]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    rsx! {
        div {
            class: "rounded-xl border bg-card text-card-foreground shadow {props.class}",
            {props.children}
        }
    }
}

#[component]
pub fn CardHeader(props: CardProps) -> Element {
    rsx! {
        div {
            class: "flex flex-col space-y-1.5 p-6 {props.class}",
            {props.children}
        }
    }
}

#[component]
pub fn CardTitle(props: CardProps) -> Element {
    rsx! {
        h3 {
            class: "font-semibold leading-none tracking-tight {props.class}",
            {props.children}
        }
    }
}

#[component]
pub fn CardDescription(props: CardProps) -> Element {
    rsx! {
        p {
            class: "text-sm text-muted-foreground {props.class}",
            {props.children}
        }
    }
}

#[component]
pub fn CardContent(props: CardProps) -> Element {
    rsx! {
        div {
            class: "p-6 pt-0 {props.class}",
            {props.children}
        }
    }
}
