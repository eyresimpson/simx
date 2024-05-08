use reqwest::get;
use tokio;

#[tokio::test]
async fn it_works() {
    println!("ok");
    let url = "http://httpbin.org/get";
    let response = get(url).await.unwrap();

    println!("Status: {}", response.status());
    println!("Headers:\n{:#?}", response.headers());

    let body = response.text().await.unwrap();
    println!("Body:\n{}", body);
    assert_eq!(2 + 2, 4);
}
