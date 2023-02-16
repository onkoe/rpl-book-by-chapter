# Hangman

In this project, we'll create a simple command-line version of the classic word-guessing game, Hangman.

## Objective

Create a command-line game that randomly picks a word from a dictionary file, and prompts the user to guess the word one letter at a time.

## Requirements

1. The game should pick a random word from a dictionary file. The dictionary file should be provided as a command-line argument.
2. The game should display the length of the word as underscores `_` and the user should guess letters to fill in the blanks.
- For example, if the word is "apple" and the correctly guessed letters are 'p' and 'e', display _ p p _ e.
3. The user should have 7 chances to guess the word before the game ends.
4. After each guess, the game should display the word with the correctly guessed letters filled in.
5. The game should display a stick figure to indicate the number of remaining guesses.
6. The game should end when the user guesses the word correctly, or when the user has no remaining guesses.

Use the following print_stick_figure library function to display the stick figure:

```rust
use hangman_game::print_stick_figure;

// To print the full stick figure:
print_stick_figure(7);

// To print the stick figure with one part removed:
print_stick_figure(6);

// To print the stick figure with two parts removed:
print_stick_figure(5);

// etc.
```

# Implementation Guidelines

1. Parse the dictionary file provided as a command-line argument, and store its words in a collection. You can use a `vector` to store the words.
2. Use the rand crate to pick a random word from the collection.
3. Use 
5. After each guess, update the HashMap and HashSet accordingly.
6. Use the provided print_stick_figure function to display the stick figure.
7. After each guess, display the word with the correctly guessed letters filled in. For example, if the word is "apple" and the correctly guessed letters are 'p' and 'e', display _ p p _ e.
8. Use the std::io module to get input from the user.

Implementation Guidelines

Create a new string variable to hold the word to be guessed.
Create a new string variable to hold the current state of the word (underscores for unguessed letters, letters for guessed letters).
Keep track of the number of guesses remaining.
Create a loop that continues until the word is guessed or the player runs out of guesses.
In each iteration of the loop, display the current state of the word and the stick figure.
Ask the player for a guess, and validate that the input is a single character.
If the guess is correct, update the current state of the word with the guessed letter.
If the guess is incorrect, decrement the number of guesses remaining and display the updated stick figure.
If the player has no more guesses, end the game and display a message indicating that they have lost.
If the player has guessed the entire word, end the game and display a message indicating that they have won.

## Deliverables

A command-line game that meets the requirements listed above. Make a Hangman game whcih

## Grading Criteria

The game should run without errors.
The game should correctly display the length of the word, and the correctly guessed letters.
The game should use the provided print_stick_figure function to display the stick figure.
The game should end when the user guesses the word correctly, or when the user has no remaining guesses.
The code should be well-documented and easy to understand.

Conclusion

Have fun building the game! Don't hesitate to ask questions if you get stuck.