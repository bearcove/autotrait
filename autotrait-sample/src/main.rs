#![allow(warnings)]

use std::{borrow::Cow, collections::HashMap};

use autotrait::autotrait;
use bytes::Bytes;
use futures_util::{future::BoxFuture, stream::BoxStream};
use http::{Error, HeaderName, HeaderValue, Method, StatusCode, Uri};

#[allow(dead_code)]
struct ClientOpts;

#[derive(Default)]
struct ModImpl;

#[autotrait]
impl Foo for ModImpl {
    fn make_watcher_1(&self, on_event: Box<dyn Fn()>) {
        todo!()
    }

    fn make_watcher_2(&self, on_event: Box<dyn Fn(u32)>) {
        todo!()
    }

    fn make_watcher_3(&self, on_event: Box<dyn Fn(u32) -> u64>) {
        todo!()
    }

    fn make_watcher_4(&self, on_event: Box<dyn Fn(u32) -> String + Send + Sync>) {
        todo!()
    }
}

#[autotrait(!Send)]
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

    fn slice_arg(&self, s: &[u8]) {
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

    fn has_lifetime<'fut>(&self) {
        todo!()
    }

    fn handle_oauth_callback<'fut>(
        &'fut self,
        tc: &'fut (),
        web: (),
        args: &'fut Cow<'_, str>,
    ) -> BoxFuture<'fut, Result<Option<Cow<'static, str>>, ()>> {
        Box::pin(async move { todo!() })
    }
}

#[allow(dead_code)]
struct HttpClientImpl {
    //
}

#[autotrait]
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

#[autotrait]
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

#[autotrait]
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

    fn query(self: Box<Self>, params: &[(&str, &str)]) -> Box<dyn RequestBuilder> {
        todo!()
    }
}

struct MultipartUploadWrapper {}

#[autotrait(!Sync)]
impl NotSync for MultipartUploadWrapper {}

#[autotrait(!Send)]
impl NotSend for MultipartUploadWrapper {}

struct SvgImpl {}

#[autotrait]
impl Svg for SvgImpl {
    fn inject_font_faces<'future>(
        &'future self,
        input: &'future [u8],
        font_faces: &'future u8,
    ) -> BoxFuture<'future, Result<Vec<u8>, ()>> {
        todo!()
    }
}

struct MutDynImpl {}

/// Here's a doc comment
#[autotrait]
impl MutDyn for MutDynImpl {
    fn render_math(&self, input: &str, mode: (), w: &mut dyn std::io::Write) -> eyre::Result<()> {
        todo!()
    }
}

struct MediaUploaderImpl {}

trait ChunkReceiver {}

#[autotrait]
impl MediaUploader for MediaUploaderImpl {
    fn done_and_download_result<'a>(
        &self,
        mut chunk_receiver: Box<dyn ChunkReceiver + 'a>,
    ) -> BoxFuture<'a, Result<(), ()>> {
        todo!()
    }
}

struct HasAsyncFnImpl;

#[autotrait]
impl HasAsyncFn for HasAsyncFnImpl {
    async fn async_fn(&self) -> Result<(), ()> {
        todo!()
    }
}

struct Bot;

type Message = ();
type RequestError = ();
type Router<T> = (T);
type AsyncError = ();
type Url = ();

#[autotrait]
impl BotExt for Bot {
    async fn reply(&self, message: &Message, text: &str) -> Result<Message, RequestError> {
        todo!()
    }

    async fn try_reply(&self, message: &Message, text: &str) -> Result<Message, RequestError> {
        todo!()
    }

    async fn try_reply_silent(
        &self,
        message: &Message,
        text: &str,
    ) -> Result<Message, RequestError> {
        todo!()
    }

    async fn replace_chat_message(
        &self,
        message: &Message,
        text: &str,
    ) -> Result<Message, RequestError> {
        todo!()
    }

    fn is_self_message(&self, message: &Message) -> bool {
        todo!()
    }

    async fn perform_replacement(
        &self,
        message: &Message,
        url_matcher: &Router<()>,
        preview_domain: &str,
        get_button_data: impl Fn(&Url) -> Option<(&str, Url)>,
    ) -> Result<(), AsyncError> {
        todo!()
    }
}
