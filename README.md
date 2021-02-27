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
