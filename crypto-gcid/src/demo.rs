struct Item;

struct P<T> {
    ptr: Box<T>,
}

impl<T> P<T> {
    fn new(v: T) -> Self {
        P { ptr: Box::new(v) }
    }
}

impl<T> std::opts::Deref for P<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.ptr
    }
}

fn main () {
    let mut item = P::new(Item);
    *item;
}
