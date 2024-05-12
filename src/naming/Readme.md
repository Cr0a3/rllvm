# The RLLVM naming convention
This is the official RLLVM naming convention document.

## Example
An example for a encoded function:
```
_R4testZ0Z3addZ3u323u32Z3u32
``` 
Which would be following function:
```rust
test::add(u32, u32) -> u32
```

## Documentation
Naming convented names allways start with **`_R`**.

The seperator betwenn different areas is a **Z**.

Values are written like following: [len of value][value]
which results in (for e.g. the function `add`) `3add` because the length of the string `add` is `3`. Empty values are just a `0`

This is the fixed seqence of values:

- namespaces
- seperator
- class
- seperator
- function name
- seperator
- arguments
- seperator
- return type

Which results in the example given above:

<pre>
_R4testZ0Z3addZ3u323u32Z3u32
|   |  |    | |  |  |  |  |
|   |  |    | |  |  |  |  |-> `u32` The return type
|   |  |    | |  |  |  |
|   |  |    | |  |  |  |---> Seperator
|   |  |    | |  |  |
|   |  |    | |  |  |------> `u32` The type of the 2. arg
|   |  |    | |  |---------> `u32` The type of the 1. arg
|   |  |    | |  |
|   |  |    | |------------> Seperator
|   |  |    | |
|   |  |    |--------------> `add` Name of the function 
|   |  |
|   |  |-------------------> Seperator 
|   |
|   |----------------------> `test` The namespace name 
|
|--------------------------> The RLLVM Naming Convention header
</pre>