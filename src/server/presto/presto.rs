use hyper::Client;

pub struct PrestoClient {
    http_client: Client,
}
