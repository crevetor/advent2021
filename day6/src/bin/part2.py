def process(myfish):
    fish = [0 for i in range(9)]
    for f in myfish:
        fish[f] += 1
    for d in range(0, 256):
        numspawning = fish[0]
        for i in range(1,9):
            fish[i-1] = fish[i]
        fish[6] += numspawning
        fish[8] = numspawning
    print(f"{sum(fish)}")

with open("inputs/prod") as fd:
    process(map(lambda x: int(x), fd.readlines()[0].strip().split(',')))
