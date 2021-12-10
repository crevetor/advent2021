import sys

MATCHING = {
    '}': '{',
    ']': '[',
    ')': '(',
    '>': '<',
}

POINTS = {
    ')': 3,
    ']': 57,
    '}': 1197,
    '>': 25137,
}

def solve(lines):
    points = 0
    for line in lines:
        stack = []
        for c in line:
            if c in '{([<':
                stack.append(c)
            if c in '})]>':
                p = stack.pop()
                if MATCHING[c] != p:
                    print(f"Found {p} when expecting {MATCHING[c]}")
                    print(f"Invalid : {line}")
                    points += POINTS[c]
                    break
    print(points)

def main():
    if len(sys.argv) != 2:
        print("Wrong number of args")
        sys.exit(1)

    with open(sys.argv[1]) as fd:
        lines = fd.readlines()

    solve(lines)

if __name__ == "__main__":
    main()
