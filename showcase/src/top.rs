use dioxus::prelude::*;
use crate::components::ui::{Button, Card, CardContent, Input};

struct PortfolioItem {
    id: i32,
    title: &'static str,
    category: &'static str,
    description: &'static str,
    image: &'static str,
}

#[component]
pub fn TopPage() -> Element {
    let _is_scrolled = use_signal(|| false);
    // In Dioxus, we don't naturally have a global window event listener via hook without web_sys setup.
    // For this generic translation, we assume `is_scrolled` is initialized but not dynamically updated on scroll.

    let portfolio_items = vec![
        PortfolioItem {
            id: 1,
            title: "Digital Art Collection",
            category: "NFT",
            description: "Exclusive digital artwork featuring vibrant colors and abstract compositions",
            image: "https://readdy.ai/api/search-image?query=stunning%20digital%20abstract%20art%20with%20vibrant%20neon%20colors%20and%20geometric%20patterns%20on%20dark%20background%2C%20modern%20artistic%20composition%20with%20glowing%20elements%2C%20high%20quality%20digital%20artwork%2C%20contemporary%20style%20with%20electric%20blue%20and%20purple%20accents&width=400&height=300&seq=portfolio1&orientation=landscape",
        },
        PortfolioItem {
            id: 2,
            title: "Brand Identity Design",
            category: "Branding",
            description: "Complete brand identity package with logo, colors, and typography",
            image: "https://readdy.ai/api/search-image?query=modern%20brand%20identity%20design%20showcase%20with%20elegant%20logo%20concepts%2C%20typography%20samples%2C%20and%20color%20palettes%20on%20dark%20background%2C%20professional%20branding%20materials%20with%20clean%20minimalist%20aesthetic%2C%20corporate%20design%20elements&width=400&height=300&seq=portfolio2&orientation=landscape",
        },
        PortfolioItem {
            id: 3,
            title: "Mobile App Interface",
            category: "UI/UX",
            description: "Intuitive mobile application design with seamless user experience",
            image: "https://readdy.ai/api/search-image?query=sleek%20mobile%20app%20interface%20design%20mockup%20on%20dark%20background%2C%20modern%20smartphone%20screen%20showing%20elegant%20user%20interface%2C%20clean%20app%20design%20with%20purple%20and%20blue%20accents%2C%20professional%20mobile%20UI%20showcase&width=400&height=300&seq=portfolio3&orientation=landscape",
        },
        PortfolioItem {
            id: 4,
            title: "3D Visualization",
            category: "3D Design",
            description: "Photorealistic 3D renders for architectural and product visualization",
            image: "https://readdy.ai/api/search-image?query=impressive%203D%20architectural%20visualization%20with%20modern%20building%20design%20on%20dark%20background%2C%20photorealistic%20rendering%20with%20dramatic%20lighting%2C%20contemporary%20architecture%20with%20glass%20and%20steel%20elements%2C%20professional%203D%20artwork&width=400&height=300&seq=portfolio4&orientation=landscape",
        },
        PortfolioItem {
            id: 5,
            title: "Web Development",
            category: "Development",
            description: "Responsive websites built with cutting-edge technologies",
            image: "https://readdy.ai/api/search-image?query=modern%20web%20development%20showcase%20with%20clean%20code%20editor%20interface%20on%20dark%20background%2C%20responsive%20website%20design%20mockups%2C%20professional%20web%20development%20workspace%20with%20multiple%20screens%2C%20contemporary%20tech%20aesthetic&width=400&height=300&seq=portfolio5&orientation=landscape",
        },
        PortfolioItem {
            id: 6,
            title: "Motion Graphics",
            category: "Animation",
            description: "Dynamic animations and motion graphics for digital media",
            image: "https://readdy.ai/api/search-image?query=dynamic%20motion%20graphics%20design%20with%20flowing%20abstract%20shapes%20and%20particles%20on%20dark%20background%2C%20colorful%20animation%20elements%20with%20neon%20trails%2C%20modern%20digital%20art%20with%20movement%20effects%2C%20vibrant%20motion%20design&width=400&height=300&seq=portfolio6&orientation=landscape",
        },
    ];

    let categories = vec![
        "All",
        "NFT",
        "Branding",
        "UI/UX",
        "3D Design",
        "Development",
        "Animation",
    ];

    let mut active_category = use_signal(|| "All".to_string());

    let filtered_items: Vec<&PortfolioItem> = if active_category.read().as_str() == "All" {
        portfolio_items.iter().collect()
    } else {
        portfolio_items.iter().filter(|item| item.category == active_category.read().as_str()).collect()
    };

    rsx! {
        div {
            class: "min-h-screen bg-gray-900 text-white",
            header {
                class: "fixed top-0 left-0 right-0 z-50 transition-all duration-300 ",
                class: if *_is_scrolled.read() { "bg-gray-900/95 backdrop-blur-sm" } else { "bg-transparent" },
                div {
                    class: "max-w-7xl mx-auto px-6 py-4",
                    nav {
                        class: "flex items-center justify-between",
                        div {
                            class: "text-2xl font-bold text-white",
                            i { class: "fas fa-cube mr-2 text-purple-400" }
                            "CreativeStudio"
                        }
                        div {
                            class: "hidden md:flex items-center space-x-8",
                            a { href: "#home", class: "text-gray-300 hover:text-white transition-colors cursor-pointer", "Home" }
                            a { href: "#portfolio", class: "text-gray-300 hover:text-white transition-colors cursor-pointer", "Portfolio" }
                            a { href: "#about", class: "text-gray-300 hover:text-white transition-colors cursor-pointer", "About" }
                            a { href: "#services", class: "text-gray-300 hover:text-white transition-colors cursor-pointer", "Services" }
                            a { href: "#contact", class: "text-gray-300 hover:text-white transition-colors cursor-pointer", "Contact" }
                        }
                        Button {
                            class: "!rounded-button whitespace-nowrap bg-purple-600 hover:bg-purple-700 text-white cursor-pointer".to_string(),
                            "Get Started"
                        }
                    }
                }
            }

            section {
                id: "home",
                class: "relative min-h-screen flex items-center justify-center overflow-hidden",
                div {
                    class: "absolute inset-0 bg-cover bg-center bg-no-repeat",
                    style: "background-image: url('https://readdy.ai/api/search-image?query=futuristic%20dark%20digital%20workspace%20with%20holographic%20elements%20and%20neon%20purple%20lighting%2C%20modern%20tech%20environment%20with%20floating%20geometric%20shapes%2C%20cyberpunk%20aesthetic%20with%20glowing%20particles%20and%20abstract%20digital%20patterns%2C%20dark%20background%20perfect%20for%20text%20overlay&width=1440&height=800&seq=hero1&orientation=landscape');",
                    div { class: "absolute inset-0 bg-gradient-to-r from-gray-900/90 via-gray-900/70 to-transparent" }
                }
                div {
                    class: "relative z-10 max-w-7xl mx-auto px-6 py-20",
                    div {
                        class: "grid grid-cols-1 lg:grid-cols-2 gap-12 items-center",
                        div {
                            class: "space-y-8",
                            h1 {
                                class: "text-5xl lg:text-7xl font-bold leading-tight",
                                "Creative\n"
                                span {
                                    class: "block text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-pink-400",
                                    "Excellence\n"
                                }
                                "Redefined"
                            }
                            p {
                                class: "text-xl text-gray-300 leading-relaxed max-w-lg",
                                "We craft extraordinary digital experiences that push boundaries and inspire innovation. From concept to creation, we bring your vision to life with cutting-edge design and technology."
                            }
                            div {
                                class: "flex flex-col sm:flex-row gap-4",
                                Button {
                                    class: "!rounded-button whitespace-nowrap bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-700 hover:to-pink-700 text-white px-8 py-3 text-lg cursor-pointer".to_string(),
                                    i { class: "fas fa-rocket mr-2" }
                                    "View Portfolio"
                                }
                                Button {
                                    class: "!rounded-button whitespace-nowrap bg-transparent border-2 border-gray-600 hover:border-purple-400 text-white px-8 py-3 text-lg cursor-pointer".to_string(),
                                    i { class: "fas fa-play mr-2" }
                                    "Watch Demo"
                                }
                            }
                        }
                    }
                }
                div {
                    class: "absolute bottom-8 left-1/2 transform -translate-x-1/2 animate-bounce",
                    i { class: "fas fa-chevron-down text-2xl text-gray-400" }
                }
            }

            section {
                id: "portfolio",
                class: "py-20 bg-gray-800",
                div {
                    class: "max-w-7xl mx-auto px-6",
                    div {
                        class: "text-center mb-16",
                        h2 {
                            class: "text-4xl lg:text-5xl font-bold mb-6",
                            "Featured "
                            span { class: "text-purple-400", "Portfolio" }
                        }
                        p {
                            class: "text-xl text-gray-300 max-w-3xl mx-auto",
                            "Discover our latest projects showcasing innovative design solutions and cutting-edge technology implementations across various industries and creative domains."
                        }
                    }
                    div {
                        class: "flex flex-wrap justify-center gap-4 mb-12",
                        for category in categories {
                            Button {
                                key: "{category}",
                                onclick: move |_| active_category.set(category.to_string()),
                                class: format!("!rounded-button whitespace-nowrap cursor-pointer {}", if active_category.read().as_str() == category { "bg-purple-600 hover:bg-purple-700 text-white" } else { "bg-gray-700 hover:bg-gray-600 text-gray-300" }),
                                "{category}"
                            }
                        }
                    }
                    div {
                        class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
                        for item in filtered_items {
                            Card {
                                key: "{item.id}",
                                class: "bg-gray-700 border-gray-600 hover:bg-gray-600 transition-all duration-300 hover:scale-105 hover:shadow-2xl hover:shadow-purple-500/20 cursor-pointer overflow-hidden".to_string(),
                                div {
                                    class: "relative overflow-hidden",
                                    img {
                                        src: "{item.image}",
                                        alt: "{item.title}",
                                        class: "w-full h-48 object-cover object-top transition-transform duration-300 hover:scale-110",
                                    }
                                    div {
                                        class: "absolute inset-0 bg-gradient-to-t from-gray-900/80 to-transparent opacity-0 hover:opacity-100 transition-opacity duration-300 flex items-end p-4",
                                        Button {
                                            class: "!rounded-button whitespace-nowrap bg-purple-600 hover:bg-purple-700 text-white cursor-pointer".to_string(),
                                            i { class: "fas fa-external-link-alt mr-2" }
                                            "View Project"
                                        }
                                    }
                                }
                                CardContent {
                                    class: "p-6".to_string(),
                                    a {
                                        href: "#",
                                        class: "block",
                                        div {
                                            class: "flex items-center justify-between mb-3",
                                            span {
                                                class: "px-3 py-1 bg-purple-600/20 text-purple-300 text-sm rounded-full",
                                                "{item.category}"
                                            }
                                            i { class: "fas fa-heart text-gray-500 hover:text-red-400 cursor-pointer transition-colors" }
                                        }
                                        h3 {
                                            class: "text-xl font-semibold mb-2 text-white",
                                            "{item.title}"
                                        }
                                        p {
                                            class: "text-gray-400 text-sm leading-relaxed",
                                            "{item.description}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            section {
                id: "about",
                class: "py-20 bg-gray-900",
                div {
                    class: "max-w-7xl mx-auto px-6",
                    div {
                        class: "grid grid-cols-1 lg:grid-cols-2 gap-16 items-center",
                        div {
                            class: "space-y-8",
                            h2 {
                                class: "text-4xl lg:text-5xl font-bold",
                                "About "
                                span { class: "text-purple-400", "Our Studio" }
                            }
                            p {
                                class: "text-lg text-gray-300 leading-relaxed",
                                "We are a collective of passionate designers, developers, and creative minds dedicated to pushing the boundaries of digital innovation. With over a decade of experience, we've helped brands transform their vision into reality through exceptional design and technology."
                            }
                            div {
                                class: "grid grid-cols-2 gap-8",
                                div {
                                    class: "text-center",
                                    div { class: "text-3xl font-bold text-purple-400 mb-2", "150+" }
                                    div { class: "text-gray-400", "Projects Completed" }
                                }
                                div {
                                    class: "text-center",
                                    div { class: "text-3xl font-bold text-purple-400 mb-2", "50+" }
                                    div { class: "text-gray-400", "Happy Clients" }
                                }
                                div {
                                    class: "text-center",
                                    div { class: "text-3xl font-bold text-purple-400 mb-2", "10+" }
                                    div { class: "text-gray-400", "Years Experience" }
                                }
                                div {
                                    class: "text-center",
                                    div { class: "text-3xl font-bold text-purple-400 mb-2", "25+" }
                                    div { class: "text-gray-400", "Awards Won" }
                                }
                            }
                        }
                        div {
                            class: "relative",
                            img {
                                src: "https://readdy.ai/api/search-image?query=modern%20creative%20team%20working%20in%20futuristic%20dark%20office%20space%20with%20multiple%20monitors%20and%20holographic%20displays%2C%20professional%20designers%20and%20developers%20collaborating%20on%20innovative%20projects%2C%20contemporary%20workspace%20with%20purple%20and%20blue%20ambient%20lighting%2C%20high-tech%20creative%20environment&width=600&height=500&seq=about1&orientation=landscape",
                                alt: "About Us",
                                class: "w-full h-full object-cover object-top rounded-lg",
                            }
                            div { class: "absolute inset-0 bg-gradient-to-t from-purple-600/20 to-transparent rounded-lg" }
                        }
                    }
                }
            }

            section {
                id: "services",
                class: "py-20 bg-gray-800",
                div {
                    class: "max-w-7xl mx-auto px-6",
                    div {
                        class: "text-center mb-16",
                        h2 {
                            class: "text-4xl lg:text-5xl font-bold mb-6",
                            "Our "
                            span { class: "text-purple-400", "Services" }
                        }
                        p {
                            class: "text-xl text-gray-300 max-w-3xl mx-auto",
                            "We offer comprehensive creative solutions tailored to meet your unique needs and exceed your expectations in the digital landscape."
                        }
                    }
                    div {
                        class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
                        {
                            let services = vec![
                                ("fas fa-palette", "Brand Design", "Complete brand identity creation including logo design, color schemes, and visual guidelines."),
                                ("fas fa-code", "Web Development", "Custom websites and web applications built with modern technologies and best practices."),
                                ("fas fa-mobile-alt", "Mobile Apps", "Native and cross-platform mobile applications with intuitive user interfaces."),
                                ("fas fa-cube", "3D Modeling", "Photorealistic 3D renders and animations for products, architecture, and visualization."),
                                ("fas fa-video", "Motion Graphics", "Engaging animations and motion graphics for marketing, explainer videos, and presentations."),
                                ("fas fa-chart-line", "Digital Strategy", "Comprehensive digital marketing strategies to enhance your online presence and growth."),
                            ];
                            rsx! {
                                for (index, service) in services.iter().enumerate() {
                                    Card {
                                        key: "{index}",
                                        class: "bg-gray-700 border-gray-600 hover:bg-gray-600 transition-all duration-300 hover:scale-105 cursor-pointer group".to_string(),
                                        CardContent {
                                            class: "p-8 text-center".to_string(),
                                            div {
                                                class: "w-16 h-16 bg-gradient-to-r from-purple-600 to-pink-600 rounded-full flex items-center justify-center mx-auto mb-6 group-hover:scale-110 transition-transform duration-300",
                                                i { class: "{service.0} text-2xl text-white" }
                                            }
                                            h3 {
                                                class: "text-xl font-semibold mb-4 text-white",
                                                "{service.1}"
                                            }
                                            p {
                                                class: "text-gray-400 leading-relaxed",
                                                "{service.2}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            section {
                id: "contact",
                class: "py-20 bg-gray-900",
                div {
                    class: "max-w-7xl mx-auto px-6",
                    div {
                        class: "text-center mb-16",
                        h2 {
                            class: "text-4xl lg:text-5xl font-bold mb-6",
                            "Let's "
                            span { class: "text-purple-400", "Connect" }
                        }
                        p {
                            class: "text-xl text-gray-300 max-w-3xl mx-auto",
                            "Ready to bring your vision to life? Get in touch with us and let's discuss how we can help you achieve your creative goals."
                        }
                    }
                    div {
                        class: "grid grid-cols-1 lg:grid-cols-2 gap-16",
                        div {
                            class: "space-y-8",
                            div {
                                class: "flex items-start space-x-4",
                                div {
                                    class: "w-12 h-12 bg-purple-600 rounded-full flex items-center justify-center flex-shrink-0",
                                    i { class: "fas fa-map-marker-alt text-white" }
                                }
                                div {
                                    h3 { class: "text-xl font-semibold mb-2 text-white", "Visit Our Studio" }
                                    p {
                                        class: "text-gray-400",
                                        "123 Creative Street, Design District\n"
                                        br {}
                                        "New York, NY 10001"
                                    }
                                }
                            }
                            div {
                                class: "flex items-start space-x-4",
                                div {
                                    class: "w-12 h-12 bg-purple-600 rounded-full flex items-center justify-center flex-shrink-0",
                                    i { class: "fas fa-phone text-white" }
                                }
                                div {
                                    h3 { class: "text-xl font-semibold mb-2 text-white", "Call Us" }
                                    p {
                                        class: "text-gray-400",
                                        "+1 (555) 123-4567\n"
                                        br {}
                                        "Mon - Fri, 9AM - 6PM EST"
                                    }
                                }
                            }
                            div {
                                class: "flex items-start space-x-4",
                                div {
                                    class: "w-12 h-12 bg-purple-600 rounded-full flex items-center justify-center flex-shrink-0",
                                    i { class: "fas fa-envelope text-white" }
                                }
                                div {
                                    h3 { class: "text-xl font-semibold mb-2 text-white", "Email Us" }
                                    p {
                                        class: "text-gray-400",
                                        "hello@creativestudio.com\n"
                                        br {}
                                        "We'll respond within 24 hours"
                                    }
                                }
                            }
                        }
                        Card {
                            class: "bg-gray-800 border-gray-700".to_string(),
                            CardContent {
                                class: "p-8".to_string(),
                                form {
                                    class: "space-y-6",
                                    onsubmit: move |e| { e.prevent_default(); },
                                    div {
                                        class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                                        div {
                                            label { class: "block text-sm font-medium text-gray-300 mb-2", "First Name" }
                                            Input {
                                                class: "bg-gray-700 border-gray-600 text-white placeholder-gray-400 text-sm".to_string(),
                                                placeholder: "John".to_string(),
                                            }
                                        }
                                        div {
                                            label { class: "block text-sm font-medium text-gray-300 mb-2", "Last Name" }
                                            Input {
                                                class: "bg-gray-700 border-gray-600 text-white placeholder-gray-400 text-sm".to_string(),
                                                placeholder: "Doe".to_string(),
                                            }
                                        }
                                    }
                                    div {
                                        label { class: "block text-sm font-medium text-gray-300 mb-2", "Email" }
                                        Input {
                                            r#type: "email".to_string(),
                                            class: "bg-gray-700 border-gray-600 text-white placeholder-gray-400 text-sm".to_string(),
                                            placeholder: "john@example.com".to_string(),
                                        }
                                    }
                                    div {
                                        label { class: "block text-sm font-medium text-gray-300 mb-2", "Project Type" }
                                        div {
                                            class: "relative",
                                            select {
                                                class: "w-full bg-gray-700 border border-gray-600 text-white text-sm rounded-md px-3 py-2 appearance-none cursor-pointer",
                                                option { "Brand Design" }
                                                option { "Web Development" }
                                                option { "Mobile App" }
                                                option { "3D Modeling" }
                                                option { "Motion Graphics" }
                                                option { "Other" }
                                            }
                                            i { class: "fas fa-chevron-down absolute right-3 top-1/2 transform -translate-y-1/2 text-gray-400 pointer-events-none" }
                                        }
                                    }
                                    div {
                                        label { class: "block text-sm font-medium text-gray-300 mb-2", "Message" }
                                        textarea {
                                            class: "w-full bg-gray-700 border border-gray-600 text-white placeholder-gray-400 text-sm rounded-md px-3 py-2 h-32 resize-none",
                                            placeholder: "Tell us about your project...",
                                        }
                                    }
                                    Button {
                                        class: "!rounded-button whitespace-nowrap w-full bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-700 hover:to-pink-700 text-white cursor-pointer".to_string(),
                                        i { class: "fas fa-paper-plane mr-2" }
                                        "Send Message"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            footer {
                class: "bg-gray-800 py-16",
                div {
                    class: "max-w-7xl mx-auto px-6",
                    div {
                        class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8 mb-12",
                        div {
                            class: "space-y-4",
                            div {
                                class: "text-2xl font-bold text-white",
                                i { class: "fas fa-cube mr-2 text-purple-400" }
                                "CreativeStudio"
                            }
                            p {
                                class: "text-gray-400 leading-relaxed",
                                "Transforming ideas into extraordinary digital experiences through innovative design and cutting-edge technology."
                            }
                            div {
                                class: "flex space-x-4",
                                a { href: "#", class: "w-10 h-10 bg-gray-700 hover:bg-purple-600 rounded-full flex items-center justify-center transition-colors cursor-pointer", i { class: "fab fa-facebook-f text-white" } }
                                a { href: "#", class: "w-10 h-10 bg-gray-700 hover:bg-purple-600 rounded-full flex items-center justify-center transition-colors cursor-pointer", i { class: "fab fa-twitter text-white" } }
                                a { href: "#", class: "w-10 h-10 bg-gray-700 hover:bg-purple-600 rounded-full flex items-center justify-center transition-colors cursor-pointer", i { class: "fab fa-instagram text-white" } }
                                a { href: "#", class: "w-10 h-10 bg-gray-700 hover:bg-purple-600 rounded-full flex items-center justify-center transition-colors cursor-pointer", i { class: "fab fa-linkedin-in text-white" } }
                            }
                        }
                        div {
                            h3 { class: "text-lg font-semibold text-white mb-4", "Services" }
                            ul {
                                class: "space-y-2",
                                li { a { href: "#", class: "text-gray-400 hover:text-white transition-colors cursor-pointer", "Brand Design" } }
                                li { a { href: "#", class: "text-gray-400 hover:text-white transition-colors cursor-pointer", "Web Development" } }
                                li { a { href: "#", class: "text-gray-400 hover:text-white transition-colors cursor-pointer", "Mobile Apps" } }
                                li { a { href: "#", class: "text-gray-400 hover:text-white transition-colors cursor-pointer", "3D Modeling" } }
                                li { a { href: "#", class: "text-gray-400 hover:text-white transition-colors cursor-pointer", "Motion Graphics" } }
                            }
                        }
                        div {
                            h3 { class: "text-lg font-semibold text-white mb-4", "Company" }
                            ul {
                                class: "space-y-2",
                                li { a { href: "#", class: "text-gray-400 hover:text-white transition-colors cursor-pointer", "About Us" } }
                                li { a { href: "#", class: "text-gray-400 hover:text-white transition-colors cursor-pointer", "Our Team" } }
                                li { a { href: "#", class: "text-gray-400 hover:text-white transition-colors cursor-pointer", "Careers" } }
                                li { a { href: "#", class: "text-gray-400 hover:text-white transition-colors cursor-pointer", "Contact" } }
                                li { a { href: "#", class: "text-gray-400 hover:text-white transition-colors cursor-pointer", "Blog" } }
                            }
                        }
                        div {
                            h3 { class: "text-lg font-semibold text-white mb-4", "Newsletter" }
                            p { class: "text-gray-400 mb-4", "Stay updated with our latest projects and insights." }
                            div {
                                class: "flex",
                                Input {
                                    r#type: "email".to_string(),
                                    placeholder: "Enter your email".to_string(),
                                    class: "bg-gray-700 border-gray-600 text-white placeholder-gray-400 text-sm rounded-r-none".to_string(),
                                }
                                Button {
                                    class: "!rounded-button rounded-l-none bg-purple-600 hover:bg-purple-700 text-white cursor-pointer".to_string(),
                                    i { class: "fas fa-arrow-right" }
                                }
                            }
                        }
                    }
                    div {
                        class: "border-t border-gray-700 pt-8",
                        div {
                            class: "flex flex-col md:flex-row justify-between items-center",
                            p { class: "text-gray-500 text-sm", "© 2025 CreativeStudio. All rights reserved." }
                            div {
                                class: "flex space-x-6 mt-4 md:mt-0",
                                a { href: "#", class: "text-gray-500 hover:text-white text-sm transition-colors cursor-pointer", "Privacy Policy" }
                                a { href: "#", class: "text-gray-500 hover:text-white text-sm transition-colors cursor-pointer", "Terms of Service" }
                                a { href: "#", class: "text-gray-500 hover:text-white text-sm transition-colors cursor-pointer", "Cookie Policy" }
                            }
                        }
                    }
                }
            }
        }
    }
}
