import sys

# skip first line
T = sys.stdin.next()
for line in sys.stdin:
    b_rev, a_rev = map(int, line.strip()[::-1].split(' '))
    print int(str(a_rev + b_rev)[::-1])
