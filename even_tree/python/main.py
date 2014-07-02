from __future__ import print_function
import collections
import sys

def update_counts(root, nodes, counts):
    for c in nodes[root]:
        update_counts(c, nodes, counts)
    counts[root] = sum([counts[c] for c in nodes[root]]) + 1

def cut_even_forest(root, nodes, counts):
    child_cut_count = 0
    for c in nodes[root]:
        if counts[c] % 2 == 0:
            child_cut_count += 1
        child_cut_count += cut_even_forest(c, nodes, counts)
    return child_cut_count

nodes = collections.defaultdict(list)
counts = collections.defaultdict(int)

for linenum, line in enumerate(sys.stdin):
    if linenum != 0:
        u, v = map(int, line.strip().split(' '))
        nodes[v].append(u)

update_counts(1, nodes, counts)

print(cut_even_forest(1, nodes, counts), end='')

