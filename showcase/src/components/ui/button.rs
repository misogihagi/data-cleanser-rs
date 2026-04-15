use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    #[props(default = "default".to_string())]
    pub variant: String,
    #[props(default = "default".to_string())]
    pub size: String,
    #[props(default = String::new())]
    pub class: String,
    pub children: Element,
    pub onclick: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let base_class = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";
    
    let variant_class = match props.variant.as_str() {
        "destructive" => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
        "outline" => "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
        "secondary" => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
        "ghost" => "hover:bg-accent hover:text-accent-foreground",
        "link" => "text-primary underline-offset-4 hover:underline",
        _ => "bg-primary text-primary-foreground hover:bg-primary/90",
    };

    let size_class = match props.size.as_str() {
        "sm" => "h-9 rounded-md px-3",
        "lg" => "h-11 rounded-md px-8",
        "icon" => "h-10 w-10",
        _ => "h-10 px-4 py-2",
    };

    rsx! {
        button {
            class: "{base_class} {variant_class} {size_class} {props.class}",
            onclick: move |e| {
                if let Some(handler) = &props.onclick {
                    handler.call(e);
                }
            },
            {props.children}
        }
    }
}
