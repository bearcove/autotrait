use std::{borrow::Cow, collections::HashMap};

use bytes::Bytes;
use futures_util::{future::BoxFuture, stream::BoxStream};
use http::{Error, Method, StatusCode, Uri};

#[allow(dead_code)]
struct ClientOpts;

#[derive(Default)]
struct ModImpl;

#[autotrait::autotrait]
impl Mod for ModImpl {
    fn blah(&self) -> Result<Vec<u8>, Error> {
        todo!()
    }

    fn hashmap_string_string(&self) -> HashMap<String, String> {
        todo!()
    }

    fn return_cow_str(&self) -> Cow<'static, str> {
        todo!()
    }

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

struct ResponseImpl {
    response: (),
}

impl ResponseImpl {
    fn new(response: ()) -> Self {
        Self { response }
    }
}

#[autotrait::autotrait]
impl Response for ResponseImpl {
    fn status(&self) -> StatusCode {
        todo!()
    }

    fn headers_only_string_safe(&self) -> HashMap<String, String> {
        todo!()
    }

    fn bytes(self: Box<Self>) -> BoxFuture<'static, Result<Vec<u8>, Error>> {
        todo!()
    }

    fn bytes_stream(self: Box<Self>) -> BoxStream<'static, Result<Bytes, Error>> {
        todo!()
    }

    fn text(self: Box<Self>) -> BoxFuture<'static, Result<String, Error>> {
        todo!()
    }
}
