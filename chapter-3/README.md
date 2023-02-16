# Rust Programming Language Book - Chapter 3
While you should read the [offical Rust book's](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html) chapter 3 for free, this README provides an overview of the book's coverage. 

3.1: Variables and Mutability
-----------------------------

**Introduction:**  
Rust allows you to declare variables and set them to different values.  
Variables are either immutable (cannot change their value once assigned) or mutable (can change their value).

**Declaring Variables:**  
Variables are declared using the `let` keyword, followed by the name of the variable and its value, e.g. `let x = 5;`  
Variables can be type annotated with the type following a colon, e.g. `let x: i32 = 5;`

**Mutability:**  
By default, variables are immutable, meaning their value cannot be changed after they are assigned.  
To make a variable mutable, you can use the `mut` keyword, e.g. `let mut x = 5;`

**Shadowing:**  
Rust allows you to re-declare a variable with the same name in the same scope. This "shadows" the original declaration, making it inaccessible.  
Shadowing can be useful for changing the type of a variable, or for resetting its value to a new one.

**Constants:**  
In addition to variables, Rust also has constants, which are values that are known at compile-time and cannot be changed during runtime.  
Constants are declared using the `const` keyword and the type must be annotated, e.g. `const MAX_POINTS: u32 = 100_000;`

**Conclusion:**  
Variables and constants are a basic way to store values in Rust.  
By default, variables are immutable, but they can be made mutable with the `mut` keyword.  
Constants are values that are known at compile-time and cannot be changed during runtime.

* * *

3.2: Data Types
---------------

**Introduction:**  
Rust has several built-in data types that can be used to store different types of values.  
The data type of a value determines how much space it will take up in memory and how it will be stored.

### Scalar Types:

Scalar types represent a single value. Rust has four primary scalar types:

*   Integer types (i8, i16, i32, i64, u8, u16, u32, u64)
*   Floating-point types (f32, f64)
*   Boolean type (bool)
*   Character type (char)

### Compound Types:

Compound types can group multiple values into one type. Rust has two primary compound types:

*   Tuples (group values of different types together)
*   Arrays (group values of the same type together)

### Determining the Type of a Value:

The type of a value can be determined at compile-time using type inference.  
You can also explicitly annotate the type of a value using a type annotation.

### Type Conversion:

Rust has two types of type conversions:

*   Implicit conversion (also known as "implicit casting") occurs automatically when you assign a value of one type to a value of another type, e.g. `let i : i32 = 5; let f: f32 = i;`
*   Explicit conversion (or "explicit casting") occurs when you use a conversion function to convert a value of one type to another type, e.g. `let i: i32 = 5; let f: f32 = i as f32;`

**Conclusion:**  
Rust has several built-in data types that can be used to store different types of values.  
Scalar types represent a single value and include integers, floating-point numbers, booleans, and characters.  
Compound types can group multiple values into one type, including tuples and arrays.  
The type of a value can be determined at compile-time using type inference or through explicit type annotations.  
Rust has two types of type conversions: implicit conversion and explicit conversion.

3.3: Functions
--------------

### Introduction

*   Functions are blocks of code that perform a specific task and can be reused throughout your program.
*   Functions in Rust are declared using the `fn` keyword, followed by the name of the function, its parameters (enclosed in parentheses), and the body of the function (enclosed in curly braces).

### Function Parameters

*   Function parameters are defined within the parentheses of a function declaration.
*   Each parameter has a type and a name, separated by a colon.
*   You can define multiple parameters for a function, separated by commas.

### Return Values

*   Functions can return values to the caller using the `return` keyword.
*   The return type of a function can be explicitly annotated following an arrow (->) after the parameter list. If no return type is specified, Rust will infer the return type based on the values returned in the function body.

### Statements and Expressions

*   Rust has two types of code blocks: statements and expressions.
*   Statements are instructions that perform some action and do not return a value.
*   Expressions evaluate to a value and can be used in place of a statement.

### Functions with Return Values

*   Functions can return values to the caller using the `return` keyword.
*   The type of the returned value must match the annotated return type of the function, or be implicitly convertible to it.

### Conclusion

*   Functions are blocks of code that perform a specific task and can be reused throughout your program.
*   Functions in Rust are declared using the `fn` keyword and can have parameters and return values.
*   Rust has two types of code blocks: statements and expressions. Expressions evaluate to a value and can be used in place of a statement.
*   Functions can return values to the caller using the `return` keyword, and the type of the returned value must match the annotated return type of the function.

3.4: Comments
-------------

### Introduction

*   Comments are notes written in the source code that are ignored by the compiler and are intended for human readers.
*   Comments are used to explain the purpose of code, provide context, or leave notes for other developers.

### Single-Line Comments

*   Single-line comments start with two slashes (//) and continue until the end of the line.
*   Single-line comments can appear anywhere in the code and do not need to be closed.

### Multi-Line Comments

*   Multi-line comments start with a slash and an asterisk (/\*) and continue until an asterisk and a slash (\*/) are encountered.
*   Multi-line comments can span multiple lines and can be used to provide longer explanations of code.

### Conclusion

*   Comments are notes written in the source code that are ignored by the compiler and are intended for human readers.
*   Rust supports two types of comments: single-line comments, which start with two slashes (//), and multi-line comments, which start with a slash and an asterisk (/\*) and continue until an asterisk and a slash (\*/) are encountered.
*   Comments are used to explain the purpose of code, provide context, or leave notes for other developers.

3.5: Control Flow
-----------------

### Introduction

*   Control flow refers to the order in which statements in a program are executed.
*   Rust provides several control flow constructs for controlling the flow of execution in your program.

### Conditional Statements

*   Rust has an `if` expression that allows you to execute a block of code only if a certain condition is true.
*   The condition is specified in parentheses after the `if` keyword and the code to be executed is specified in a block following the condition.
*   You can use an `else` clause to specify an alternative block of code to be executed if the condition is false.

### Loops

*   Rust provides two types of loops: `loop` and `while`.
*   The `loop` construct creates an infinite loop that can be terminated using the `break` keyword.
*   The `while` loop executes a block of code while a certain condition is true.

### for Loops

*   Rust provides a `for` loop that allows you to iterate over a range of values.
*   The `for` loop takes an index variable and a range, specified using the `in` keyword.
*   The loop will execute once for each value in the range, and the index variable will be updated with the current value of the range on each iteration.

### Conclusion

*   Control flow refers to the order in which statements in a program are executed. 
*   Rust provides several control flow constructs for controlling the flow of execution in your program, including if expressions, loop and while loops, and for loops. 
*   The `if` expression allows you to execute a block of code only if a certain condition is true, and the else clause can be used to specify an alternative block of code to be executed if the condition is false. 
*   The `loop` and `while` loops allow you to repeatedly execute a block of code, and the `for` loop allows you to iterate over a range of values. 
*   Additionally, implicit conversions and explicit conversions can be used to convert between types.