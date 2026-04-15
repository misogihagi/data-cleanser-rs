use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct AvatarProps {
    #[props(default = "")]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    rsx! {
        div {
            class: "relative flex h-10 w-10 shrink-0 overflow-hidden rounded-full {props.class}",
            {props.children}
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct AvatarImageProps {
    pub src: String,
    #[props(default = "")]
    pub alt: String,
    #[props(default = "")]
    pub class: String,
}

#[component]
pub fn AvatarImage(props: AvatarImageProps) -> Element {
    rsx! {
        img {
            src: "{props.src}",
            alt: "{props.alt}",
            class: "aspect-square h-full w-full {props.class}"
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct AvatarFallbackProps {
    #[props(default = "")]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn AvatarFallback(props: AvatarFallbackProps) -> Element {
    rsx! {
        div {
            class: "flex h-full w-full items-center justify-center rounded-full bg-muted {props.class}",
            {props.children}
        }
    }
}
