use http::{Method, Uri};

#[allow(dead_code)]
struct ClientOpts;

#[derive(Default)]
struct ModImpl;

#[autotrait::autotrait]
impl Mod for ModImpl {
    fn client(&self) -> Box<dyn HttpClient> {
        todo!()
    }

    fn client_with_opts(&self, _opts: ClientOpts) -> Box<dyn HttpClient> {
        todo!()
    }
}

#[allow(dead_code)]
struct HttpClientImpl {
    //
}

#[autotrait::autotrait]
impl HttpClient for HttpClientImpl {
    fn request(&self, _method: Method, _uri: Uri) -> Box<dyn RequestBuilder> {
        Box::new(RequestBuilderImpl {})
    }

    fn get(&self, uri: Uri) -> Box<dyn RequestBuilder> {
        self.request(Method::GET, uri)
    }

    fn post(&self, uri: Uri) -> Box<dyn RequestBuilder> {
        self.request(Method::POST, uri)
    }

    fn put(&self, uri: Uri) -> Box<dyn RequestBuilder> {
        self.request(Method::PUT, uri)
    }

    fn delete(&self, uri: Uri) -> Box<dyn RequestBuilder> {
        self.request(Method::DELETE, uri)
    }
}

#[allow(dead_code)]
struct RequestBuilderImpl;

#[autotrait::autotrait]
impl RequestBuilder for RequestBuilderImpl {}

fn main() {
    let m: Box<dyn Mod> = Box::new(ModImpl);
    m.client();
}
