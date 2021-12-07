











with open('in2.txt', 'r') as f:
    data = [int(i) for i in f.readlines()]


# for part1
c = 0
for i in range(1, len(data)):
    if data[i-1] < data[i]:
        c += 1

print(c)
    


# for part2
sliding_sum = [sum(i) for i in zip(data, data[1:], data[2:])]
print([i < j for i, j in zip(sliding_sum, sliding_sum[1:])].count(True))

