#[derive(Default)]
struct ModImpl;

#[autotrait::autotrait]
impl Mod for ModImpl {
    fn client(&self) -> Box<dyn HttpClient> {
        Box::new(HttpClientImpl::new(None))
    }

    fn client_with_opts(&self, opts: ClientOpts) -> Box<dyn HttpClient> {
        Box::new(HttpClientImpl::new(Some(opts)))
    }
}

fn main() {
    let m: Box<dyn Mod> = Box::new(ModImpl);
    m.greet();
}
