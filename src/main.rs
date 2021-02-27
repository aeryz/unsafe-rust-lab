struct Closure<F> {
    data: (u8, u16),
    func: F,
}

impl<F> Closure<F>
where
    F: Fn(&(u8, u16)) -> &u8,
{
    fn call<'a>(&'a self) -> &'a u8 {
        (self.func)(&self.data)
    }
}

fn do_it<'b>(data: &'b (u8, u16)) -> &'b u8 {
    &data.0
}

fn main() {
    let closure = Closure {
        data: (1, 1),
        func: do_it,
    };
    let a = closure.call();
    println!("{}", a);
}
