[//]: # (TODO: AI GENERATED TEMPLATE, REMOVE ONCE LESSON IMPLEMENTATION IS COMPLETE)

# Variables & Mutability

In Rust, variables are **immutable by default**. Once you bind a value to
a name, you can't change it:

```rust
let x = 5;
x = 6; // error: cannot assign twice to immutable variable
```

This isn't a limitation — it's a guarantee. When you see a `let` binding
in Rust code, you know that value never changes underneath you, which
makes code easier to reason about, especially as programs grow.

## Making a variable mutable

When you *do* need a value to change, opt in with `mut`:

```rust
let mut x = 5;
println!("x is: {x}");
x = 6;
println!("x is now: {x}");
```

That single keyword is doing real work: it's a signal, both to the
compiler and to anyone reading your code, that this value is expected to
change. Everything without `mut` is a promise that it won't.

## Why this matters

In many languages, "can this change?" is a question you answer by reading
carefully through the surrounding code. In Rust, it's answered at the
declaration site. This becomes especially valuable once you're passing
values between functions or sharing them across parts of a program —
you'll cover that when we get to ownership and borrowing.

## Try it

Use the editor on the right to:

1. Declare a mutable variable and print it.
2. Reassign it to a new value.
3. Print it again to show the change.

Run **Validate** when you're ready to check your output.