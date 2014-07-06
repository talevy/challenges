import sys
from collections import deque

class Board:
    def __init__(self, leaps_dict, N=100):
        self.N = N
        self.leaps = leaps_dict

    def next_moves(self, idx):
        roll_moves = []
        for i in range(idx + 1, idx + 7):
            if i in self.leaps:
                roll_moves.append(self.leaps[i])
            elif i <= self.N:
                roll_moves.append(i)
        return roll_moves

    def solve(self):
        Q = set([0])
        num_rolls = 0
        while True:
            NQ = set()
            for curr_square in Q:
                if curr_square == self.N:
                    return num_rolls
                NQ |= set(self.next_moves(curr_square))
            Q = NQ
            num_rolls += 1

# grab input
stdin = sys.stdin
T = int(stdin.next().strip())  # number of test cases
for case in xrange(0, T):
    stdin.next()  # ignore num_ladders, num_snakes
    leap_lines = stdin.next().strip() + ' ' + stdin.next().strip()
    leaps_dict = dict([map(int, l.split(',')) for l in leap_lines.split(' ')])
    B = Board(leaps_dict)
    print B.solve()
