# Introduction to Algorithm Design
## Induction and Recursion
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
```rust
fn fibonacci_recursion(n: u64) -> u64 {
    if n <= 1 { // starting point
        return n
    }
    fibonacci_recursion(n-1) + fibonacci_recursion(n-2) // rule of succession
}
```

## Combinatorial Objects
Most of the algorithms are designed to work on specific abstract structures:
* **Permutation:** Are arrangements or ordering of items. The permutation of a set are different ways of organizing it's elements. E.g.: {1, 4, 3, 2} and {4, 3, 2, 1} are permutations of the same set
* **Subsets:** Represent a section, a small set, of an existing set. E.g.: {1, 3, 4} and {2} are two distinct subsets of the first four integers
* **Trees:** Represent hierarchical relationship between items. E.g.: A family tree
* **Graphs:** Represent relationships between arbitrary pairs of objects. E.g.: A network of road between points on a map
* **Points:** Define a location on a geometric space. E.g.: Latitude and longitude
* **Plygons:** Define regions in some geometric spaces. E.g.: The borders of a country
* **Strings:** Sequence of characters or patterns. E.g.: A personal name

## Recursive Objects
Learning to do things recursively is learning to look for big things that are made of small things from the same type. Each abstract structure can be described on a recursive way.
* **Permutation:** Delete the first element of a permutation of *n* things {1,...,n} and you end up with a permutation of *n-1* things {1,...,n-1}
* **Subsets:** Every subset of {1,...,n} contains a subset of {1,...,n-1}
* **Trees:** Delete the root of a tree and we get a set of small trees
* **Graphs:** Delete any vertex of a graph and you get a smaller graph
* **Points:** Take a cloud of points and separate them with a line, now we have two smaller clouds of points
* **Polygons:** Inserting any internal line between two non-adjacent vertices on a polygon result on two smaller polygons
* **Strings:** Delete the first character of a string and end up with a smaller string

## Proof by Contradiction
The basic schema of a contradiction is:
* Assume that the hypothesis *is false*
* Develop some logical consequences of this assumption
* Show that one consequence is demonstrably false, thereby showing that the assumption is false

E.g.: Euclid's proof of infinite prime numbers. If we assume that there is a finite number of prime numbers as *p1, ..., pm* and start working with that.
We construct a number that is the product of all the prime numbers:
```rust
    let n = [p1,...,pn];
    let m = n.len();
    let mut prod = 1;

    for i in 0..m {
        prod *= n[i];
    }
```

This `prod` should be divisible by all prime numbers because the way it is contructed. But if we consider `prod + 1` it can't be divisible by 2 (a prime number) because `prod` already is. Same if try division by 3 and every other listed prime number. Since `prod + 1` can not be divible by all listed number on `n`, `prod + 1` must be a prime number, disproving that only exist a `m` amount of prime numbers.
