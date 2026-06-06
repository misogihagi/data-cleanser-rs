use crate::constants::DEFAULT_BASE_PATH;
use async_trait::async_trait;
use flate2::read::GzDecoder;
use futures::future::join_all;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::sync::OnceLock;
use std::time::Duration;
use std::vec;
use url::Url;

pub struct HierarchicalFlow<'a> {
    pub index: &'a str,
    pub base: &'a str,
    pub level3_base: &'a str,
    pub level2_base: &'a str,
    pub level1_base: &'a str,
    pub level3_selector: &'a str,
    pub level2_selector: &'a str,
    pub level1_selector: &'a str,
    pub title_selector: &'a str,
    pub body_selector: &'a str,
    pub image_selector: Option<&'a str>,
    pub encoding: &'a str,
    pub level3_links: Vec<String>,
    pub level2_links: Vec<String>,
    pub links: Vec<String>,
    pub pool_size: usize,
    pub rest: u64,
}

impl Default for HierarchicalFlow<'_> {
    fn default() -> HierarchicalFlow<'static> {
        HierarchicalFlow {
            index: "",
            base: "",
            level3_base: "",
            level2_base: "",
            level1_base: "",
            level3_selector: "",
            level2_selector: "",
            level1_selector: "",
            title_selector: "",
            body_selector: "",
            image_selector: None,
            encoding: "utf-8",
            level3_links: vec![],
            level2_links: vec![],
            links: vec![],
            pool_size: POOL_SIZE,
            rest: REST,
        }
    }
}

pub struct SinglepageFlow {
    pub index: &'static str,
    pub base: &'static str,
    pub level1_selector: &'static str,
    pub titles_selector: &'static str,
    pub bodies_selector: &'static str,
    pub encoding: &'static str,
    pub links: Vec<String>,
}
impl Default for SinglepageFlow {
    fn default() -> SinglepageFlow {
        SinglepageFlow {
            index: "",
            base: "",
            level1_selector: "",
            titles_selector: "",
            bodies_selector: "",
            encoding: "utf-8",
            links: vec![],
        }
    }
}

pub struct HeadingRangeFlow {
    pub index: &'static str,
    pub base: &'static str,
    pub level1_selector: &'static str,
    pub container_selector: &'static str,
    pub titles_selector: &'static str,
    pub last_body_selector: &'static str,
    pub encoding: &'static str,
    pub links: Vec<String>,
}
impl Default for HeadingRangeFlow {
    fn default() -> HeadingRangeFlow {
        HeadingRangeFlow {
            index: "",
            base: "",
            level1_selector: "",
            container_selector: "",
            titles_selector: "",
            last_body_selector: "",
            encoding: "utf-8",
            links: vec![],
        }
    }
}

pub struct PageLinkFlow {
    pub index: &'static str,
    pub base: &'static str,
    pub level3_base: &'static str,
    pub level2_base: &'static str,
    pub level1_base: &'static str,
    pub level3_selector: &'static str,
    pub level2_selector: &'static str,
    pub level1_selector: &'static str,
    pub title_selector: &'static str,
    pub body_selector: &'static str,
    pub image_selector: Option<&'static str>,
    pub encoding: &'static str,
    pub level3_links: Vec<String>,
    pub level2_links: Vec<String>,
    pub links: Vec<String>,
    pub pool_size: usize,
    pub rest: u64,
}

impl Default for PageLinkFlow {
    fn default() -> PageLinkFlow {
        PageLinkFlow {
            index: "",
            base: "",
            level3_base: "",
            level2_base: "",
            level1_base: "",
            level3_selector: "",
            level2_selector: "",
            level1_selector: "",
            title_selector: "",
            body_selector: "",
            image_selector: None,
            encoding: "utf-8",
            level3_links: vec![],
            level2_links: vec![],
            links: vec![],
            pool_size: POOL_SIZE,
            rest: REST,
        }
    }
}

//https://stackoverflow.com/questions/65028499/rust-structs-that-have-box-fields-and-that-impl-async-traits
#[async_trait]
pub trait Flow {
    async fn get_level3_links(&self) -> Vec<String> {
        vec![]
    }
    async fn get_level2_links(&self) -> Vec<String> {
        vec![]
    }
    async fn get_links(&self) -> Vec<String>;
    async fn get_terms(&self) -> Vec<Term>;
}

const POOL_SIZE: usize = 50;
const REST: u64 = 5;

#[async_trait]
impl Flow for HierarchicalFlow<'_> {
    async fn get_level3_links(&self) -> Vec<String> {
        if !self.level3_links.is_empty() {
            return self.level3_links.clone();
        }
        let base = resolve_base(self.level3_base, self.base);
        get_links(LinkQuery {
            url: &self.index,
            base,
            selector_string: &self.level3_selector,
            encoding: &self.encoding,
        })
        .await
        .unwrap()
    }
    async fn get_level2_links(&self) -> Vec<String> {
        if !self.level2_links.is_empty() {
            return self.level2_links.clone();
        }
        if !self.level2_selector.is_empty() && !self.level3_selector.is_empty() {
            let base = resolve_base(self.level3_base, self.base);
            let level3_links = self.get_level3_links().await;
            join_all(level3_links.iter().map(|l| {
                get_links(LinkQuery {
                    url: l,
                    base: base,
                    selector_string: self.level1_selector,
                    encoding: &self.encoding,
                })
            }))
            .await
            .into_iter()
            .flat_map(|l| l.unwrap())
            .collect()
        } else {
            let base = resolve_base(self.level2_base, self.base);
            get_links(LinkQuery {
                url: &self.index,
                base: base,
                selector_string: &self.level2_selector,
                encoding: &self.encoding,
            })
            .await
            .unwrap()
        }
    }
    async fn get_links(&self) -> Vec<String> {
        if !self.links.is_empty() {
            return self.links.clone();
        }
        let level2_links = if !self.level2_links.is_empty() {
            self.level2_links.clone()
        } else {
            self.get_level2_links().await
        };

        let base = resolve_base(self.level1_base, self.base);

        let chunks: Vec<Vec<String>> = level2_links
            .chunks(self.pool_size)
            .map(|c| c.to_vec())
            .collect();

        let mut result = vec![];

        for c in chunks.iter().map(|links| {
            join_all(links.iter().map(|l| {
                get_links(LinkQuery {
                    url: l,
                    base: base,
                    selector_string: self.level1_selector,
                    encoding: &self.encoding,
                })
            }))
        }) {
            let mut links: Vec<_> = c.await.into_iter().flat_map(|r| r.unwrap()).collect();

            tokio::time::sleep(Duration::from_secs(self.rest)).await;

            result.append(&mut links);
        }
        result
    }
    async fn get_terms(&self) -> Vec<Term> {
        let links = self.get_links().await;
        get_terms_chunked(
            links,
            self.pool_size,
            self.rest,
            self.title_selector,
            self.body_selector,
            self.image_selector,
            self.encoding,
        )
        .await
    }
}
#[async_trait]
impl Flow for SinglepageFlow {
    async fn get_links(&self) -> Vec<String> {
        if !self.links.is_empty() {
            self.links.clone()
        } else if self.level1_selector == "" {
            vec![self.index.to_string()]
        } else {
            get_links(LinkQuery {
                url: self.index,
                base: self.base,
                selector_string: self.level1_selector,
                encoding: &self.encoding,
            })
            .await
            .unwrap()
        }
    }
    async fn get_terms(&self) -> Vec<Term> {
        let links: Vec<String> = self.get_links().await;

        join_all(links.into_iter().map(|l| {
            get_terms(
                l,
                self.titles_selector,
                self.bodies_selector,
                None,
                self.encoding,
            )
        }))
        .await
        .into_iter()
        .flat_map(|r| r.unwrap())
        .collect()
    }
}

#[async_trait]
impl Flow for HeadingRangeFlow {
    async fn get_links(&self) -> Vec<String> {
        if !self.links.is_empty() {
            self.links.clone()
        } else if self.level1_selector == "" {
            vec![self.index.to_string()]
        } else {
            get_links(LinkQuery {
                url: self.index,
                base: self.base,
                selector_string: self.level1_selector,
                encoding: &self.encoding,
            })
            .await
            .unwrap()
        }
    }
    async fn get_terms(&self) -> Vec<Term> {
        let links: Vec<String> = self.get_links().await;

        let mut result = vec![];
        for l in links {
            result.extend(
                get_terms_heading_range(
                    l,
                    self.container_selector,
                    self.titles_selector,
                    self.last_body_selector,
                    self.encoding,
                )
                .await
                .unwrap(),
            );
        }
        result
    }
}

#[async_trait]
impl Flow for PageLinkFlow {
    async fn get_level2_links(&self) -> Vec<String> {
        if !self.level3_links.is_empty() {
            return self.level3_links.clone();
        }
        let base = resolve_base(self.level3_base, self.base);
        get_links(LinkQuery {
            url: &self.index,
            base,
            selector_string: &self.level3_selector,
            encoding: &self.encoding,
        })
        .await
        .unwrap()
    }
    async fn get_links(&self) -> Vec<String> {
        if !self.links.is_empty() {
            return self.links.clone();
        }
        let level2_links = if !self.level2_links.is_empty() {
            self.level2_links.clone()
        } else {
            self.get_level2_links().await
        };

        let mut links = vec![];
        if !self.links.is_empty() {
            self.links.clone()
        } else if self.level1_selector == "" {
            vec![self.index.to_string()]
        } else {
            for link_link in level2_links {
                links.extend(
                    get_links_by_page_link(
                        &link_link,
                        self.base,
                        self.level1_selector,
                        self.level2_selector,
                        &self.encoding,
                    )
                    .await,
                )
            }
            links
        }
    }
    async fn get_terms(&self) -> Vec<Term> {
        let links = self.get_links().await;
        get_terms_chunked(
            links,
            self.pool_size,
            self.rest,
            self.title_selector,
            self.body_selector,
            self.image_selector,
            self.encoding,
        )
        .await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Term {
    pub title: String,
    pub body: String,
    pub images: Vec<String>,
}

const RETRY: usize = 5;
const RETRY_INTERVAL: u64 = 5;
const BANNED_INTERVAL: u64 = 600;
const APP_USER_AGENT: &str = "Mozilla/5.0 (MSIE; Windows 10)";

fn http_client() -> &'static reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
    CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()
            .expect("Failed to build HTTP client")
    })
}

pub async fn get_html(url: impl AsRef<str>, encoding_str: &str) -> reqwest::Result<String> {
    let client = http_client();

    let encoding = match encoding_str {
        "utf-8" => encoding_rs::UTF_8,
        "euc-jp" => encoding_rs::EUC_JP,
        "shift-jis" => encoding_rs::SHIFT_JIS,
        _ => encoding_rs::UTF_8,
    };
    println!("{} is getting", url.as_ref());
    let mut bytes_opt = None;
    for i in 1..RETRY {
        let result = client.get(url.as_ref()).send().await;
        if result.is_err() {
            println!(
                "{} failed {} times due to {:?}. retrying",
                url.as_ref(),
                i,
                result
            );
            tokio::time::sleep(Duration::from_secs(RETRY_INTERVAL)).await;
            continue;
        }
        let response = result.unwrap();
        if response.status() == reqwest::StatusCode::SERVICE_UNAVAILABLE {
            println!(
                "{} failed {} times due to service unavailable. retrying",
                url.as_ref(),
                i
            );
            tokio::time::sleep(Duration::from_secs(RETRY_INTERVAL)).await;
            continue;
        } else if response.status() == reqwest::StatusCode::FORBIDDEN {
            println!(
                "{} failed {} times due to forbidden. retrying",
                url.as_ref(),
                i
            );
            tokio::time::sleep(Duration::from_secs(BANNED_INTERVAL)).await;
            continue;
        } else if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
            println!(
                "{} failed {} times due to too many requests. retrying",
                url.as_ref(),
                i
            );
            tokio::time::sleep(Duration::from_secs(BANNED_INTERVAL)).await;
            continue;
        }
        bytes_opt = Some(response.bytes().await.unwrap());
        break;
    }
    let mut bytes = bytes_opt.expect("the number of retries exceeded");
    if bytes.starts_with(&[0x1f, 0x8b]) {
        let mut decoder = GzDecoder::new(&bytes[..]);
        let mut decoded_bytes = Vec::new();
        decoder.read_to_end(&mut decoded_bytes).unwrap();
        bytes = decoded_bytes.into();
    }
    let (res, _, _) = encoding.decode(&bytes);
    Ok(res.to_string())
}

pub fn use_write(path: String) -> Box<dyn Fn(&Vec<Term>) -> ()> {
    Box::new(move |terms| {
        create_dir_all(DEFAULT_BASE_PATH).unwrap();
        let serialized = serde_json::to_string(terms).unwrap();
        let mut f = File::create(DEFAULT_BASE_PATH.to_string() + &path).unwrap();
        f.write_all(serialized.as_bytes()).unwrap();
        ()
    })
}

pub struct LinkQuery<'a> {
    pub base: &'a str,
    pub url: &'a str,
    pub selector_string: &'a str,
    pub encoding: &'a str,
}

pub async fn get_links(q: LinkQuery<'_>) -> reqwest::Result<Vec<String>> {
    let body: String = get_html(q.url, q.encoding).await?;

    let fragment = Html::parse_fragment(&body);

    let selector = Selector::parse(q.selector_string).unwrap();

    // hrefだけ書いてあることがあったり、その他は正常だったりするので、正常なもののみ抽出
    Ok(fragment
        .select(&selector)
        .map(|e| join_url(q.url, e.value().attr("href").unwrap_or("")))
        .filter(|l| !l.is_empty())
        .collect())
}

pub trait Converter {
    fn get_fragment(&self) -> String;
    fn get_selector(&self) -> String;
}

pub enum GetTextFragment {
    Html(Html),
    RefHtml(&'static Html),
}

pub enum GetTextSelector {
    Selector(Selector),
    RefSelector(&'static Selector),
}

pub fn get_text(fragment: Html, selector: Selector) -> String {
    fragment
        .select(&selector)
        .flat_map(|e| {
            e.text()
                .map(|t| t.to_string().trim().to_string())
                .collect::<Vec<String>>()
        })
        .collect()
}
pub fn get_texts(fragment: GetTextFragment, selector: GetTextSelector) -> Vec<String> {
    let html_fragment: Html = match fragment {
        GetTextFragment::Html(h) => h,
        GetTextFragment::RefHtml(h) => h.clone(),
    };

    let css_selector = match selector {
        GetTextSelector::Selector(s) => s,
        GetTextSelector::RefSelector(s) => s.clone(),
    };

    html_fragment
        .select(&css_selector)
        .map(|e| {
            e.text()
                .map(|t| t.to_string().trim().to_string())
                .collect::<String>()
        })
        .collect()
}

pub fn get_image_source(fragment: &Html, selector: &Selector) -> String {
    fragment
        .select(selector)
        .flat_map(|e| {
            e.text()
                .map(|t| t.to_string().trim().to_string())
                .collect::<Vec<String>>()
        })
        .collect()
}

pub fn parse_fragment(html: String) -> Html {
    Html::parse_fragment(&html)
}

pub async fn get_links_by_page_link(
    start: &str,
    base: &str,
    link_selector: &str,
    next_selector: &str,
    encoding: &str,
) -> Vec<String> {
    let mut links = Vec::new();
    let mut next_link = start.to_string();
    loop {
        let mut links_result = get_links(LinkQuery {
            url: &next_link,
            base,
            selector_string: link_selector,
            encoding,
        })
        .await
        .unwrap();
        links.append(&mut links_result);
        let next_result = get_links(LinkQuery {
            url: &next_link,
            base,
            selector_string: next_selector,
            encoding,
        })
        .await
        .unwrap();
        if next_result.is_empty() {
            break;
        } else {
            next_link = next_result.first().unwrap().clone();
        }
    }
    links
}

pub async fn get_term(
    url: String,
    s_title: &str,
    s_body: &str,
    s_images: Option<&str>,
    encoding: &str,
) -> reqwest::Result<Term> {
    for i in 1..RETRY {
        let html = get_html(&url, encoding).await.unwrap();

        let mut term_opt = None;
        {
            let fragment = Html::parse_fragment(&html);
            let title_selector = Selector::parse(s_title).unwrap();
            let body_selector = Selector::parse(s_body).unwrap();

            let title = get_text(fragment.clone(), title_selector.clone());
            let body = get_text(fragment.clone(), body_selector.clone());

            if !title.is_empty() || !body.is_empty() {
                let images: Vec<String> = match s_images {
                    Some(s) => {
                        let images_selector = Selector::parse(s).unwrap();
                        fragment
                            .select(&images_selector)
                            .map(|e| {
                                e.value()
                                    .attr("src")
                                    .map(|s| s.to_string())
                                    .unwrap_or_else(|| "".to_string())
                            })
                            .filter(|s| !s.is_empty())
                            .collect::<Vec<String>>()
                    }
                    None => vec![],
                };

                term_opt = Some(Term {
                    title: title,
                    body: body,
                    images: images,
                });
            }
        }

        if let Some(term) = term_opt {
            return Ok(term);
        }

        println!("{} title and body is empty. retrying {} times", url, i);
        tokio::time::sleep(Duration::from_secs(RETRY_INTERVAL)).await;
    }

    Ok(Term {
        title: "".to_string(),
        body: "".to_string(),
        images: vec![],
    })
}

pub async fn get_terms(
    url: String,
    s_title: &str,
    s_body: &str,
    s_images: Option<&str>,
    encoding: &str,
) -> reqwest::Result<Vec<Term>> {
    let html = get_html(url, encoding).await.unwrap();

    let title_selector = Selector::parse(s_title).unwrap();
    let body_selector = Selector::parse(s_body).unwrap();
    let fragment = Html::parse_fragment(&html);

    let titles = get_texts(
        GetTextFragment::Html(fragment.clone()),
        GetTextSelector::Selector(title_selector.clone()),
    );

    let bodies = get_texts(
        GetTextFragment::Html(fragment.clone()),
        GetTextSelector::Selector(body_selector.clone()),
    );

    if titles.len() != bodies.len() {
        panic!(
            "titles and bodies is not coincident\ntitles: {}, bodies: {}",
            titles.len(),
            bodies.len()
        )
    }

    let images: Vec<String> = match s_images {
        Some(s) => {
            let images_selector = Selector::parse(s).unwrap();
            fragment
                .select(&images_selector)
                .map(|e| e.value().attr("src").unwrap().to_string())
                .collect::<Vec<String>>()
        }
        None => vec![],
    };

    Ok((0..titles.len())
        .map(|i| Term {
            title: titles[i].to_string(),
            body: bodies[i].to_string(),
            images: images.clone(),
        })
        .collect())
}

pub async fn get_terms_heading_range(
    url: String,
    s_container: &str,
    s_title: &str,
    s_stop: &str,
    encoding: &str,
) -> reqwest::Result<Vec<Term>> {
    let html = get_html(url, encoding).await.unwrap();

    let title_selector = Selector::parse(s_title).unwrap();
    let last_body_selector = if !s_stop.is_empty() {
        Some(Selector::parse(s_stop).unwrap())
    } else {
        None
    };
    let fragment = Html::parse_fragment(&html);

    let containers = if !s_container.is_empty() {
        let container_selector = Selector::parse(s_container).unwrap();
        fragment.select(&container_selector).collect::<Vec<_>>()
    } else {
        vec![fragment.root_element()]
    };

    let mut terms = vec![];

    for container in containers {
        let titles = container.select(&title_selector).collect::<Vec<_>>();

        for title_node in titles {
            let title_text = title_node.text().collect::<String>().trim().to_string();
            let mut body_text = String::new();

            let mut curr = title_node.next_sibling();
            while let Some(node) = curr {
                if node.value().is_element() {
                    let element = scraper::ElementRef::wrap(node).unwrap();
                    if title_selector.matches(&element) {
                        break;
                    }
                    if let Some(ref stop) = last_body_selector {
                        if stop.matches(&element) {
                            body_text.push_str(&element.text().collect::<String>());
                            break;
                        }
                    }
                    body_text.push_str(&element.text().collect::<String>());
                } else if node.value().is_text() {
                    body_text.push_str(node.value().as_text().unwrap());
                }
                curr = node.next_sibling();
            }

            terms.push(Term {
                title: title_text,
                body: body_text.trim().to_string(),
                images: vec![],
            });
        }
    }

    Ok(terms)
}

async fn get_terms_chunked(
    links: Vec<String>,
    pool_size: usize,
    rest_secs: u64,
    title_selector: &str,
    body_selector: &str,
    image_selector: Option<&str>,
    encoding: &str,
) -> Vec<Term> {
    let mut result = vec![];
    for chunk in links.chunks(pool_size) {
        let terms: Vec<_> = join_all(chunk.iter().map(|l| {
            get_term(
                l.clone(),
                title_selector,
                body_selector,
                image_selector,
                encoding,
            )
        }))
        .await
        .into_iter()
        .map(|r| r.unwrap())
        .collect();

        result.extend(terms);
        if links.len() > pool_size {
            tokio::time::sleep(Duration::from_secs(rest_secs)).await;
        }
    }
    result
}

fn resolve_base<'a>(primary: &'a str, fallback: &'a str) -> &'a str {
    if !primary.is_empty() {
        primary
    } else {
        fallback
    }
}

fn join_url(left: &str, right: &str) -> String {
    let left_url = Url::parse(left).unwrap();

    left_url.join(right).unwrap().to_string()
}
