# Rust Calculator Assignment

## Objective

The objective of this assignment is to build a simple calculator that can perform basic arithmetic operations like addition, subtraction, multiplication, and division using Rust programming language concepts.

## Requirements

1. Your Rust program should take in two numbers and an operation from the user as input.
2. The program should prompt the user to enter the first number and then read in the user's input as a string using Rust's standard library. You should then use Rust's parse method to convert the string to a number. Repeat this process for the second number and the operation.
3. You should define a function that takes the two numbers and the operation as input, and returns the result of the operation. You can use Rust's match expression to handle different types of operations (e.g., +, -, *, /).
4. Call the function you defined in step 3 with the two numbers and the operation, and print the result to the console using Rust's `println!` macro.
5. You can add error handling by checking that the user inputs are valid numbers and that the operation is one of the supported operators. You can use Rust's `Result` type to handle errors and Rust's `unwrap` method to panic if an error occurs.

## Implementation Guidelines

1. Use Rust's built-in `f32` or `f64` data types to store the two numbers, depending on the level of precision you need.
2. For the operation, you can use Rust's `String` data type to store the user's input as a string. You can then use a `match` expression to convert the string to the corresponding operator.
3. You can use Rust's standard library to get user input from the console. Use the `println!` macro to prompt the user to enter the first number, and then use the `read_line` method to read in the user's input as a string. You can then use Rust's `parse` method to convert the string to a number. Repeat this process for the second number and the operation.
4. Define a function that takes the two numbers and the operation as input, and returns the result of the operation. You can use Rust's `match` expression to handle different types of operations (e.g., +, -, *, /).
5. Call the function you defined in step 4 with the two numbers and the operation, and print the result to the console using Rust's `println!` macro.
6. Add error handling by checking that the user inputs are valid numbers and that the operation is one of the supported operators. You can use Rust's `Result` type to handle errors and Rust's `unwrap` method to panic if an error occurs.


## Deliverables
You should submit your source code for the Rust Calculator assignment as a single Rust file. You should include comments in your code to explain what each part of the code does. You should submit your code in a Git repository along with a README file that includes instructions on how to run your program.

## Grading Criteria
Your Rust Calculator assignment will be graded based on the following criteria:
1. The program should be able to take two numbers and an operation as input and return the correct result.
2. The code should be well-organized, easy to read, and use meaningful variable names.
3. The code should include error handling to prevent crashes and handle invalid inputs.
4. The README file should provide clear instructions on how to run the program.

## Conclusion
Building a Rust Calculator is a great way to practice and reinforce the concepts learned in Chapter 3 of The Rust Programming Language book. By completing this assignment, you will gain a deeper understanding of Rust's data types, functions, error handling, and input/output. This assignment will also help you improve your problem-solving and programming skills. Remember to submit your completed assignment before


