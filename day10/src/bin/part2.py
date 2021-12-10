from functools import reduce
import sys

MATCHING = {
    '}': '{',
    ']': '[',
    ')': '(',
    '>': '<',
}

REVMATCHING = {
    '{': '}',
    '[': ']',
    '(': ')',
    '<': '>',
}

POINTS = {
    ')': 1,
    ']': 2,
    '}': 3,
    '>': 4,
}

def solve(lines):
    scores = []
    for line in lines:
        stack = []
        valid = True
        for c in line:
            if c in '{([<':
                stack.append(c)
            if c in '})]>':
                p = stack.pop()
                if MATCHING[c] != p:
                    valid = False
                    break
        if valid:
            missing = ''.join(map(lambda x: REVMATCHING[x], reversed(stack)))
            score = reduce(lambda acc, y: acc*5 + POINTS[y], missing, 0)
            scores.append(score)
            print(f"Missing : {missing}, score : {score}")

    print(sorted(scores)[len(scores)//2])

def main():
    if len(sys.argv) != 2:
        print("Wrong number of args")
        sys.exit(1)

    with open(sys.argv[1]) as fd:
        lines = fd.readlines()

    solve(lines)

if __name__ == "__main__":
    main()
