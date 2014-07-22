Erdos Numbers
[[1](http://uva.onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&page=show_problem&problem=985)]
==================

The Hungarian Paul Erdos (1913 - 1996) was one of the most famous
mathematicians of the 20th century. Every mathematician having the honor
of being a co-author of Erdos is well respected. Unfortunately, not
everybody got the chance to write a paper with Erdos, so the
best they could do was publish a paper with somebody who had published a
scientific paper with Erdos. This gave rise to the so-called Erdos numbers.
An author who has jointly published with Erdos had Erdos number 1.
An author who had not published with Erdos but with somebody with Erdos
number 1 obtained Erdos number 2, and so on.

Your task is to write a program which computes Erdos numbers for a
given set of papers and scientists.

#####Input Format

The first line of the input contains the number of scenarios.
The input for each scenario consists of a paper database and a list of names. It
begins with the line

P N

where P and N are natural numbers. Following this line are P lines containing
descriptions of papers (this is the paper database). A paper appears on a line
by itself and is specified in the following way:

Smith, M.N., Martin, G., Erdos, P.: Newtonian forms of prime factors matrices

Note that umlauts like `o` are simply written as `o`. After the P papers follow
N lines with names. Such a name line has the following format:

Martin, G.

#####Output Format

For every scenario you are to print a line containing a string "Scenario"
(where i is the number of the scenario) and the author names together with
their Erdos number of all authors in the list of names. The authors should
appear in the same order as they appear in the list of names. The Erdos
number is based on the papers in the paper database of this scenario.
Authors which do not have any relation to Erdos via the papers in the
database have Erdos number "infinity"

#####Sample Input

```
1
4 3
Smith, M.N., Martin, G., Erdos, P.: Newtonian forms of prime factor matrices 
Erdos, P., Reisig, W.: Stuttering in petri nets
Smith, M.N., Chen, X.: First oder derivates in structured programming
Jablonski, T., Hsueh, Z.: Selfstabilizing data structures
Smith, M.N.
Hsueh, Z.
Chen, X.
```

#####Sample Output

```
Scenario 1
Smith, M.N. 1
Hsueh, Z. infinity
Chen, X. 2
```
