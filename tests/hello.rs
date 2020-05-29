use reqwest::Client;
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};


#[tokio::test]
async fn hello_reqwest() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

    println!("sending reqwest request");
    let resp = Client::new()
        .get(&mock_server.uri())
        .send()
        .await
        .unwrap();
    println!("reqwest request done");

    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn hello_surf() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

    println!("sending surf request");
    let status = surf::get(&mock_server.uri())
        .await
        .unwrap()
        .status();
    println!("surf request done");

    assert_eq!(status.as_u16(), 200);
}