use crate::sites::interface::WorkFlowTrait;
use crate::utils::Term;
use chromiumoxide::browser::{Browser, BrowserConfig};
use futures::StreamExt;
use scraper::{Html, Selector};
use std::time::Duration;

pub enum SiteKindBrowser {
    Konest,
}

pub struct BrowserWorkFlow {
    pub kind: SiteKindBrowser,
}

impl BrowserWorkFlow {
    pub fn new(kind_str: &'static str) -> Self {
        BrowserWorkFlow {
            kind: Self::my_kind(kind_str).unwrap(),
        }
    }

    pub fn my_kind(kind_str: &'static str) -> Option<SiteKindBrowser> {
        match kind_str {
            "konest" => Some(SiteKindBrowser::Konest),
            _ => None,
        }
    }
}

impl WorkFlowTrait for BrowserWorkFlow {
    fn is_my_kind(kind_str: &'static str) -> bool {
        Self::my_kind(kind_str).is_some()
    }

    async fn get_terms(&self) -> Vec<Term> {
        match self.kind {
            SiteKindBrowser::Konest => {}
        }
    }
}

async fn get_html_with_browser(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (mut browser, mut handler) =
        Browser::launch(BrowserConfig::builder().with_head().build()?).await?;

    let handle = tokio::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });
    let page = browser
        .start_incognito_context()
        .await?
        .new_page(url)
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_secs(2)).await;

    let content = page.content().await?;

    browser.close().await?;
    let _ = handle.await;

    Ok(content)
}
