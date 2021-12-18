import sys
import copy

class Node:

    def __init__(self, name):
        self.name = name
        self.neighbors = set()

    def visit(self, visited):
        if self.name == "end":
            print(f"Path {visited + [self.name]}")
            return 1

        paths = 0
        if self.name.islower() and self.name in visited:
            return 0
        else:
            for neighbor in self.neighbors:
                paths += neighbor.visit(visited + [self.name])

        return paths

    def __repr__(self):
        return f"{self.name} {self.neighbors}"


def main():
    if len(sys.argv) != 2:
        print("Wrong number of args")
        sys.exit(1)

    nodes = {}
    with open(sys.argv[1]) as fd:
        for line in fd.readlines():
            (fromname, toname) = line.strip().split('-', 1)
            if fromname not in nodes:
                nodes[fromname] = Node(fromname)
            if toname not in nodes:
                nodes[toname] = Node(toname)

            nodes[fromname].neighbors.add(nodes[toname])
            nodes[toname].neighbors.add(nodes[fromname])

    paths = nodes["start"].visit([])
    print(f"Found {paths} paths")

if __name__ == "__main__":
    main()
