use futures::future;

use gotham::handler::{Handler, HandlerFuture};
use gotham::helpers::http::response::create_response;
use gotham::state::State;
use hyper::StatusCode;

#[derive(Clone, Debug)]
pub struct RootHandler(ToshiInfo);

#[derive(Clone, Debug, Serialize)]
struct ToshiInfo {
    name:    String,
    version: String,
}

impl RootHandler {
    pub fn new(version: String) -> Self {
        RootHandler(ToshiInfo {
            version,
            name: "Toshi Search".to_string(),
        })
    }
}

impl Handler for RootHandler {
    fn handle(self, state: State) -> Box<HandlerFuture> {
        let body = serde_json::to_vec(&self.0).unwrap();
        let resp = create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, body);
        Box::new(future::ok((state, resp)))
    }
}

new_handler!(RootHandler);

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::test::TestServer;
    use settings::VERSION;

    #[test]
    fn test_root() {
        let handler = RootHandler::new(VERSION.to_string());
        let test_server = TestServer::new(handler).unwrap();
        let client = test_server.client();

        let req = client.get("http://localhost").perform().unwrap();
        assert_eq!(req.status(), StatusCode::OK);
        assert_eq!(req.read_utf8_body().unwrap(), r#"{"name":"Toshi Search","version":"0.1.1"}"#);
    }
}
