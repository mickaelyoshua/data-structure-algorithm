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

## Containers: Stacks and Queues

