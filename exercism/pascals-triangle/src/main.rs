fn main() {
    let iter = vec![1,2,3];
    let iter = std::iter::once(1).chain(iter.iter());
    iter.for_each(|x| println!("{}", x));
}
