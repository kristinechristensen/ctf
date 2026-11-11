# Rust: Who Owns the String? — Instructor Solution

**Correct flag:** `CTF{sensor}`

## How to solve

Change fn show_label(label: String) to fn show_label(label: &String) (or &str), and call show_label(&label). The program then prints sensor twice, and the requested word is sensor.

## Learning objective

Recognize ownership transfer by move and borrowed references.

## Suggested tools

Rust compiler or code reasoning

## Teaching note

Have learners describe the evidence they used, not just submit the flag. The attached artifact is fictional and designed for an isolated classroom exercise.
