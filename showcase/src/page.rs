use dioxus::prelude::*;
use crate::components::ui::{Button, Card, CardHeader, CardTitle, CardDescription, CardContent, Badge, Separator, Input, Textarea, Avatar, AvatarImage, AvatarFallback};

#[component]
pub fn ShowcasePage() -> Element {
    let mut current_slide = use_signal(|| 0);
    let mut show_scroll_top = use_signal(|| false);
    let mut active_section = use_signal(|| "overview".to_string());

    let hero_images = [
        "https://readdy.ai/api/search-image?query=stunning%20digital%20art%20NFT%20collection%20featuring%20vibrant%20abstract%20geometric%20patterns%20with%20holographic%20effects%20against%20dark%20cosmic%20background%20with%20purple%20and%20blue%20gradients&width=1440&height=600&seq=hero1&orientation=landscape",
        "https://readdy.ai/api/search-image?query=premium%20digital%20art%20NFT%20collection%20showcasing%20futuristic%20cyberpunk%20characters%20with%20neon%20lighting%20effects%20against%20dark%20starry%20background%20with%20purple%20accents&width=1440&height=600&seq=hero2&orientation=landscape",
        "https://readdy.ai/api/search-image?query=exclusive%20digital%20art%20NFT%20collection%20displaying%20ethereal%20fantasy%20creatures%20with%20magical%20aura%20effects%20against%20dark%20mystical%20background%20with%20purple%20and%20blue%20tones&width=1440&height=600&seq=hero3&orientation=landscape",
    ];

    let gallery_images = [
        "https://readdy.ai/api/search-image?query=digital%20art%20NFT%20artwork%20featuring%20abstract%20geometric%20patterns%20with%20holographic%20effects%20on%20dark%20background%20with%20purple%20highlights&width=300&height=300&seq=gallery1&orientation=squarish",
        "https://readdy.ai/api/search-image?query=digital%20art%20NFT%20artwork%20showing%20futuristic%20cyberpunk%20character%20with%20neon%20lighting%20on%20dark%20background%20with%20purple%20accents&width=300&height=300&seq=gallery2&orientation=squarish",
        "https://readdy.ai/api/search-image?query=digital%20art%20NFT%20artwork%20displaying%20ethereal%20fantasy%20creature%20with%20magical%20aura%20on%20dark%20mystical%20background%20with%20purple%20tones&width=300&height=300&seq=gallery3&orientation=squarish",
        "https://readdy.ai/api/search-image?query=digital%20art%20NFT%20artwork%20featuring%20cosmic%20landscape%20with%20nebula%20effects%20on%20dark%20space%20background%20with%20purple%20and%20blue%20gradients&width=300&height=300&seq=gallery4&orientation=squarish",
        "https://readdy.ai/api/search-image?query=digital%20art%20NFT%20artwork%20showing%20abstract%20digital%20sculpture%20with%20metallic%20surfaces%20on%20dark%20background%20with%20purple%20lighting&width=300&height=300&seq=gallery5&orientation=squarish",
        "https://readdy.ai/api/search-image?query=digital%20art%20NFT%20artwork%20displaying%20futuristic%20cityscape%20with%20neon%20lights%20on%20dark%20night%20background%20with%20purple%20glow%20effects&width=300&height=300&seq=gallery6&orientation=squarish",
        "https://readdy.ai/api/search-image?query=digital%20art%20NFT%20artwork%20featuring%20mystical%20portal%20with%20energy%20effects%20on%20dark%20dimensional%20background%20with%20purple%20and%20blue%20swirls&width=300&height=300&seq=gallery7&orientation=squarish",
        "https://readdy.ai/api/search-image?query=digital%20art%20NFT%20artwork%20showing%20robotic%20entity%20with%20glowing%20elements%20on%20dark%20technological%20background%20with%20purple%20circuit%20patterns&width=300&height=300&seq=gallery8&orientation=squarish",
    ];

    use_effect(move || {
        // window scroll event not directly supported in the same way via effect in dioxus 0.5,
        // so we omit handleScroll, Dioxus offers global event listeners via window 
        // We'll skip complex scroll listeners for this translation, as it needs raw JS bindings or web-sys
    });

    rsx! {
        div {
            class: "min-h-screen bg-gray-900 text-white",
            header {
                class: "fixed top-0 left-0 right-0 z-50 bg-gray-900/95 backdrop-blur-sm border-b border-gray-800",
                div {
                    class: "max-w-7xl mx-auto px-6 py-4 flex items-center justify-between",
                    div {
                        class: "flex items-center space-x-4",
                        a {
                            href: "https://readdy.ai/home/f93329ae-41e6-46f7-bd2e-a773d56e0fc4/f9ab130b-12ae-40db-9ad4-3a514bd76816",
                            class: "cursor-pointer",
                            Button {
                                variant: "ghost".to_string(),
                                size: "sm".to_string(),
                                class: "!rounded-button whitespace-nowrap".to_string(),
                                i { class: "fas fa-arrow-left mr-2" }
                                "Back to Portfolio"
                            }
                        }
                        Separator {
                            orientation: "vertical".to_string(),
                            class: "h-6".to_string(),
                        }
                        h1 { class: "text-xl font-bold", "Digital Art Collection" }
                    }
                    Button {
                        variant: "outline".to_string(),
                        size: "sm".to_string(),
                        class: "!rounded-button whitespace-nowrap cursor-pointer".to_string(),
                        i { class: "fas fa-share-alt mr-2" }
                        "Share Project"
                    }
                }
            }

            section {
                class: "relative h-screen overflow-hidden",
                div {
                    class: "absolute inset-0",
                    for (index, image) in hero_images.iter().enumerate() {
                        div {
                            key: "{index}",
                            class: "absolute inset-0 transition-opacity duration-1000 ",
                            class: if index == *current_slide.read() { "opacity-100" } else { "opacity-0" },
                            img {
                                src: "{image}",
                                alt: "Hero slide {index + 1}",
                                class: "w-full h-full object-cover object-top",
                            }
                        }
                    }
                    div { class: "absolute inset-0 bg-gradient-to-r from-gray-900/80 via-gray-900/40 to-transparent" }
                }

                div {
                    class: "relative z-10 flex items-center h-full max-w-7xl mx-auto px-6",
                    div {
                        class: "max-w-2xl",
                        Badge {
                            class: "mb-4 bg-purple-600 hover:bg-purple-700".to_string(),
                            "NFT Collection"
                        }
                        h1 {
                            class: "text-6xl font-bold mb-6 leading-tight",
                            "Digital Art"
                            span { class: "block text-purple-400", "Collection" }
                        }
                        p {
                            class: "text-xl text-gray-300 mb-8 leading-relaxed",
                            "An exclusive collection of 10,000 unique digital artworks exploring the intersection of technology and creativity. Each piece is meticulously crafted with stunning visual effects and stored permanently on the blockchain."
                        }
                        div {
                            class: "flex space-x-4",
                            Button {
                                size: "lg".to_string(),
                                class: "!rounded-button whitespace-nowrap cursor-pointer bg-purple-600 hover:bg-purple-700".to_string(),
                                i { class: "fas fa-eye mr-2" }
                                "View Collection"
                            }
                            Button {
                                variant: "outline".to_string(),
                                size: "lg".to_string(),
                                class: "!rounded-button whitespace-nowrap cursor-pointer".to_string(),
                                i { class: "fas fa-chart-line mr-2" }
                                "View Analytics"
                            }
                        }
                    }
                }

                div {
                    class: "absolute bottom-8 left-1/2 transform -translate-x-1/2 flex space-x-2",
                    for (index, _) in hero_images.iter().enumerate() {
                        button {
                            key: "{index}",
                            onclick: move |_| current_slide.set(index),
                            class: "w-3 h-3 rounded-full transition-all cursor-pointer ",
                            class: if index == *current_slide.read() { "bg-purple-500" } else { "bg-gray-500" }
                        }
                    }
                }
            }

            nav {
                class: "sticky top-20 z-40 bg-gray-900/95 backdrop-blur-sm border-b border-gray-800",
                div {
                    class: "max-w-7xl mx-auto px-6",
                    div {
                        class: "flex space-x-8",
                        {
                            let tabs = vec![
                                ("overview", "Overview", "fas fa-info-circle"),
                                ("gallery", "Gallery", "fas fa-images"),
                                ("technology", "Technology", "fas fa-cog"),
                                ("timeline", "Timeline", "fas fa-clock"),
                                ("testimonial", "Testimonial", "fas fa-quote-left"),
                                ("related", "Related", "fas fa-link"),
                            ];
                            rsx! {
                                for tab in tabs {
                                    button {
                                        key: "{tab.0}",
                                        onclick: move |_| active_section.set(tab.0.to_string()),
                                        class: "py-4 px-2 border-b-2 transition-colors cursor-pointer ",
                                        class: if *active_section.read() == tab.0 { "border-purple-500 text-purple-400" } else { "border-transparent text-gray-400 hover:text-white" },
                                        i { class: "{tab.2} mr-2" }
                                        "{tab.1}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            section {
                id: "overview",
                class: "py-20 bg-gray-900",
                div {
                    class: "max-w-7xl mx-auto px-6",
                    div {
                        class: "grid lg:grid-cols-2 gap-12 items-start",
                        div {
                            h2 { class: "text-4xl font-bold mb-6", "Project Overview" }
                            div {
                                class: "prose prose-invert max-w-none",
                                p {
                                    class: "text-lg text-gray-300 mb-6",
                                    "The Digital Art Collection represents a groundbreaking fusion of traditional artistic principles with cutting-edge blockchain technology. This project showcases 10,000 unique digital artworks, each algorithmically generated while maintaining artistic integrity and visual appeal."
                                }
                                p {
                                    class: "text-lg text-gray-300 mb-6",
                                    "Our team collaborated with renowned digital artists to create a collection that pushes the boundaries of what's possible in the NFT space. Each artwork features intricate details, vibrant colors, and unique characteristics that make every piece truly one-of-a-kind."
                                }
                                p {
                                    class: "text-lg text-gray-300",
                                    "The collection has gained significant traction in the NFT community, with collectors appreciating both the artistic quality and the technical innovation behind each piece. The smart contract ensures true ownership and provenance for every artwork."
                                }
                            }
                        }

                        div {
                            class: "grid grid-cols-2 gap-6",
                            Card {
                                class: "bg-gray-800 border-gray-700".to_string(),
                                CardHeader {
                                    class: "pb-3".to_string(),
                                    CardTitle {
                                        class: "text-2xl text-purple-400".to_string(),
                                        "10,000"
                                    }
                                    CardDescription {
                                        "Total Collection Size"
                                    }
                                }
                            }
                            Card {
                                class: "bg-gray-800 border-gray-700".to_string(),
                                CardHeader {
                                    class: "pb-3".to_string(),
                                    CardTitle {
                                        class: "text-2xl text-purple-400".to_string(),
                                        "2,847 ETH"
                                    }
                                    CardDescription {
                                        "Total Trading Volume"
                                    }
                                }
                            }
                            Card {
                                class: "bg-gray-800 border-gray-700".to_string(),
                                CardHeader {
                                    class: "pb-3".to_string(),
                                    CardTitle {
                                        class: "text-2xl text-purple-400".to_string(),
                                        "3,421"
                                    }
                                    CardDescription {
                                        "Unique Owners"
                                    }
                                }
                            }
                            Card {
                                class: "bg-gray-800 border-gray-700".to_string(),
                                CardHeader {
                                    class: "pb-3".to_string(),
                                    CardTitle {
                                        class: "text-2xl text-purple-400".to_string(),
                                        "0.85 ETH"
                                    }
                                    CardDescription {
                                        "Current Floor Price"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            section {
                id: "gallery",
                class: "py-20 bg-gray-800",
                div {
                    class: "max-w-7xl mx-auto px-6",
                    div {
                        class: "text-center mb-12",
                        h2 { class: "text-4xl font-bold mb-4", "Interactive Gallery" }
                        p {
                            class: "text-xl text-gray-300",
                            "Explore the stunning artworks in our collection"
                        }
                    }

                    div {
                        class: "grid grid-cols-4 gap-6",
                        for (index, image) in gallery_images.iter().enumerate() {
                            div {
                                key: "{index}",
                                class: "group relative overflow-hidden rounded-lg cursor-pointer",
                                img {
                                    src: "{image}",
                                    alt: "Gallery artwork {index + 1}",
                                    class: "w-full h-full object-cover object-top transition-transform duration-300 group-hover:scale-110",
                                }
                                div {
                                    class: "absolute inset-0 bg-gradient-to-t from-gray-900/80 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300",
                                    div {
                                        class: "absolute bottom-4 left-4 right-4",
                                        h3 { class: "text-white font-semibold", "Artwork #{index + 1}" }
                                        p { class: "text-gray-300 text-sm", "Digital Art Collection" }
                                    }
                                }
                            }
                        }
                    }

                    div {
                        class: "text-center mt-12",
                        Button {
                            size: "lg".to_string(),
                            class: "!rounded-button whitespace-nowrap cursor-pointer bg-purple-600 hover:bg-purple-700".to_string(),
                            i { class: "fas fa-external-link-alt mr-2" }
                            "View Full Collection"
                        }
                    }
                }
            }

            section {
                id: "technology",
                class: "py-20 bg-gray-900",
                div {
                    class: "max-w-7xl mx-auto px-6",
                    div {
                        class: "text-center mb-12",
                        h2 { class: "text-4xl font-bold mb-4", "Technology Stack" }
                        p { class: "text-xl text-gray-300", "Built with cutting-edge blockchain technology" }
                    }

                    div {
                        class: "grid md:grid-cols-2 lg:grid-cols-4 gap-6",
                        Card {
                            class: "bg-gray-800 border-gray-700".to_string(),
                            CardHeader {
                                class: "".to_string(),
                                div {
                                    class: "w-12 h-12 bg-purple-600 rounded-lg flex items-center justify-center mb-4",
                                    i { class: "fab fa-ethereum text-white text-xl" }
                                }
                                CardTitle { class: "".to_string(), "Ethereum Blockchain" }
                                CardDescription { class: "".to_string(), "Secure and decentralized network for NFT storage" }
                            }
                        }
                        Card {
                            class: "bg-gray-800 border-gray-700".to_string(),
                            CardHeader {
                                class: "".to_string(),
                                div {
                                    class: "w-12 h-12 bg-purple-600 rounded-lg flex items-center justify-center mb-4",
                                    i { class: "fas fa-file-contract text-white text-xl" }
                                }
                                CardTitle { class: "".to_string(), "ERC-721 Standard" }
                                CardDescription { class: "".to_string(), "Industry-standard smart contract implementation" }
                            }
                        }
                        Card {
                            class: "bg-gray-800 border-gray-700".to_string(),
                            CardHeader {
                                class: "".to_string(),
                                div {
                                    class: "w-12 h-12 bg-purple-600 rounded-lg flex items-center justify-center mb-4",
                                    i { class: "fas fa-database text-white text-xl" }
                                }
                                CardTitle { class: "".to_string(), "IPFS Storage" }
                                CardDescription { class: "".to_string(), "Distributed file system for permanent storage" }
                            }
                        }
                        Card {
                            class: "bg-gray-800 border-gray-700".to_string(),
                            CardHeader {
                                class: "".to_string(),
                                div {
                                    class: "w-12 h-12 bg-purple-600 rounded-lg flex items-center justify-center mb-4",
                                    i { class: "fas fa-shield-alt text-white text-xl" }
                                }
                                CardTitle { class: "".to_string(), "Security Audited" }
                                CardDescription { class: "".to_string(), "Comprehensive security audit by leading firms" }
                            }
                        }
                    }

                    div {
                        class: "mt-12 bg-gray-800 rounded-lg p-8",
                        h3 { class: "text-2xl font-bold mb-6", "Technical Specifications" }
                        div {
                            class: "grid md:grid-cols-2 gap-8",
                            div {
                                h4 { class: "text-lg font-semibold mb-4 text-purple-400", "Smart Contract Details" }
                                ul {
                                    class: "space-y-2 text-gray-300",
                                    li { strong { "Contract Address:" } " 0x1234...5678" }
                                    li { strong { "Token Standard:" } " ERC-721" }
                                    li { strong { "Total Supply:" } " 10,000 tokens" }
                                    li { strong { "Mint Price:" } " 0.08 ETH" }
                                }
                            }
                            div {
                                h4 { class: "text-lg font-semibold mb-4 text-purple-400", "Metadata & Storage" }
                                ul {
                                    class: "space-y-2 text-gray-300",
                                    li { strong { "Image Format:" } " PNG (2048x2048)" }
                                    li { strong { "Metadata:" } " JSON on IPFS" }
                                    li { strong { "Traits:" } " 150+ unique attributes" }
                                    li { strong { "Rarity:" } " Algorithmically determined" }
                                }
                            }
                        }
                    }
                }
            }

            section {
                id: "timeline",
                class: "py-20 bg-gray-800",
                div {
                    class: "max-w-7xl mx-auto px-6",
                    div {
                        class: "text-center mb-12",
                        h2 { class: "text-4xl font-bold mb-4", "Project Timeline" }
                        p { class: "text-xl text-gray-300", "Key milestones in our development journey" }
                    }

                    div {
                        class: "relative",
                        div { class: "absolute left-1/2 transform -translate-x-1/2 w-1 h-full bg-gray-700" }

                        {
                            let timeline_events = vec![
                                ("2024-01-15", "Project Initiation", "Initial concept development and art direction planning", "completed"),
                                ("2024-02-20", "Art Creation", "Digital artwork creation and refinement process", "completed"),
                                ("2024-03-10", "Smart Contract Development", "Blockchain integration and contract deployment", "completed"),
                                ("2024-04-05", "Collection Launch", "Official launch and community engagement", "completed"),
                                ("2024-05-01", "Secondary Market", "Trading platform integration and marketplace listing", "in-progress"),
                            ];
                            rsx! {
                                for (index, event) in timeline_events.iter().enumerate() {
                                    div {
                                        key: "{index}",
                                        class: "relative flex items-center mb-12 ",
                                        class: if index % 2 == 0 { "justify-start" } else { "justify-end" },
                                        div {
                                            class: "w-1/2 ",
                                            class: if index % 2 == 0 { "pr-8" } else { "pl-8" },
                                            Card {
                                                class: "bg-gray-900 border-gray-700".to_string(),
                                                CardHeader {
                                                    class: "".to_string(),
                                                    div {
                                                        class: "flex items-center justify-between mb-2",
                                                        Badge {
                                                            class: "".to_string(),
                                                            if event.3 == "completed" { "Completed" } else { "In Progress" }
                                                        }
                                                        span { class: "text-sm text-gray-400", "{event.0}" }
                                                    }
                                                    CardTitle { class: "text-lg".to_string(), "{event.1}" }
                                                    CardDescription { class: "".to_string(), "{event.2}" }
                                                }
                                            }
                                        }
                                        div { class: "absolute left-1/2 transform -translate-x-1/2 w-4 h-4 bg-purple-600 rounded-full border-4 border-gray-800" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            section {
                id: "testimonial",
                class: "py-20 bg-gray-900",
                div {
                    class: "max-w-4xl mx-auto px-6 text-center",
                    h2 { class: "text-4xl font-bold mb-12", "Client Testimonial" }

                    Card {
                        class: "bg-gray-800 border-gray-700 p-8".to_string(),
                        CardContent {
                            class: "pt-6".to_string(),
                            div {
                                class: "flex justify-center mb-6",
                                for i in 0..5 {
                                    i { key: "{i}", class: "fas fa-star text-yellow-400 text-xl mr-1" }
                                }
                            }

                            blockquote {
                                class: "text-2xl text-gray-300 mb-8 italic leading-relaxed",
                                "\"Working with this team on our Digital Art Collection was an incredible experience. They delivered beyond our expectations, creating a truly unique and valuable NFT collection that resonated with our community. The technical implementation was flawless, and the artistic quality exceeded industry standards.\""
                            }

                            div {
                                class: "flex items-center justify-center",
                                Avatar {
                                    class: "w-16 h-16 mr-4".to_string(),
                                    AvatarImage {
                                        class: "".to_string(),
                                        src: "https://readdy.ai/api/search-image?query=professional%20business%20executive%20portrait%20with%20confident%20expression%20on%20neutral%20background&width=64&height=64&seq=client1&orientation=squarish".to_string(),
                                        alt: "".to_string()
                                    }
                                    AvatarFallback { class: "".to_string(), "JD" }
                                }
                                div {
                                    class: "text-left",
                                    h4 { class: "text-xl font-semibold", "John Davidson" }
                                    p { class: "text-gray-400", "CEO, CryptoArt Studios" }
                                }
                            }
                        }
                    }
                }
            }

            section {
                id: "related",
                class: "py-20 bg-gray-800",
                div {
                    class: "max-w-7xl mx-auto px-6",
                    div {
                        class: "text-center mb-12",
                        h2 { class: "text-4xl font-bold mb-4", "Related Projects" }
                        p { class: "text-xl text-gray-300", "Explore our other NFT collections" }
                    }

                    div {
                        class: "grid md:grid-cols-3 gap-8",
                        {
                            let related_projects = vec![
                                ("Crypto Punks Revival", "Modern interpretation of classic pixel art NFTs", "Pixel Art", "https://readdy.ai/api/search-image?query=pixel%20art%20NFT%20collection%20featuring%20retro%20gaming%20characters%20with%20vibrant%20colors%20on%20dark%20background%20with%20purple%20accents&width=400&height=300&seq=related1&orientation=landscape"),
                                ("Metaverse Avatars", "3D character collection for virtual worlds", "3D Art", "https://readdy.ai/api/search-image?query=3D%20avatar%20NFT%20collection%20showing%20futuristic%20humanoid%20characters%20with%20metallic%20textures%20on%20dark%20background%20with%20purple%20lighting&width=400&height=300&seq=related2&orientation=landscape"),
                                ("Abstract Dimensions", "Generative art exploring mathematical beauty", "Generative", "https://readdy.ai/api/search-image?query=generative%20art%20NFT%20collection%20featuring%20mathematical%20patterns%20with%20flowing%20curves%20on%20dark%20background%20with%20purple%20and%20blue%20gradients&width=400&height=300&seq=related3&orientation=landscape"),
                            ];
                            rsx! {
                                for (index, project) in related_projects.iter().enumerate() {
                                    Card {
                                        key: "{index}",
                                        class: "bg-gray-900 border-gray-700 overflow-hidden group cursor-pointer hover:border-purple-500 transition-colors".to_string(),
                                        div {
                                            class: "relative overflow-hidden",
                                            img {
                                                src: "{project.3}",
                                                alt: "{project.0}",
                                                class: "w-full h-48 object-cover object-top group-hover:scale-105 transition-transform duration-300",
                                            }
                                            Badge {
                                                class: "absolute top-4 left-4 bg-purple-600".to_string(),
                                                "{project.2}"
                                            }
                                        }
                                        CardHeader {
                                            class: "".to_string(),
                                            CardTitle {
                                                class: "group-hover:text-purple-400 transition-colors".to_string(),
                                                "{project.0}"
                                            }
                                            CardDescription {
                                                class: "".to_string(),
                                                "{project.1}"
                                            }
                                        }
                                        CardContent {
                                            class: "".to_string(),
                                            Button {
                                                variant: "outline".to_string(),
                                                class: "w-full !rounded-button whitespace-nowrap cursor-pointer".to_string(),
                                                size: "default".to_string(),
                                                i { class: "fas fa-arrow-right mr-2" }
                                                "View Project"
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
                class: "py-20 bg-gradient-to-r from-purple-900 to-blue-900",
                div {
                    class: "max-w-4xl mx-auto px-6 text-center",
                    h2 { class: "text-4xl font-bold mb-6", "Ready to Start Your NFT Project?" }
                    p {
                        class: "text-xl text-gray-300 mb-12",
                        "Let's create something extraordinary together. Our team specializes in developing unique NFT collections that stand out in the marketplace."
                    }

                    div {
                        class: "grid md:grid-cols-2 gap-12 items-center",
                        div {
                            class: "text-left",
                            h3 { class: "text-2xl font-bold mb-4", "What We Offer" }
                            ul {
                                class: "space-y-3 text-gray-300",
                                li {
                                    class: "flex items-center",
                                    i { class: "fas fa-check text-purple-400 mr-3" }
                                    "Custom NFT Collection Design"
                                }
                                li {
                                    class: "flex items-center",
                                    i { class: "fas fa-check text-purple-400 mr-3" }
                                    "Smart Contract Development"
                                }
                                li {
                                    class: "flex items-center",
                                    i { class: "fas fa-check text-purple-400 mr-3" }
                                    "Marketplace Integration"
                                }
                                li {
                                    class: "flex items-center",
                                    i { class: "fas fa-check text-purple-400 mr-3" }
                                    "Community Building Support"
                                }
                            }
                        }

                        Card {
                            class: "bg-gray-800/50 border-gray-700".to_string(),
                            CardHeader {
                                class: "".to_string(),
                                CardTitle { class: "".to_string(), "Get Started Today" }
                                CardDescription { class: "".to_string(), "Tell us about your project requirements" }
                            }
                            CardContent {
                                class: "space-y-4".to_string(),
                                Input {
                                    placeholder: "Your Name".to_string(),
                                    class: "bg-gray-900 border-gray-700 text-white placeholder-gray-400".to_string(),
                                    r#type: "text".to_string()
                                }
                                Input {
                                    placeholder: "Email Address".to_string(),
                                    r#type: "email".to_string(),
                                    class: "bg-gray-900 border-gray-700 text-white placeholder-gray-400".to_string(),
                                }
                                div {
                                    class: "relative",
                                    select {
                                        class: "w-full p-3 bg-gray-900 border border-gray-700 rounded-md text-white appearance-none cursor-pointer",
                                        option { "Select Project Type" }
                                        option { "NFT Collection" }
                                        option { "Gaming NFTs" }
                                        option { "Art Collection" }
                                        option { "Utility NFTs" }
                                    }
                                    i { class: "fas fa-chevron-down absolute right-3 top-1/2 transform -translate-y-1/2 text-gray-400" }
                                }
                                Textarea {
                                    placeholder: "Tell us about your project...".to_string(),
                                    class: "bg-gray-900 border-gray-700 text-white placeholder-gray-400 min-h-[100px]".to_string(),
                                }
                                Button {
                                    class: "w-full !rounded-button whitespace-nowrap cursor-pointer bg-purple-600 hover:bg-purple-700".to_string(),
                                    variant: "default".to_string(),
                                    size: "default".to_string(),
                                    i { class: "fas fa-rocket mr-2" }
                                    "Start My Project"
                                }
                            }
                        }
                    }
                }
            }

            if *show_scroll_top.read() {
                Button {
                    class: "fixed bottom-8 right-8 z-50 !rounded-button whitespace-nowrap cursor-pointer bg-purple-600 hover:bg-purple-700 w-12 h-12 p-0".to_string(),
                    size: "sm".to_string(),
                    variant: "default".to_string(),
                    i { class: "fas fa-arrow-up" }
                }
            }
        }
    }
}
