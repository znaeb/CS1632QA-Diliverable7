# CS 1632 - Software Quality Assurance
Summer Semester 2016

DUE 3 August 2016

## Deliverable 7

For this assignment, groups of two will implement a game of Rock-Paper-Scissors in Rust.

You may work on this assignment alone if you wish, but the same requirements apply.

## Functional Requirements

1. The game shall accept the letters r, p, s, and q as input.  It shall be case-sensitive (that is, "R" is an invalid option).  All other input shall be regarded as invalid and ignored; the game should let the user know the possible options and ask for more input.
2. The game shall exit and display statistics (as described below) when the player selects the option "q".
3. Input from the player shall be interpreted as follows: "r" for Rock, "p" for Paper, "s" for Scissors.
4. After the player selects a valid, non-quitting option ("r", "p", or "s"), the game shall pseudorandomly select a response (Rock, Paper, or Scissors) and compare the two using the following algorithm:
  1. Paper beats Rock.
  2. Scissors beats Paper.
  3. Rock beats Scissors.
  4. If both the player and the game have the same choice, then the round is a tie.
5. The game shall display what the player selected and what the game selected, as well as the results.
5. If the player selects to quit, the game shall display the statistics of the player.  These will include:
  1. Number and percentage of wins
  2. Number and percentage of ties
  3. Number and percentage of losses
  4. Number of rocks thrown
  5. Number of scissors thrown
  6. Number of papers thrown
6. The percentage shall be construed as the percentage of time that result occurred vs the number of rounds played.  For example, if a player plays two rounds, and wins one and loses one, then the corresponding percentages are:
  1. Win: 50.00%
  2. Loss: 50.00%
  3. Tie: 0.00%
7. The percentage shall be displayed to two decimal points (e.g, `23.45`, not `23.450176540`).
8. The percentage shall always be a number (that is, do not display NaNs - these should be display as "0.00").
8. The prompt should not be on the same line as the input.  (That is, use println! and not print! for the prompt.)
9. In the case of questions, the sample_output.txt file shall be considered the truth copy.  Any result is shows is valid.

Sample output is available in the sample_output.txt file in this directory.

## Implementation Requirements

You must have at least four (4) functions besides main().

At least one function must use borrowing.

You must use at least one struct.

You must use at least one enum.

## Format
Every assignment should have a title page with:
* The name of the students in the group
* The title "CS 1632 - DELIVERABLE 7: Rock-Paper-Scissors in Rust"
* The URL for your git repository

There is no need to print out the code.  It should be available on GitHub or a similar code-sharing site BY THE BEGINNING OF CLASS.


## Grading
* Summary - 10%
* Functional requirements met - 45%
* Implementation requirements met - 30%
* No compiler errors OR warnings - 15%

Extra credit is available on this assignment, for up to +15 points.  The amount of extra credit is subjective based on the amount of functionality added.  Possible extra credit options:

1. Player data is saved and loaded to a file after exiting/starting
2. A simple "AI" which attempts to make "better" guesses based on the player's previous choices
3. Proper unit tests on functions
4. If you have other ideas, please email me asking for permission before implementing them.

If you do extra credit, you must mention it in the summary.

## Notes

Please feel free to email me or come to office hours to discuss any problems you have. 
 
