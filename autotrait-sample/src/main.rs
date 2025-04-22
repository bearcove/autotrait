#[derive(Default)]
struct ModImpl;

#[autotrait::autotrait]
impl Mod for ModImpl {
    fn greet(&self) {
        println!("Hello, world!");
    }
}

fn main() {
    let m: Box<dyn Mod> = Box::new(ModImpl);
    m.greet();
}
