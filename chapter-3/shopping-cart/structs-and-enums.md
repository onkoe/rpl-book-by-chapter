# Structs and Enums in Rust

## Structs
A `struct` is a way to group data of different types under a single name. It is similar to a class in other programming languages. To define a `struct` in Rust, you use the `struct` keyword followed by the name of the struct and the fields inside curly braces:

```
struct Person {
name: String,
age: u32,
}
```

In the example above, we define a `Person` struct with two fields: `name` and `age`. We can create an instance of this struct and access its fields like this:

```
let person = Person {
name: String::from("Alice"),
age: 30,
};

println!("Name: {}, Age: {}", person.name, person.age);
```

Here, we create a `Person` instance with a name of "Alice" and an age of 30, and then we print out the values of its fields.

## Enums
An `enum` is a way to define a type by enumerating its possible values. It is similar to an algebraic data type in other programming languages. To define an `enum` in Rust, you use the `enum` keyword followed by the name of the enum and the possible values inside curly braces:

```
enum Color {
Red,
Green,
Blue,
}
```

In the example above, we define a `Color` enum with three possible values: `Red`, `Green`, and `Blue`. We can create an instance of this enum like this:

`let color = Color::Green;`

Here, we create a `Color` instance with a value of `Green`.

Enums can also have associated data with each variant. For example, we can modify the `Color` enum to associate an RGB value with each color:

```
enum Color {
Red(u8, u8, u8),
Green(u8, u8, u8),
Blue(u8, u8, u8),
}
```

Here, we create a `Color` instance with a value of `Green` and an RGB value of `(0, 255, 0)`.

## Conclusion
In this brief explanation, you have learned how to define and use structs and enums in Rust. Structs and enums are powerful tools for organizing and modeling data in your programs.
