import math
import sys

DISTANCE = 4.5
SECONDS_IN_HOUR = 3600

stdin = sys.stdin

N = int(stdin.next().strip())
while N > 0:
    times = []
    for i in range(0, N):
        l = map(int, stdin.next().strip().split('\t'))
        vi, ti = l[0], l[1]
        if ti >= 0:
            time = (DISTANCE / vi) * SECONDS_IN_HOUR + ti
            times.append(time)

    print int(math.ceil(min(times)))

    N = int(stdin.next().strip())
