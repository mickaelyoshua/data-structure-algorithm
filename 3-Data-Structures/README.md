# Data Structures
## Contiguous vs. Linked Data Structures
**Contiguous Data Structures** store elements in a single, continuous block of memory.
- **Examples:** Arrays, heaps, and hash tables.
- **Pros:** Excellent for random access (O(1)) because element addresses can be calculated directly from their index.
- **Cons:** Fixed size. Resizing is an expensive operation, and insertions/deletions in the middle of the structure require shifting elements, which is inefficient (O(n)).

**Linked Data Structures** store elements in separate chunks of memory (nodes) that are connected through pointers or references.
- **Examples:** Linked lists, trees, and graphs.
- **Pros:** Dynamic size, allowing for efficient insertions and deletions (O(1)) without reallocating the entire structure.
- **Cons:** Accessing an element requires traversing the structure from the beginning (O(n)), so random access is not possible. They also require extra memory to store the pointers.

### Key Differences

| Feature              | Contiguous Structures (e.g., Array) | Linked Structures (e.g., Linked List) |
| -------------------- | ----------------------------------- | ------------------------------------- |
| **Memory Allocation**| Single continuous block             | Individual, scattered blocks (nodes)  |
| **Random Access**    | O(1)                                | O(n)                                  |
| **Insertion/Deletion**| O(n) - requires shifting elements   | O(1) - requires updating pointers     |
| **Size**             | Static (fixed at creation)          | Dynamic (can grow or shrink)          |
| **Memory Overhead**  | Minimal                             | Higher (stores data + pointers)       |

Advantages of linked list structures over static arrays include:
* Overflow on linked structures never occurs unless the memory is actually full.
* Insertion and deletion are simples then for static arrays.
* With large records, moving points is easier and faster then moving the items themselves.

Conversely, the relative advantages of arrays include:
* Space efficiency: linked structures require extra memory for storing pointer field.
* Efficient random access to items in arrays.
* Better memory locality and cache performance then random pointer jumping.

Dynamic memory allocation provides flexibility on how it is used the limited storage resources.

Since lists and arrays can be thought as recursive data structures (cut a string, list or array and have a two smaller pieces) it leads to efficient dived-and-conquer algorithms such as quicksort and binary search.

## Priority Queue
Is an abstract data type that provides more flexibility then simple sorting, because allows element to be inserted at arbitrary intervals. It can be more cost-effective to insert a new item into a priority queue than to resort the entire set.

The basic priority queue supports 3 basic operations:
* **Insert**: given *x* insert into the priority queue.
* **Find-Minimum and Find-Maximum**: return a pointer to the less priority and to the more priority item.
* **Delete-Minimum and Delete-Maximum**: remove the less priority item or the more priority.

## Hashing
A hash function is a mathematical function that maps keys to integers. They exploit the fact that looking an item up in an array takes constant time if the index is known.

It works in two steps:

1. **Convert to a Giant Number:** The function first treats the string as a number in a different base system. Instead of our usual base-10, it uses a base equal to the size of the alphabet (e.g., base-26). Each character acts as a "digit," creating a unique but enormous number that represents the string.

2. **Shrink to a Usable Index:** Since this giant number is too big to be an array index, the modulo operator (`mod m`) is used. This is like finding the remainder of a division. It "wraps" the giant number around the size of the array (`m`), guaranteeing the result is always a valid index between `0` and `m-1`, much like how a roulette ball always lands in one of the fixed pockets regardless of how many times it spins.

### Collision Resolution
There are two different approaches for maintaining a hash table:
* **Chaining**: represents a hash table as an array of `m` linked lists ("buckets"). The *i*th list will contain all the items that hash to the value *i*. Search, insertion and deletion will the correspond to the linked list. It is very natural but demands a considerable amount of memory for pointers.

* **Open addressing**: maintain the hash table as a simple array of elements (not buckets). Each cell is initialized with null. On each insertion is verifyed if the cell is empty, if it is the value is inserted, if it is not will be search another place to insert the value. The simples possibility (called *sequential probing*) is to insert on the next empty cell on the hash table.

Hash functions provide useful tools beyond powering hash tables by using many-to-one mappings where it's unlikely for the "many" to become "too many." Two such methods are **canonicalization** and **compaction**.

### Other Hasing Tricks
#### Canonicalization
Canonicalization is the process of converting complex objects into a standard, or "canonical," form. The goal is to make multiple, varied strings collide on the same hash code, which is useful for pattern matching problems.

#### Compaction
Compaction, also known as fingerprinting, involves representing large objects with small hash codes. It's easier to work with smaller objects than larger ones, and the hash code typically preserves the identity of the original item.
