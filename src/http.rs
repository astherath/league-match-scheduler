use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use std::env;

type ParamPairs<'a> = &'a [(&'a str, &'a str)];
type UrlString = String;

pub async fn get_data_from_url<T>(client: &Client, url: UrlString) -> T
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    client.get(&url).send().await.unwrap().json().await.unwrap()
}

pub fn get_url_for_endpoint_with_params(endpoint: &str, params: ParamPairs) -> UrlString {
    let mut base_url = base_url().join(endpoint).unwrap();
    let mut url_with_params = base_url.query_pairs_mut();
    url_with_params.clear();

    params.iter().for_each(|x| {
        url_with_params.append_pair(&x.0, &x.1);
    });

    url_with_params.finish().as_str().to_string()
}

fn base_url() -> Url {
    let base_url_str = "https://esports-api.lolesports.com/persisted/gw/";
    Url::parse(base_url_str).unwrap()
}

pub fn create_http_client() -> Client {
    let headers = get_default_headers();
    Client::builder().default_headers(headers).build().unwrap()
}

fn get_default_headers() -> HeaderMap {
    let api_key = get_api_key();
    let x_api_key = HeaderValue::from_str(&api_key).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert("X-API-KEY", x_api_key);

    headers
}

fn get_api_key() -> String {
    let env_key = "RIOT_API_KEY";
    env::var(env_key).expect(&format!("riot games API key (\"{}\") not set!", env_key))
}
