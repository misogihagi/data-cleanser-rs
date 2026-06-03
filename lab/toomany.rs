use futures::future::join_all;
use reqwest::Client;

#[tokio::main]
async fn main() {
    let url =
        "https://www.yodosha.co.jp/jikkenigaku/keyword/90%EF%BC%85%20from%20peak,%20s/id/1852";
    let client = Client::new();

    let tasks = (1..100).map(|_| {
        let client = client.clone();
        async move {
            let response = client.get(url.to_string()).send().await.unwrap();

            if &response.status().as_u16() != &200 {
                println!("{:?}", &response.headers());
                let html = response.text().await.unwrap();
                println!("{}", html);
            }
        }
    });

    join_all(tasks).await;
}
