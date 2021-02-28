# Rustomonicon

# 1. Meet Safe and Unsafe

## 1.1 How Safe and Unsafe Interact

Safe code relies on the unsafe code and trusts that unsafe code is implemented correctly.
But unsafe code should not trust safe code to be correct. For example, `Ord` trait might be
implemented incorrectly by some type `T` and this might break the unsafe code in `BTreeMap`.
So `BTreeMap` must deal with this. But if incorrectness in a safe code cannot be detected, it is
marked as unsafe, like `Send`, `Sync` and `GlobalAllocator`.

## 1.2 What Unsafe Can Do

- Dereference raw pointers
- Call `unsafe` functions
- Implement `unsafe` traits
- Mutate statics
- Access fields of `union`s

# 2. Data Layout

## 2.1 repr(Rust)

By default, composite structure have an alignment equal to the maximum of their fields' alignments. Eg.
```rust
struct A {
    a: u8,
    b: u32,
    c: u16
}
```
`A` will be 32-bit aligned.

Rust does not guarantee `A` and `B` have their data laid out in exactly the same way. (Padding or ordering might be different)

```rust
struct A {
    a: i32,
    b: u64
}

struct B {
    a: i32,
    b: u64
}
```

## 2.2 Exotically Sized Types

### Dynamically Sized Types

Because they lack a statically known size, these types can only exist behind a pointer.

Example: `[T]`, `str` and `dyn MyTrait`.

### Zero Sized Types

Types that have no size occupies no space.

### Empty Types

```rust
enum Void {}
```

This can be used like `Result<T, Void>` where you must return a `Result` but it is guaranteed that
any error won't occur.

## 2.3 Other reprs

### repr(C)

The order, size, and alignment of fields is exactly what you would expect from C or C++.

### repr(transparent)

This can only be used on struct with a single non-zero-sized field. The effect is that the layout and ABI of
the whole struct is guaranteed to be the same as that one field.

### repr(u\*), repr(i\*)

These specify the size to make fieldless enum (C-like enums). Only works on fieldless enums.

### repr(packed)

Strip any padding, and only align the type to a byte. This will like have negative side-effects and can cause undefined behavior.

### repr(align(n))

Force the type to have an alignment of at least `n`.

# 3. Ownership

## 3.5 Lifetime Elision

- Each elided lifetime in input position becomes a distinct lifetime parameter.

- If there is exactly one input lifetime poisition (elided or not), that lifetime is assigned to all elided output lifetimes.

- If there are multiple input lifetime positions, but one of them is `&self` or `&mut self`, the lifetime of `self` is assigned to all elided output lifetimes.

- Otherwise, it is an error to elide an output lifetime.

## 3.6 Unbounded Lifetimes

Example, dereferencing a raw pointer. Such a lifetime becomes as big as context demands.

## 3.7 Higher-Rank Trait Bounds

```rust
struct Closure<F> {
    func: F
}

impl<F> Closure<F>
where for<'a> F: Fn(&'a (u8, u16)) -> &'a u8,
{
    fn call<'a>(&'a self) -> &'a u8 {
        (self.func)(&self.data)
    }
}
```

## 3.8 Subtyping and Variance

### Variance

- `F` is covariant if `F<Sub>` is a subtype of `F<Super>` (subtyping "passes through")
- `F` is contravariant if `F<Super>` is a subtype of `F<Sub>` (subtyping is "inverted")
- `F` is invariant otherwise (no subtyping relationship exists)

```rust
              'a          T               U
-----------------------------------------------------
&'a T         covariant   covariant       -
&'a mut T     covariant   invariant       -
Box<T>        -           covariant       -
Vec<T>        -           covariant       -
UnsafeCell<T> -           invariant       -
Cell<T>       -           invariant       -
fn(T) -> U    -           contravariant   covariant
*const T      -           covariant       -
*mut T        -           invariant       -
```
## 3.9 Drop Check

### may_dangle

```rust
unsafe impl<#[may_dangle] 'a> Drop for Inspector<'a> {
    fn drop(&mut self) {
        println!("... {}", self.1);
    }
}
```

- When the order of drop is important, it is best to use `ManuallyDrop`.

## 3.10 PhantomData

If `Vec` would defined like this:
```rust
struct Vec<T> {
    data: *const T,
    len: usize,
    cap: usize,
}
```

The drop checker will determine that `Vec<T>` does not own any values of type T. This will make it conclude that it doesn't need to worry about Vec dropping any T's in its destructor for determining drop check soundness.

To strictly tell the drop checker that `Vec<T>` owns T, we use `PhantomData`.

```rust
struct Vec<T> {
    data: *const T, // *const for variance
    len: usize,
    cap: usize,
    _marker: marker::PhantomData<T>,
}
```
