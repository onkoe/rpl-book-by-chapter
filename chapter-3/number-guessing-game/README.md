# Number Guessing Game

## Objective
The objective of this project is to create a number guessing game that generates a random number and allows the player to guess the number until they get it right.

## Requirements
The program should:
- Generate a random number between 1 and 100
- Prompt the player to enter a number
- Tell the player if their guess is too high, too low, or correct
- Keep track of the number of guesses the player makes
- Allow the player to play again

## Implementation Guidelines
- Use the `rand` crate to generate a random number
- Use the `std::io` module to handle user input
- Use a `loop` to allow the player to make multiple guesses
- Use conditional statements to check if the player's guess is too high, too low, or correct
- Use a `break` statement to exit the loop if the player guesses correctly
- Use the `std::cmp` module to compare the player's guess with the generated number
- Use a `bool` variable to allow the player to play again

## Deliverables
- A command-line program that generates a random number and allows the player to guess the number until they get it right

## Grading Criteria
The program should:
- Correctly generate a random number between 1 and 100
- Prompt the player to enter a number and provide feedback on whether the guess is too high, too low, or correct
- Keep track of the number of guesses the player makes
- Allow the player to play again
- Be well-documented and easy to follow

## Conclusion
In this project, you have learned how to use common programming concepts such as random number generation, user input/output, loops, conditional statements, and comparison operators to implement a simple but fun number guessing game. This project can be extended to add more complex features such as a time limit, score calculation, and leaderboard.
