import sys

for line in sys.stdin:
    if line == "0.00\n": break;
    c = float(line.strip())
    (curr_n, curr_len) = (0, 0.0)
    while c > curr_len:
        curr_n += 1
        curr_len += 1.0/(1 + curr_n)
    print curr_n, "card(s)"

