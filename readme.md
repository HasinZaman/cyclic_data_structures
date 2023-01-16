# Cyclic Data Types

Cyclic data types are series of structs, enums, types and functions to create a series of fast data types.

## Implementation
This achieved by using generic arrays and start & end points. As a result, insertions, deletions at the beginning and end of any datatype occurs in O(1). However, this is at the cost of limiting the size of the each datatype at compile time.

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

The stack module contains a series of structs to create stacks and their utility functionalities.