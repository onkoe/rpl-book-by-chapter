# Word Frequency Counter

## Objective
The objective of this project is to create a program that takes a text file as input and outputs the frequency of each word in the file. ignore case and punctuation marks when counting words.

## Requirements
The program should:
- Take a text file as input
- Output the frequency of each word in the file
- Ignore case when counting words
- Ignore punctuation marks when counting words

## Implementation Guidelines
- Use the `std::env` module to handle command-line arguments
- Use the `std::fs` module to read the input file
- Use a `Vec` containing `tuple`s to store the words and their frequencies
- Use regular expressions to split the input text into words

## Deliverables
- A command-line program that takes a text file as input and outputs the frequency of each word in the file

## Grading Criteria
The program should:
- Correctly count the frequency of each word in the input file
- Ignore case and punctuation marks when counting words
- Be well-documented and easy to follow

## Conclusion
In this project, you have learned how to use common programming concepts such as command-line arguments, file input/output, regular expressions, and `Vec` to implement a simple but useful word frequency counting program. This project can be extended to perform more advanced text analysis tasks such as sentiment analysis, topic modeling, and natural language processing.
