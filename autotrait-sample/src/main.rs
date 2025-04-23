use std::{borrow::Cow, collections::HashMap};

use bytes::Bytes;
use futures_util::{future::BoxFuture, stream::BoxStream};
use http::{Error, HeaderName, HeaderValue, Method, StatusCode, Uri};

#[allow(dead_code)]
struct ClientOpts;

#[derive(Default)]
struct ModImpl;

#[autotrait::autotrait]
impl Mod for ModImpl {
    fn box_self(self: Box<Self>) -> Result<Vec<u8>, Error> {
        todo!()
    }

    fn mut_box_self(mut self: Box<Self>) -> Result<Vec<u8>, Error> {
        self = todo!();
    }

    fn mut_ref_self(&mut self) -> Result<Vec<u8>, Error> {
        *self = todo!();
    }

    fn tuple_arg(&self, a: (u32, u64)) {
        todo!()
    }

    fn and_str(&self, s: &str) {
        todo!()
    }

    fn and_string(&self, s: String) {
        todo!()
    }

    fn and_qualified_string(&self, s: std::string::String) {
        todo!()
    }

    fn and_dyn(&self, s: &dyn std::fmt::Debug) {
        todo!()
    }

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

#[autotrait::autotrait]
impl RequestBuilder for RequestBuilderImpl {
    fn body(mut self: Box<Self>, body: Bytes) -> Box<dyn RequestBuilder> {
        todo!()
    }

    fn form(mut self: Box<Self>, form: String) -> Box<dyn RequestBuilder> {
        todo!()
    }

    fn header(mut self: Box<Self>, key: HeaderName, value: HeaderValue) -> Box<dyn RequestBuilder> {
        todo!()
    }

    /// Sets a "polite" user agent, letting the server know where to reach us.
    fn polite_user_agent(mut self: Box<Self>) -> Box<dyn RequestBuilder> {
        todo!()
    }

    /// Sets a browser-like user Agent
    fn browser_like_user_agent(mut self: Box<Self>) -> Box<dyn RequestBuilder> {
        todo!()
    }

    fn basic_auth(
        mut self: Box<Self>,
        username: &str,
        password: Option<&str>,
    ) -> Box<dyn RequestBuilder> {
        todo!()
    }

    fn bearer_auth(mut self: Box<Self>, token: &str) -> Box<dyn RequestBuilder> {
        todo!()
    }

    fn send(self: Box<Self>) -> BoxFuture<'static, Result<Box<dyn Response>, Error>> {
        todo!()
    }

    fn send_and_expect_200(
        self: Box<Self>,
    ) -> BoxFuture<'static, Result<Box<dyn Response>, Error>> {
        todo!()
    }

    fn json(
        self: Box<Self>,
        body: &dyn std::fmt::Display,
    ) -> Result<Box<dyn RequestBuilder>, Cow<'static, str>> {
        todo!()
    }

    // fn query(self: Box<Self>, params: &[(&str, &str)]) -> Box<dyn RequestBuilder> {
    //     todo!()
    // }
}
