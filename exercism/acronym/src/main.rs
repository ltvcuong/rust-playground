fn main() {}

pub trait T {
    fn ta(&self);
}

fn f(x: &dyn T) {
    x.ta();
}
