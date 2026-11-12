# Instructor Solution: Suspicious Web Request

**Flag:** `CTF{198.51.100.88}`

## Explanation

The request from `198.51.100.88` contains `..%2f..%2fetc%2fpasswd`, where `%2f` represents `/`, suggesting an attempted directory traversal. The 403 response indicates the request was denied, not that compromise occurred.

## Discussion prompt

What additional evidence would you request before deciding whether this represents a confirmed security incident?
