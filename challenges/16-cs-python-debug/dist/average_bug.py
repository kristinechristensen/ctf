grades = [81, 90, 95]
# BUG: this performs floor division, discarding the fractional part.
average = sum(grades) // len(grades)
print(f"{average:.1f}")
