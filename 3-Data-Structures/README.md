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


