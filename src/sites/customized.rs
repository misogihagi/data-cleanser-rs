use super::interface::WorkFlowTrait;
use crate::utils::{get_links, Flow, FlowA, FlowB, LinkQuery, Term};

pub enum SiteKindCustomized {
    Hrpro,
    Shimauma,
    Zexy,
    HomemateResearchSoccer,
}

pub struct CustomizedWorkFlow {
    pub kind: SiteKindCustomized,
}
impl CustomizedWorkFlow {
    pub fn new(kind_str: &'static str) -> CustomizedWorkFlow {
        CustomizedWorkFlow {
            kind: CustomizedWorkFlow::my_kind(kind_str).unwrap(),
        }
    }
    pub fn my_kind(kind_str: &'static str) -> Option<SiteKindCustomized> {
        match kind_str {
            "hrpro" => Some(SiteKindCustomized::Hrpro),
            "shimauma" => Some(SiteKindCustomized::Shimauma),
            "zexy" => Some(SiteKindCustomized::Zexy),
            "homemateresearchsoccer" => Some(SiteKindCustomized::HomemateResearchSoccer),
            _ => None,
        }
    }
}

impl WorkFlowTrait for CustomizedWorkFlow {
    fn is_my_kind(kind_str: &'static str) -> bool {
        match CustomizedWorkFlow::my_kind(kind_str) {
            Some(_) => true,
            None => false,
        }
    }
    async fn get_terms(&self) -> Vec<Term> {
        customize(&self.kind).await.get_terms().await
    }
}

async fn customize(kind: &SiteKindCustomized) -> Box<dyn Flow> {
    match kind {
        SiteKindCustomized::HomemateResearchSoccer => {
            let from_the_second_links = get_links(LinkQuery {
                url: "https://www.homemate-research-soccer.com/useful/14329_sport_002/index.php",
                base: "https://www.homemate-research-soccer.com/useful/14329_sport_002/",
                selector_string: "#con_nav > ul > li > a",
                encoding: "utf-8",
            })
            .await
            .unwrap();

            let mut links = vec![
                "https://www.homemate-research-soccer.com/useful/14329_sport_002/index.php"
                    .to_string(),
            ];
            links.extend(from_the_second_links);

            Box::new(FlowB {
                links: links,
                titles_selector: "#article > section > section > h3",
                bodies_selector: "#article > section > section > p",
                ..Default::default()
            })
        }
        SiteKindCustomized::Hrpro => {
            let resource = "https://www.hrpro.co.jp/glossary.php?";
            let urls = vec!["a", "k", "s", "t", "n", "h", "m", "y", "r", "w"]
                .into_iter()
                .fold(vec![], |mut total, query| {
                    total.append(&mut vec![
                        resource.to_string() + "index=" + query,
                        resource.to_string() + "index=" + query + "&pcnt=2",
                    ]);
                    total
                });
            Box::new(FlowA {
                index: "https://www.hrpro.co.jp/glossary.php",
                base: "https://www.hrpro.co.jp/",
                link_selector: ".rlt-list > li > a",
                title_selector: "h1.ttl",
                body_selector: ".article-body",
                link_links: urls,
                ..Default::default()
            })
        }
        SiteKindCustomized::Shimauma => {
            let mut links = vec![String::from("https://makitani.net/shimauma/page/1")];

            // for unexpected infinite loop
            // in preparation for selector undetection
            for n in 1..200 {
                let result = get_links(LinkQuery {
                    base: "",
                    url: links.last().unwrap(),
                    selector_string: ".next",
                    encoding: "utf-8",
                })
                .await
                .unwrap();

                if let Some(link) = result.first() {
                    if n > 199 {
                        panic!("shimauma has more pages than expected!")
                    }
                    links.push(link.clone());
                } else {
                    break;
                }
            }

            Box::new(FlowA {
                link_links: links,
                link_selector: "#content > article > header > h1 > a",
                title_selector: ".entry-title",
                body_selector: ".entry-content > p",
                ..Default::default()
            })
        }
        SiteKindCustomized::Zexy => {
            let resource = "https://zexy.net/contents/yogo/50?key=";
            let urls = vec!["あ", "か", "さ", "た", "な", "は", "ま", "や", "ら", "わ"]
                .into_iter()
                .map(|s| String::from(resource) + s)
                .collect();
            Box::new(FlowA {
                link_links: urls,
                base: "https://zexy.net/contents/yogo/50/",
                link_selector: ".glossary > ul > li > a",
                title_selector: ".textBody > h3",
                body_selector: "#item01 > p, #item01 > dl",
                ..Default::default()
            })
        }
    }
}
