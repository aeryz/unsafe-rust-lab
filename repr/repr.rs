use std::mem;

struct Foo<T, U> {
    count: u16,
    data1: T,
    data2: U,
}

fn main() {
    let foo1: Foo<u16, u32> = Foo {
        count: 1,
        data1: 1,
        data2: 1,
    };
    let foo2: Foo<u32, u16> = Foo {
        count: 1,
        data1: 1,
        data2: 1,
    };

    println!("{} {}", mem::size_of_val(&foo1), mem::size_of_val(&foo2));
}
