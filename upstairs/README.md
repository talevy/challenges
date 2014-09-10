Upstairs [[1](https://www.hackerrank.com/contests/addepar/challenges/upstairs)]
==================

Joe goes to school in a very tall building. He attends `N` classes during the day,
numbered chronologically from `1` through `N`. Class `i` is located on floor `a_i`.
Joe's schedule allows him to switch the order of two consecutive classes.
Since Joe does not like to walk up stairs, he wants to choose the schedule that
requires him to walk up the least number of stairs throughout the day (each
pair of consecutive floors is separated by the same number of stairs).
Help Joe determine which classes to switch to minimize the number of stairs
he must walk up during the school day, assuming he starts counting at the
beginning of his first class.


#####Input Format

The first line of the input contains a single integer N. The second line
of the input contains N space-separated integers a1,a2,…,aN.

#####Output Format

A single integer `f`, if it is optimal for Joe to switch classes `f` and `f+1`.
If switching no classes is at least as good as the best possible switch, output `−1`.
Otherwise, if there are multiple possible `f`, output the smallest one.

##### Constraints.

1 <= `N` <= 100000, 1 <= `a_i` <= 1000.

#####Sample Input

```
4
5 3 5 2
```

#####Sample Output

```
2
```
