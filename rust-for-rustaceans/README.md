# Rust for Rustaceans: Idiomatic Programming for Experienced Developers

## [Chapter 1: Foundations](./chapter1.md)

### Talking About Memory

- Memory Terminology
- Variables in Depth
- Memory Regions

### Ownership

### Borrowing and Lifetimes

- Shared References
- Mutable References
- Interior Mutability
- Lifetimes

### Summary

## [Chapter 2: Types](./chapter2.md)

### Types in Memory

- Alignment
- Layout
- Complex Types
- Dynamically Sized Types and wide Pointers

### Traits and Trait Bounds

- Compilation and Dispatch
- Generic Traits
- Coherence and the Orphan Rule
- Trait Bounds
- Maker Traits

### Existential Types

### Summary

## [Chapter 3: Designing Interfaces](./chapter3.md)

### Unsurprsing

- Naming Practices
- Common Traits for Types
- Ergonomic Trait Implementations
- Wrapper Types

### Flexible

- Generic Arguments
- Object Safety
- Borrowed vs. Owned
- Fallible and Blocking Destructors

### Obvious

- Documentation
- Type System Guidance

### Constrained

- Type Modifications
- Trait Implementations
- Hidden Contracts

### Summary

## [Chapter 4: Error Handling](./chapter4.md)

### Representing Errors

- Enumeration
- Opaque Errors
- Special Error Cases

### Propagating Errors

### Summary

## [Chapter 5: Project Structure](./chapter5.md)

### Features

- Defining and Including Features
- Using Features in Your Crate

### Workspaces

### Project Configuration

- Crate Metadata
- Build Configuration

### Conditional Compilation

### Versioning

- Minimum Supported Rust Version
- Minimal Dependency Versions
- Changlelogs
- Unreleased Versions

### Summary

## [Chapter 6: Testing](./chapter6.md)

### Rust Testing Mechanisms

- The Test Harness
- #[cfg(test)]
- Doctests

### Additional Testing Tools 

- Linting
- Test Generation
- Test Augmentation
- Performance Testing

### Summary

## [Chapter 7: Macros](./chapter7.md)

### Declarative Macros

- When to Use Them
- How They Work
- How to Write Declarative Macros

### Procedural Macros

- Types of Procedural Macros
- The Cost of Procedural Macros
- So You Think You Want a Macro
- How Do They Work?

### Summary

## [Chapter 8: Asymchronous Programming](./chapter8.md)

### What's the Deal with Asynchrony?

- Synchronous Interfaces
- Multithreading
- Asynchronous Interfaces
- Standardized Polling

### Ergonomic Futures

- Async/Await
- Pin and Unpin

### Going to Sleep

- Waking Up
- Fulfilling the Poll Contract
- Waking is a Misnomer
- Tasks and Subexecutors

### Tying It All Together with spawn

### Summary

## [Chapter 9: Unsafe code](./chapter9.md)

### The unsafe keyword

### Great Power

- Juggling Raw Pointers
- Calling Unsafe Functions
- Implementing Unsafe Traits

### Great Responsibility

- What Can go Wrong?
- Validity
- Panics
- Casting
- The Drop Check

### Coping with Fear

- Manage unsafe Boundaries
- Read and write Documentation
- Check Your Work

### Summary

## [Chapter 10: Concurrency(And Parallelism)](./chapter10.md)

### The trouble with Concurrency

- Correctness
- Performance

### Concurrency Models

- Shared Memory
- Worker Pools
- Actors

### Asynchrony And Parallelism

### Lower-Level Concurrency

- Memory Operations
- Atomic Types
- Memory Ordering
- Compare and Exchange
- The Fetch Methods

### Sane Concurrency

- Start Simple
- Write StressTest
- Use Concurrency Testing Tools

### Summary

## [Chapter 11: Foreign Function Interfaces](./chapter11.md)

### Crossing Boundaries with extern

- Symbols
- Calling Conventions

### Types Across Language Boundaries

- Type Matching
- Allocations
- Callbacks
- Safety

### bindgen and Build Scripts

### Summary

## [Chapter 12: Rust without the Standard Library](./chapter12.md)

### Opting Out of the Standard Library

### Dynamic Memory Allocation

### The Rust Runtime

- The Panic Handler
- ProgramInitialization
- The Out-of-Memory Handler

### Low-Level Memory Accesses

### Misuse-Resistant Hardware Abstraction

### Cross-Compilation

### Summary

## [Chapter 13: The Rust Ecosystem](./chapter13.md)

### What's Out There?

- Tools
- Libraries
- Rust Tooling
- The Standard Library

### Patterns in the Wild

- Index Pointers
- Drop Guards
- Extension Traits
- Crate Preludes

### Staying Up to Date

### What Next?

- Learn by Watching
- Learn by Doing
- Learn by Reading
- Learn By Teaching

### Summary

