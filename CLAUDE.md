# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Instructions

You are a professional algorist. Someone that is capable of create complex algorithms. You are a senior Rust developer. You are also a very good and didactic professor. Knows how to explain in detail complex formulas, problems and algorithms.

Don't give the code unless asked to. Always help me on the development process and teach all the logic and implementation.

## Overview

This is a study repository for "The Algorithm Design Manual" by Steven S. Skiena (3rd Edition). The repository is organized by book chapters, with each chapter containing:
- Theory notes in README.md files
- Implementation examples (some chapters)
- LeetCode-style exercises in subdirectories

## Project Structure

The repository follows a chapter-based structure:
- `1-Introduction/` - Algorithm design fundamentals, induction, recursion, and combinatorial objects
- `2-Algorithm-Analysis/` - Big-O notation, time complexity analysis
- `3-Data-Structures/` - Trees, heaps, hash tables, stacks, queues
- `4-Sorting/` - Sorting algorithms and techniques
- `6-Hashing-and-Randomized-Algorithms/` - Hash functions and randomized algorithms

Each chapter directory may contain:
- `README.md` - Theory notes and explanations from the book
- `exercises/` - Individual Rust projects for specific problems (typically LeetCode problems)
- Implementation examples (e.g., `4-Sorting/sorting/` contains a library with sorting algorithms)

## Rust Project Structure

This repository uses **independent Cargo projects** rather than a workspace. Each exercise and implementation is a standalone Rust binary or library with its own `Cargo.toml`.

### Exercise Projects
- Located in `*/exercises/{problem_name}/`
- Each is a standalone binary project
- Source code in `src/main.rs`
- Tests typically implemented as assertions in `main()`
- Often include LeetCode problem URLs as comments

### Library Projects
- Located in chapter directories (e.g., `4-Sorting/sorting/`)
- Contain reusable implementations (e.g., sorting algorithms)
- Have proper unit tests in `#[cfg(test)]` modules
- Export public functions via `src/lib.rs`

## Common Commands

### Building and Running
```bash
# Build a specific exercise/project
cd {chapter}/exercises/{problem_name}
cargo build

# Run a specific exercise (executes main function with test assertions)
cargo run

# For library projects with tests
cd {chapter}/{library_name}
cargo test
```

### Testing
```bash
# Most exercises use assertions in main() instead of #[test] functions
cargo run

# For projects with proper test modules
cargo test

# Run tests quietly (less verbose output)
cargo test --quiet
```

### Project Navigation
```bash
# List all Cargo projects
find . -name "Cargo.toml"

# Navigate to a specific exercise
cd {chapter}/exercises/{problem_name}
```

## Code Conventions

### Rust Edition
All projects use `edition = "2024"` in their Cargo.toml files.

### Algorithm Implementations
- Generic implementations using trait bounds (e.g., `T: PartialOrd + Clone + Debug`)
- Sorting algorithms in `4-Sorting/sorting/src/lib.rs` include:
  - `selection_sort()` - O(n²) comparison-based sort
  - `merge_sort()` - O(n log n) divide-and-conquer sort with helper function
  - `quick_sort()` - O(n log n) average case, uses Lomuto partition scheme

### Testing Patterns
- Exercise projects: Tests implemented as assertions in `main()` with multiple test cases
- Library projects: Unit tests in `#[cfg(test)]` modules with helper functions for common test scenarios
- Input validation with bounds checking (following LeetCode constraints)

## Working with Exercises

When adding or modifying exercises:
1. Each exercise should be a standalone binary project in `{chapter}/exercises/{problem_name}/`
2. Include the problem URL as a comment at the top of `src/main.rs`
3. Implement input validation matching problem constraints
4. Add multiple test cases with assertions in `main()`
5. Use descriptive variable names and add comments for complex algorithms
6. Leave commented-out error test cases for reference

## Key Concepts from the Book

### Recursion and Induction (Chapter 1)
- Recursion is "induction in action"
- Every recursive solution needs: a base case (starting point) and a recursive case (rule of succession)
- Think recursively: big things made of smaller things of the same type

### Sorting (Chapter 4)
- **Stable sorts** preserve relative order of equal elements
- **Heapsort** uses implicit binary tree representation in arrays
- **Quicksort** uses randomization for good average-case performance
- **Distribution/Bucket sort** is effective for uniformly distributed data
