Even Tree [[1](https://www.hackerrank.com/challenges/even-tree)]
=========

You are given a tree (a simple connected graph with no cycles). You have to
remove as many edges from the tree as possible to obtain
a [forest](http://en.wikipedia.org/wiki/Tree_%28graph_theory%29) with the
condition that : Each connected component of the forest should contain an even
number of vertices.

Your task is to calculate the number of removed edges in such a forest.

#####Input Format 

The first line of input contains two integers N and M. N is the number of
vertices and M is the number of edges. 
The next M lines contain two integers ui and vi which specifies an edge of the
tree. (1-based index)

#####Output Format 

Print the answer, a single integer.

#####Constraints 
2 <= N <= 100.

_Note_: The tree in the input will be such that it can always be decomposed into
components containing even number of nodes.

#####Sample Input

```
10 9
2 1
3 1
4 3
5 2
6 1
7 2
8 6
9 8
10 8
```

#####Sample Output

```
2
```

#####Explanation  

On removing the edges (1, 3) and (1, 6), we can get the desired result.

Original tree:

![originaltree](http://linode.interviewstreet.com/eventree1.png)

Decomposed tree:

![decomposedtree](http://linode.interviewstreet.com/eventree2.png)
