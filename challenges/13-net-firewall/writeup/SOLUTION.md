# Firewall Rule Review — Instructor Solution

**Correct flag:** `CTF{2}`

## How to solve

Rule 2 matches source 10.20.30.45 in 10.20.30.0/24 and TCP destination port 443; the DROP is applied before rule 3.

## Learning objective

Evaluate first-match firewall rule ordering.

## Suggested tools

Spreadsheet application

## Teaching note

Have learners describe the evidence they used, not just submit the flag. The attached artifact is fictional and designed for an isolated classroom exercise.
