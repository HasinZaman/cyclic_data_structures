# Cyclic Data Types

Cyclic data types are series of structs, enums, types and functions to create a series of fast data types.

## Implementation
This achieved by using generic arrays and start & end points. As a result, insertions, deletions at the beginning and end of any datatype occurs in O(1). Reading anywhere in the list is O(1). However, this is at the cost of limiting the size of the each datatype at compile time.

## Types
### List

The list module contains a series of structs to create cyclic lists and their utility functionalities.

Cyclic List Example
```text
 1 ↔ 2 ↔ ... ↔ n
 ↑             ↑
 └─────────────┘
```
Note: Even though the diagram uses arrow to denote the relationship between nodes/elements; implying a linked node structure - however, the implementation in this crate uses arrays.

### Stack

The stack module contains a series of structs to create stacks and their utility functionalities using cyclic lists.

It is recommended to use [`Vec`] over [`Stack`] for most applications. As [`Vec`] has better - if not similar performance to the [`Stack`]. It is therefore, [`Stack`] should only be used when the stack should strictly be limited to a given size and or convince of life features provided by the [`Stack`].

### Queue

The queue module contains a series of structs to create queues and their utility functionalities using cyclic lists.

As a result, the queue inherits the O(1) insertion and deletion for enqueuing & dequeuing.