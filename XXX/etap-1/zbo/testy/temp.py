import random

# n = random.randint(10001, 100000)
# k = random.randint(2, n)

n = 10

for i in range(n - 1):
    print(f"{random.randint(1, n)} {random.randint(1, n)} {random.randint(1, 10)}")
