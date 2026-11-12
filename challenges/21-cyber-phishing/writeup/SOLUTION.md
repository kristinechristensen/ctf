# Instructor Solution: Phishing Email Investigation

**Flag:** `CTF{verify-account.security-check.example}`

## Explanation

The message uses urgency, fails SPF, has no DKIM signature, and links to `verify-account.security-check.example` rather than the institutional support hostname. The exact hostname from the login link is the flag value. The training domains are fictitious.

## Discussion prompt

What additional evidence would you request before deciding whether this represents a confirmed security incident?
