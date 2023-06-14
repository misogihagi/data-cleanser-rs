use std::{fs::File, error::Error};
use std::io::Write;
use futures::future::join_all;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

async fn get_indexes()->reqwest::Result<Vec<String>>{
     let body = reqwest::get("https://www.mintetsu.or.jp/knowledge/index.html")
    .await?
    .text()
    .await?;

    let fragment = Html::parse_fragment(&body);
    let aselector = Selector::parse("#aside_inr").unwrap();
    let bselector = Selector::parse("ul").unwrap();
    let cselector = Selector::parse("a").unwrap();

    Ok(
    fragment.select(&aselector)
    .next()
    .unwrap()
    .select(&bselector).nth(1).unwrap()
    .select(&cselector)
        .map(
            |e| e.value().attr("href").unwrap().to_string()
        ).collect()
    )
}

async fn get_terms(url:&str)->reqwest::Result<Vec<String>>{
    let body = reqwest::get(url)
    .await?
    .text()
    .await?;

    let fragment = Html::parse_fragment(&body);
    let aselector = Selector::parse("#section_inr > div").unwrap();
    let bselector = Selector::parse("a").unwrap();

    Ok(fragment.select(&aselector).next().unwrap()
    .select(&bselector)
    .map(
        |e| e.value().attr("href").unwrap().to_string()
    ).collect()
)

}

#[derive(Serialize, Deserialize)]
struct Term {
    title: String,
    body: String,
}

type Terms = Vec<Term>;


async fn get_term(url:&str)->reqwest::Result<Term>{
    println!("{}",url);
    let retry=5;
    let mut count=0;
    let mut body:String="".to_string();

    loop {
        let res=reqwest::get(url).await;
        match res {
            Ok(response) => {
                body=response.text().await?;
                break;
            },
            Err(err)=>{
                 if  err.source().unwrap().source().unwrap().to_string() == "tcp connect error: Connection refused (os error 111)" {
                    tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
                 }else {
                    panic!("{}",err)
                 };
            }
        };
        count+=1;
        println!("retrying...{}",&url);
        if count > retry {
            panic!("retry count exceed")
        }
    };
    
    let fragment = Html::parse_fragment(&body);

    let title_selector = Selector::parse("h1").unwrap();
    let body_selector = Selector::parse(".section.clearfix").unwrap();

    let title_element=
    fragment.select(&title_selector).next().unwrap();

    let body_element=
    fragment.select(&body_selector).next().unwrap();

    Ok(Term {
        title:title_element.text().collect(),
        body:body_element.text().collect(),
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let host="https://www.mintetsu.or.jp".to_string();

    let idxs:Vec<String>=
    get_indexes()
    .await?
    .iter()
    .map(|x| host.clone()+x)
    .collect()
    ;


    let term_urls:Vec<String>=
    join_all(
        idxs
        .iter()
        .map(|x| get_terms(x))
    ).await
    .into_iter().flat_map(|x| x.unwrap()).collect();

    let t:Terms=
    join_all(term_urls.iter().map(|x| get_term(x))).await
    .into_iter().map(|x| x.unwrap()).collect();
    
    let serialized = serde_json::to_string(&t).unwrap();
    
    let mut f = File::create("output.json").unwrap(); // open file, you can write to file.
    // "create" open as write only mode.
    f.write_all(serialized.as_bytes()).unwrap(); // byte-only

    Ok(())
}
