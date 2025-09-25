# Sorting
## Pragmatics of Sorting
When having the repeated elements on a set, can be used a secound criteria to sort them. Sometimes is required to leave items in the same relative order as in the original permutation. Sorting algorithms that force this behaviour are called *stable*. Stability can be achieved for any sorting algorithm bya adding the initial position as a secondary key.

## Heapsort: Fast Sorting via Data Structures
The *heap* is a slick data structure that enables to represent a binary tree without the use of pointers. This is a good approach because on a regular binary search tree the memory to allocate all the pointers can be big. This is possible by storing data as an array of keys and use this keys to *implicitly* play the rool of pointers (similar to a Fenwick Tree).
