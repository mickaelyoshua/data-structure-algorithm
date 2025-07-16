# Induction and Recursion

As the book says, recursion is basically induction in action.

Induction is a form of prove the correctness of and algorithm to a infinite number of samples, from a finite number of samples.
It can work with only two steps:
1. A starting point
2. A rule of succession

E.g.: The Domino Effect
* **A starting point:** The first domino must be knocked over. This is the initial push that starts the entire process.
* **A rule of succession:** The dominoes must be spaced correctly. This means that for any domino in the line, if it falls, it is guaranteed to knock over its immediate successor.

For recursion we have the same principle: Something dealing with the first values, the starting point, and a condition to make the succession.

E.g.: Fibonacci Recursion:
```go
func FibonacciRecursion(n int) int {
	if n <= 1 { // Starting Point
		return n
	}
	return FibonacciRecursion(n-1) + FibonacciRecursion(n-2) // Rule of Succession
}
```

# Combinatorial Objects
Most of the algorithms are designed to work on specific abstract structures:
* **Permutation:** Are arrangements or ordering of items. The permutation of a set are different ways of organizing it's elements. E.g.: {1, 4, 3, 2} and {4, 3, 2, 1} are permutations of the same set
* **Subsets:** Represent a section, a small set, of an existing set. E.g.: {1, 3, 4} and {2} are two distinct subsets of the first four integers
* **Trees:** Represent hierarchical relationship between items. E.g.: A family tree
* **Graphs:** Represent relationships between arbitrary pairs of objects. E.g.: A network of road between points on a map
* **Points:** Define a location on a geometric space. E.g.: Latitude and longitude
* **Plygons:** Define regions in some geometric spaces. E.g.: The borders of a country
* **Strings:** Sequence of characters or patterns. E.g.: A personal name

# Recurcive Objects
Learning to thing recursively iss learning to look for big things that are mad of small things from the same type. Each abstract structure can be described on a recursive way.
* **Permutation:** Delete the first element of a permutation of *n* things {1,...,n} and you end up with a permutation of *n-1* things {1,...,n-1}
* **Subsets:** Every subset of {1,...,n} contains a subset of {1,...,n-1}
* **Trees:** Delete the root of a tree and we get a set of small trees
* **Graphs:** Delete any vertex of a graph and you get a smaller graph
* **Points:** Take a cloud of points and separate them with a line, now we have two smaller clouds of points
* **Plygons:** Inserting any internal line between two non-adjacent vertices on a polygon result on two smaller polygons
* **Strings:** Delete the first character of a string and end up with a smaller string


