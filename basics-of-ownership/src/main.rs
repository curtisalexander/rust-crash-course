#[derive(Debug, Clone)]
struct Foobar(i32);

impl Copy for Foobar {}

impl Foobar {
    fn use_it(&self) {
        println!("I consumed a Foobar: {:?}", &self);
    }
}

fn double(foobar: Foobar) -> Foobar {
    Foobar(foobar.0 * 2)
}
// impl Drop for Foobar {
//    fn drop(&mut self) {
//        println!("Dropping a Foobar: {:?}", self);
//    }
//}

fn uses_foobar(foobar: Foobar) {
    println!("I consumed a Foobar: {:?}", foobar);
}

fn main() {
    let x: Foobar = Foobar(1);
    // let y: &Foobar = &x;
    println!("Before uses_foobar");
    x.use_it();
    // uses_foobar(y);
    println!("After uses_foobar");
    uses_foobar(x);
    let z: Foobar = double(x);
    println!("{}", z.0)
}
