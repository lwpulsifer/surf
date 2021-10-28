# Surf programming language

## Introduction

The Surf programming language is, or plans to be, a general-purpose scripting language with the following major features:

- Static typing
- Type inference with optional type annotations
  - And the ability to let the compiler add type annotations for you when you've gotten your code working
- Parametric polymorphism/Generics
- First-class functions
- Simple, minimal syntax focused on legibility for programmers coming from most modern programming languages

## Motivating examples (beta)

A simple example program in Surf might look like

```
fn add = (a: int, b: int) -> a + b;

println(add(5, 10))
```

A slightly more complicated program might read

```
---
This is a block comment.

The fibonacci function takes in an integer n and 
generates the n-th term of the Fibonacci sequence.
---
fn fibonacci = (n) => {
  if (n <= 1) {
    return 0;
  }
  let lastTerm = 0;
  let currentTerm = 1;
  for _ in range(2, n) {
    lastTerm, currentTerm = currentTerm, lastTerm + currentTerm; 
  }
  return currentTerm;
}
```

When the program above is compiled with the `--add-type-annotations` flag, it will read:

```
---
This is a block comment.

The fibonacci function takes in an integer n and 
generates the n-th term of the Fibonacci sequence.
---
fn fibonacci = (n: int) => {
  if (n <= 1) {
    return 0;
  }
  let lastTerm: int = 0;
  let currentTerm: int = 1;
  for _: int in range(2, n) {
    lastTerm, currentTerm = currentTerm, lastTerm + currentTerm; 
  }
  return currentTerm;
}
```

## Typing

TBD

I'm thinking enums, type definitions, datatypes, etc. 
Functional stuff.

Nominal typing for interface implementation.

```
interface Equatable[T] {
  equals: (other: T) -> boolean,
}

enum Ord {
  LT,
  EQ,
  GT,
}

interface Comparable: T {
  compareTo: (other: T) -> Ord,
}

interface Set[T | Hashes>] implements Equatable[T] {
  add: (item: T) -> int,

  remove: (item: T) -> int,

  size: () -> int,
}

interface DumbSet[T] implements Set[T] {
  elements: []: List[T],
  add: (item: T) -> {
    elements += item;
    return elements.size();
  },
  remove: (item: T) -> {
    elements.remove(item);
    return elements.size();
  },
  size:
}
```

## Implementation
I'm planning to implement Surf in Rust. It's completely
up in the air at the moment whether Surf will be interpreted, compiled, or run using some combination
of the two and a VM. Whatever it is, it certainly won't
be fancy or heavily optimized. 

