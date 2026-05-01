#[derive(Clone)]
pub struct HttpState {
    pub client: reqwest::Client,
}
