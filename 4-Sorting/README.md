# Sorting
## Pragmatics of Sorting
When having the repeated elements on a set, can be used a secound criteria to sort them. Sometimes is required to leave items in the same relative order as in the original permutation. Sorting algorithms that force this behaviour are called *stable*. Stability can be achieved for any sorting algorithm bya adding the initial position as a secondary key.

## Heapsort: Fast Sorting via Data Structures
The *heap* is a slick data structure that enables to represent a binary tree without the use of pointers. This is a good approach because on a regular binary search tree the memory to allocate all the pointers can be big. This is possible by storing data as an array of keys and use this keys to *implicitly* play the rool of pointers (similar to a Fenwick Tree).

## Quicksort
### Randomized algorithms
Some of the basic approaches to design efficient randomized algorithms are:
* **Ramdom sampling**: Gives a good average-case scenario for some algorithms (like quicksort);
* **Randomized hashing**: For any hash function there is a given worst-case set of keys that will all get hashed to the same bucket. By randomly selecting the hash function from a large family of good ones as the first step we get the same average-case scenario in most cases.
* **Randomized search**: Randomization can also be used to drive search techniques such as simulated annealing.

## Distribution Sort / Bucket Sort
Bucketing is a vary effective idea whenever we are confident that the distribution of data will be roughly uniform. It is the idea that underlies hash tables, kd-trees, and a variety of other practical data structures.
