numbers = [4, 7, 2, 9]
score = 0

for index, number in enumerate(numbers):
    if number % 2 == 0:
        score += number * 2
    else:
        score += index

print(score)
