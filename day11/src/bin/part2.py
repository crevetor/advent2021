import sys

import numpy as np

class Octopus:
    
    def __init__(self, pos, level):
        self.pos = pos
        self.level = level
        self.flashed = False
        self.neighbors = []

    def add_neighbor(self, neighbor):
        self.neighbors.append(neighbor)

    def step(self):
        self.flashed = False
        self.level += 1

    @property
    def full(self):
        return self.level > 9
        
    def flash(self):
        self.flashed = True
        numflashes = 1
        print(f"{self} flashing")
        for neighbor in self.neighbors:
            if not neighbor.flashed:
                neighbor.level += 1
                if neighbor.full:
                    numflashes += neighbor.flash()
        self.level = 0
        return numflashes
    
    def __repr__(self):
        return f"({self.pos[0]}, {self.pos[1]}) {self.level}"

def main():
    if len(sys.argv) != 2:
        print("Wrong number of args")
        sys.exit(1)
    
    tmpgrid = []
    x = 0
    y = 0
    with open(sys.argv[1]) as fd:
        for line in fd.readlines():
            x=0
            row = []
            for c in line.strip():
                row.append(Octopus((x, y), int(c)))
                x += 1
            tmpgrid.append(row)
            y += 1
    
    grid = np.asarray(tmpgrid)
    print(grid)

    for oct in grid.flat:
        minx = max(oct.pos[0] - 1, 0)
        maxx = min(oct.pos[0] + 1, x)
        miny = max(oct.pos[1] - 1, 0)
        maxy = min(oct.pos[1] + 1, y)
        print(f"Neighbors of {oct} ({minx}:{maxx}, {miny}:{maxy})")
        for neighbor in grid[miny:maxy + 1, minx:maxx + 1].flat:
            if neighbor.pos != oct.pos:
                print(neighbor, end='')
                oct.add_neighbor(neighbor)
        print("")

    step = 0
    while True:
        flashes = 0
        for oct in grid.flat:
            if oct.full and not oct.flashed:
                print(f"Flashing {oct}")
                octflashes = oct.flash()
                print(f"{octflashes} flashes for {oct}")
                flashes += octflashes
        
        if flashes == len(grid.flat):
            print(f"Step {step}")
            break

        for oct in grid.flat:
            oct.step()
        step += 1
    
if __name__ == "__main__":
    main()