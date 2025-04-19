use async_trait::async_trait;
use data_cleanser_rs::constants::DEFAULT_BASE_PATH;
use futures::future::join_all;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::vec;
use std::{thread, time};
use url::{Host, Position, Url};

pub struct FlowA<'a> {
    pub index: &'a str,
    pub base: &'a str,
    pub link_link_base: &'a str,
    pub link_base: &'a str,
    pub link_link_selector: &'a str,
    pub link_selector: &'a str,
    pub title_selector: &'a str,
    pub body_selector: &'a str,
    pub image_selector: Option<&'a str>,
    pub encoding: &'a str,
    pub link_links: Vec<String>,
    pub links: Vec<String>,
    pub pool_size: usize,
    pub rest: u64,
}

impl Default for FlowA<'_> {
    fn default() -> FlowA<'static> {
        FlowA {
            index: "",
            base: "",
            link_link_base: "",
            link_base: "",
            link_link_selector: "",
            link_selector: "",
            title_selector: "",
            body_selector: "",
            image_selector: None,
            encoding: "utf-8",
            link_links: vec![],
            links: vec![],
            pool_size: POOL_SIZE,
            rest: REST,
        }
    }
}

pub struct FlowB {
    pub index: &'static str,
    pub base: &'static str,
    pub link_selector: &'static str,
    pub titles_selector: &'static str,
    pub bodies_selector: &'static str,
    pub encoding: &'static str,
    pub links: Vec<String>,
}
impl Default for FlowB {
    fn default() -> FlowB {
        FlowB {
            index: "",
            base: "",
            link_selector: "",
            titles_selector: "",
            bodies_selector: "",
            encoding: "utf-8",
            links: vec![],
        }
    }
}

//https://stackoverflow.com/questions/65028499/rust-structs-that-have-box-fields-and-that-impl-async-traits
#[async_trait]
pub trait Flow {
    async fn get_link_links(&self) -> Vec<String>;
    async fn get_links(&self) -> Vec<String>;
    async fn get_terms(&self) -> Vec<Term>;
}

const POOL_SIZE: usize = 50;
const REST: u64 = 5;

#[async_trait]
impl Flow for FlowA<'_> {
    async fn get_link_links(&self) -> Vec<String> {
        if self.link_links.len() > 0 {
            self.link_links.clone()
        } else {
            let base = if !self.link_link_base.is_empty() {
                self.link_link_base
            } else {
                if !self.base.is_empty() {
                    self.base
                } else {
                    ""
                }
            };
            get_links(LinkQuery {
                url: &self.index,
                base: base,
                selector_string: &self.link_link_selector,
                encoding: &self.encoding,
            })
            .await
            .unwrap()
        }
    }
    async fn get_links(&self) -> Vec<String> {
        if self.links.len() > 0 {
            return self.links.clone();
        }
        let link_links = if self.link_links.len() > 0 {
            self.link_links.clone()
        } else {
            self.get_link_links().await
        };

        let base = if !self.link_base.is_empty() {
            self.link_base
        } else {
            if !self.base.is_empty() {
                self.base
            } else {
                ""
            }
        };
        join_all(link_links.iter().map(|l| {
            get_links(LinkQuery {
                url: l,
                base: base,
                selector_string: self.link_selector,
                encoding: &self.encoding,
            })
        }))
        .await
        .into_iter()
        .flat_map(|l| l.unwrap())
        .collect()
    }
    async fn get_terms(&self) -> Vec<Term> {
        let chunks: Vec<Vec<String>> = self
            .get_links()
            .await
            .chunks(self.pool_size)
            .map(|c| c.to_vec())
            .collect();

        let mut result = vec![];

        for c in chunks.into_iter().map(|links| {
            join_all(links.into_iter().map(|l| {
                get_term(
                    l,
                    self.title_selector,
                    self.body_selector,
                    self.image_selector,
                    self.encoding,
                )
            }))
        }) {
            let mut terms: Vec<_> = c.await.into_iter().map(|r| r.unwrap()).collect();

            thread::sleep(time::Duration::from_secs(self.rest));

            result.append(&mut terms)
        }
        result
    }
}
#[async_trait]
impl Flow for FlowB {
    async fn get_link_links(&self) -> Vec<String> {
        vec![]
    }
    async fn get_links(&self) -> Vec<String> {
        if self.links.len() > 0 {
            self.links.clone()
        } else if self.link_selector == "" {
            vec![self.index.to_string()]
        } else {
            get_links(LinkQuery {
                url: self.index,
                base: self.base,
                selector_string: self.link_selector,
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

pub async fn get_html(url: impl AsRef<str>, encoding_str: &str) -> reqwest::Result<String> {
    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;

    let encoding = match encoding_str {
        "utf-8" => encoding_rs::UTF_8,
        "euc-jp" => encoding_rs::EUC_JP,
        "shift-jis" => encoding_rs::SHIFT_JIS,
        _ => encoding_rs::UTF_8,
    };
    println!("{} is getting", url.as_ref());
    let mut r = None;
    for i in 1..RETRY {
        let result = client.get(url.as_ref()).send().await;
        if result.is_err() {
            println!("{} failed {} times. retrying", url.as_ref(), i);
            thread::sleep(time::Duration::from_secs(RETRY_INTERVAL));
            continue;
        }
        let response = result.unwrap();
        if response.status() == reqwest::StatusCode::SERVICE_UNAVAILABLE {
            println!("{} failed {} times. retrying", url.as_ref(), i);
            thread::sleep(time::Duration::from_secs(RETRY_INTERVAL));
            continue;
        } else if response.status() == reqwest::StatusCode::FORBIDDEN {
            println!("{} failed {} times. retrying", url.as_ref(), i);
            thread::sleep(time::Duration::from_secs(BANNED_INTERVAL));
            continue;
        }
        r = Some(response.bytes().await.unwrap());
        break;
    }
    let bytes = r.expect("the number of retries exceeded");
    let (res, _, _) = encoding.decode(&bytes);
    Ok(res.to_string())
}

pub fn use_write(path: String) -> Box<dyn Fn(&Vec<Term>) -> ()> {
    Box::new(move |terms| {
        create_dir_all(DEFAULT_BASE_PATH).unwrap();
        let serialized = serde_json::to_string(terms).unwrap();
        let mut f = File::create(DEFAULT_BASE_PATH.clone().to_owned() + &path).unwrap();
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

    Ok(fragment
        .select(&selector)
        .map(|e| join_url(q.url, e.value().attr("href").unwrap()))
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

    let t = html_fragment.select(&css_selector).map(|e| e.children());

    let mut i = 0;

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
pub async fn get_term(
    url: String,
    s_title: &str,
    s_body: &str,
    s_images: Option<&str>,
    encoding: &str,
) -> reqwest::Result<Term> {
    let html = get_html(url, encoding).await.unwrap();

    let title_selector = Selector::parse(s_title).unwrap();
    let body_selector = Selector::parse(s_body).unwrap();
    let fragment = Html::parse_fragment(&html);

    let title = get_text(fragment.clone(), title_selector.clone());
    let body = get_text(fragment.clone(), body_selector.clone());

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

    Ok(Term {
        title: title,
        body: body,
        images: images,
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

    if (titles.len() != bodies.len()) {
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

fn join_url(left: &str, right: &str) -> String {
    let left_url = Url::parse(left).unwrap();

    left_url.join(right).unwrap().to_string()
}
