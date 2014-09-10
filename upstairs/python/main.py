import sys

def num_steps_climbed(floor_schedule):
    return sum(map(
	    lambda (x, y): y-x if y > x else 0,
	    zip(floor_schedule, floor_schedule[1:])))

def swap(i, floor_schedule):
	tmp = floor_schedule[i]
	floor_schedule[i] = floor_schedule[i+1]
	floor_schedule[i+1] = tmp

stdin = sys.stdin

N = int(stdin.next().strip())
floor_schedule = map(int, stdin.next().strip().split())

# Solve

min_steps = num_steps_climbed(floor_schedule)
f = -1

for i, a in enumerate(floor_schedule[:-1]):
    swap(i, floor_schedule)
    next_min = num_steps_climbed(floor_schedule)
    if next_min < min_steps:
        min_steps = next_min
	f = i + 1
    swap(i, floor_schedule)
 
print f
