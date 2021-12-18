import sys

import numpy as np


def parse_input(filename):

    with open(filename) as fd:
        lines = [line.strip() for line in fd.readlines()]
    
    coords = []
    line = lines.pop(0)
    while line != "":
        coords.append(tuple(map(int, line.split(",", 1))))
        line = lines.pop(0)

    folds = []
    for line in lines:
        (direction, pos) = line.split(" ")[-1].split("=", 1)
        folds.append((direction, int(pos)))
    
    return (coords, folds)
    

def main():
    if len(sys.argv) != 2:
        print("Wrong number of args")
        sys.exit(1)
    
    (coords, folds) = parse_input(sys.argv[1])

    maxx = max(map(lambda x: x[0], coords))
    maxy = max(map(lambda x: x[1], coords))

    grid = np.zeros((maxy + 1, maxx + 1), int)
    for coord in coords:
        grid[coord[1], coord[0]] = 1

    print(grid)
    print('---')
    newgrid = grid
    for fold in folds:
        print(fold)
        flippedhalf = newgrid[::,fold[1] + 1:] if fold[0] == 'x' else newgrid[fold[1] + 1:, ::]
        flippedhalf = np.flip(flippedhalf, 0 if fold[0] == 'y' else 1)
        newgrid = newgrid[::,:fold[1]] if fold[0] == 'x' else newgrid[:fold[1],::]
        newgrid = newgrid|flippedhalf
    print(newgrid)

    for row in newgrid:
        line = ""
        for col in row:
            if col == 0:
                line += ' '
            else:
                line += '#'
        print(line)

if __name__ == "__main__":
    main()