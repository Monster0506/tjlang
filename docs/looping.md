# Looping in TJLang

TJLang provides three types of loop constructs to handle iteration: **for loops** (both iterator-style and C-style), **while loops**, and **do-while loops**. This guide covers all three in detail.

---

## Table of Contents
- [For Loops](#for-loops)
  - [Iterator-Style For Loops](#iterator-style-for-loops)
  - [C-Style For Loops](#c-style-for-loops)
  - [Range Expressions](#range-expressions)
- [While Loops](#while-loops)
- [Do-While Loops](#do-while-loops)
- [Nested Loops](#nested-loops)

---

## For Loops

TJLang supports **two types of for loops**:
1. **Iterator-style** (`for (variable: type; iterable)`) - iterates over collections or ranges
2. **C-style** (`for (init; condition; increment)`) - traditional counter-based loops

### Iterator-Style For Loops

Iterator-style for loops are the most common way to iterate over ranges and collections in TJLang.

#### Basic Syntax
```tjlang
for (variable: type; iterable) {
    # loop body
}
```

#### Iterating Over Ranges

##### Exclusive Range (`$`)
The `$` operator creates a range that **excludes** the end value:
```tjlang
for (i: int; 0 $ 5) {
    IO.println("i = " + i.to_string())
}
# Prints: 0, 1, 2, 3, 4
```

##### Inclusive Range (`$=`)
The `$=` operator creates a range that **includes** the end value:
```tjlang
for (i: int; 0 $= 5) {
    IO.println("i = " + i.to_string())
}
# Prints: 0, 1, 2, 3, 4, 5
```

#### Iterating Over Arrays

##### By Item
Loop directly over array elements:
```tjlang
arr: [int] = [10, 20, 30, 40, 50]
for (item: int; arr) {
    IO.println("item = " + item.to_string())
}
# Prints: item = 10, item = 20, item = 30, item = 40, item = 50
```

##### By Index
Use a range with the array's length to access elements by index:
```tjlang
arr: [int] = [10, 20, 30, 40, 50]
for (i: int; 0 $ arr.length()) {
    IO.println("arr[" + i.to_string() + "] = " + arr.get(i).to_string())
}
```

#### Using Variables in Ranges
You can use variables for range bounds:
```tjlang
start: int = 0
end: int = 3
for (i: int; start $= end) {
    IO.println("i = " + i.to_string())
}
```

You can even use function calls:
```tjlang
def get_max() -> int {
    return 5
}

for (i: int; 0 $ get_max()) {
    IO.println("i = " + i.to_string())
}
```

#### Ignoring Loop Variables
Use `_` when you don't need the loop variable:
```tjlang
# Print a newline 3 times
for (_: int; 0 $ 3) {
    IO.println()
}
```

### C-Style For Loops

C-style for loops provide explicit control over initialization, condition, and increment.

#### Basic Syntax
```tjlang
for (initializer; condition; increment) {
    # loop body
}
```

#### Examples

##### Counting Up
```tjlang
for (i: int = 0; i < 10; i = i + 1) {
    IO.println("i = " + i.to_string())
}
# Prints: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9
```

##### Counting Down
```tjlang
for (i: int = 5; i >= 0; i = i - 1) {
    IO.println("i = " + i.to_string())
}
# Prints: 5, 4, 3, 2, 1, 0
```

##### Using Complex Conditions
```tjlang
arr: [int] = [10, 20, 30]
for (i: int = 0; i < arr.length(); i = i + 1) {
    IO.println("arr[" + i.to_string() + "] = " + arr.get(i).to_string())
}
```

##### Using Function Calls in Conditions
```tjlang
def return_2() -> int {
    return 2
}

for (i: int = 0; i <= return_2(); i = i + 1) {
    IO.println("i = " + i.to_string())
}
# Prints: 0, 1, 2
```

### Range Expressions

Range expressions in TJLang are technically **immutable arrays of integers** generated on-the-fly.

#### Operators
- **`$`** - Exclusive range: `start $ end` creates `[start, start+1, ..., end-1]`
- **`$=`** - Inclusive range: `start $= end` creates `[start, start+1, ..., end]`

#### Examples
```tjlang
# Exclusive: 0 $ 5 produces [0, 1, 2, 3, 4]
for (i: int; 0 $ 5) {
    IO.println(i.to_string())
}

# Inclusive: 0 $= 5 produces [0, 1, 2, 3, 4, 5]
for (i: int; 0 $= 5) {
    IO.println(i.to_string())
}
```

---

## While Loops

While loops execute their body repeatedly as long as a condition remains true. The condition is checked **before** each iteration.

### Syntax
```tjlang
while condition {
    # loop body
}
```

### Basic Example
```tjlang
i: int = 0
while i < 3 {
    IO.println("i = " + i.to_string())
    i = i + 1
}
# Output:
# i = 0
# i = 1
# i = 2
```

### While Loop with False Condition
If the condition is false initially, the loop body never executes:
```tjlang
j: int = 10
while j < 5 {
    IO.println("This will never print")
}
IO.println("j is still " + j.to_string())
# Output: j is still 10
```

### Complex Conditions
While loops support complex boolean expressions:
```tjlang
a: int = 0
b: int = 10
while a < 5 and b > 5 {
    IO.println("a = " + a.to_string() + ", b = " + b.to_string())
    a = a + 1
    b = b - 1
}
# Output:
# a = 0, b = 10
# a = 1, b = 9
# a = 2, b = 8
# a = 3, b = 7
# a = 4, b = 6
```

### When to Use While Loops
- When the number of iterations is **unknown** beforehand
- When you need to loop until a certain **state** is reached
- When the loop condition depends on **runtime data** or **external factors**

---

## Do-While Loops

Do-while loops are similar to while loops, but they check the condition **after** each iteration. This means the loop body **always executes at least once**.

### Syntax
```tjlang
do {
    # loop body
} while condition
```

### Basic Example
```tjlang
k: int = 0
do {
    IO.println("k = " + k.to_string())
    k = k + 1
} while k < 3
# Output:
# k = 0
# k = 1
# k = 2
```

### Do-While with False Condition
Even if the condition is false, the body executes once:
```tjlang
m: int = 10
do {
    IO.println("m = " + m.to_string())
    m = m + 1
} while m < 5
IO.println("Final m = " + m.to_string())
# Output:
# m = 10
# Final m = 11
```

### Practical Use Case: Array Accumulation
```tjlang
arr: [int] = []
n: int = 0
do {
    arr.push(n)
    n = n + 1
} while n < 5
IO.println("Array: " + arr.to_string())
# Output: Array: [0, 1, 2, 3, 4]
```

### When to Use Do-While Loops
- When you need the loop body to execute **at least once**
- For **menu systems** or **input validation** where you always need to prompt the user first
- When initializing or accumulating values before checking a condition

---

## Nested Loops

All loop types can be nested inside each other.

### Nested For Loops
```tjlang
for (i: int; 0 $ 2) {
    IO.println("i = " + i.to_string())
    for (j: int; 0 $ 2) {
        IO.println("  j = " + j.to_string())
    }
}
# Output:
# i = 0
#   j = 0
#   j = 1
# i = 1
#   j = 0
#   j = 1
```

### Nested While Loops
```tjlang
x: int = 0
while x < 2 {
    IO.println("outer x = " + x.to_string())
    y: int = 0
    while y < 2 {
        IO.println("  inner y = " + y.to_string())
        y = y + 1
    }
    x = x + 1
}
# Output:
# outer x = 0
#   inner y = 0
#   inner y = 1
# outer x = 1
#   inner y = 0
#   inner y = 1
```

### Mixing Loop Types
You can nest different loop types:
```tjlang
arr: [int] = [1, 2, 3]
for (item: int; arr) {
    i: int = 0
    while i < item {
        IO.println("  " + i.to_string())
        i = i + 1
    }
}
```
