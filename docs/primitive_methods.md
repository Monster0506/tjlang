# TJLang Primitive Methods Documentation

This document describes all the methods available on primitive types in TJLang. These methods work on the basic data types: `int`, `float`, `bool`, `str`, and `None`.

## Table of Contents

- [Universal Methods](#universal-methods) - Methods available on all primitive types
- [Integer Methods](#integer-methods) - Methods specific to `int` type
- [Float Methods](#float-methods) - Methods specific to `float` type
- [Boolean Methods](#boolean-methods) - Methods specific to `bool` type
- [String Methods](#string-methods) - Methods specific to `str` type
- [None Methods](#none-methods) - Methods specific to `None` type
- [Type Conversion](#type-conversion) - Methods for converting between types
- [Examples](#examples) - Usage examples

## Universal Methods

These methods are available on all primitive types.

### Core Methods

#### `to_string() -> str`
Returns a string representation of the value.

```tjlang
x: int = 42
s: str = x.to_string()  // "42"

b: bool = true
s2: str = b.to_string() // "true"
```

#### `clone() -> T`
Creates a copy of the value.

```tjlang
x: int = 42
y: int = x.clone()  // y is a copy of x
```

#### `equals(other: T) -> bool`
Compares the value with another value for equality.

```tjlang
x: int = 42
y: int = 42
equal: bool = x.equals(y)  // true
```

#### `type_name() -> str`
Returns the type name of the value.

```tjlang
x: int = 42
type_name: str = x.type_name()  // "int"

s: str = "hello"
type_name2: str = s.type_name() // "str"
```

#### `is_null() -> bool`
Checks if the value is `None`.

```tjlang
x: int = 42
is_none: bool = x.is_null()  // false

n: Option<int> = None
is_none2: bool = n.is_null() // true
```

#### `is_not_null() -> bool`
Checks if the value is not `None`.

```tjlang
x: int = 42
not_none: bool = x.is_not_null()  // true

n: Option<int> = None
not_none2: bool = n.is_not_null() // false
```

#### `hash() -> int`
Returns a hash code for the value.

```tjlang
x: int = 42
hash_code: int = x.hash()  // 42

s: str = "hello"
hash_code2: int = s.hash() // 5 (length of string)
```

### Type Checking Methods

#### `is_int() -> bool`
Checks if the value is an integer.

```tjlang
x: int = 42
is_integer: bool = x.is_int()  // true

f: float = 3.14
is_integer2: bool = f.is_int() // false
```

#### `is_float() -> bool`
Checks if the value is a float.

```tjlang
f: float = 3.14
is_float: bool = f.is_float()  // true

x: int = 42
is_float2: bool = x.is_float() // false
```

#### `is_bool() -> bool`
Checks if the value is a boolean.

```tjlang
b: bool = true
is_boolean: bool = b.is_bool()  // true

x: int = 42
is_boolean2: bool = x.is_bool() // false
```

#### `is_str() -> bool`
Checks if the value is a string.

```tjlang
s: str = "hello"
is_string: bool = s.is_str()  // true

x: int = 42
is_string2: bool = x.is_str() // false
```

#### `is_none() -> bool`
Checks if the value is `None`.

```tjlang
n: Option<int> = None
is_none: bool = n.is_none()  // true

x: int = 42
is_none2: bool = x.is_none() // false
```

### Utility Methods

#### `debug_string() -> str`
Returns a debug representation of the value.

```tjlang
x: int = 42
debug: str = x.debug_string()  // "Int(42)"

s: str = "hello"
debug2: str = s.debug_string() // "String(\"hello\")"
```

#### `pretty_string() -> str`
Returns a pretty-printed string representation of the value.

```tjlang
x: int = 42
pretty: str = x.pretty_string()  // "42"

f: float = 3.0
pretty2: str = f.pretty_string() // "3" (no decimal for whole numbers)

s: str = "hello"
pretty3: str = s.pretty_string() // "\"hello\""
```

## Integer Methods

These methods are only available on `int` values.

#### `abs() -> int`
Returns the absolute value.

```tjlang
x: int = -42
abs_x: int = x.abs()  // 42

y: int = 42
abs_y: int = y.abs()  // 42
```

#### `neg() -> int`
Returns the negated value.

```tjlang
x: int = 42
neg_x: int = x.neg()  // -42

y: int = -42
neg_y: int = y.neg()  // 42
```

#### `inc() -> int`
Returns the value incremented by 1.

```tjlang
x: int = 42
inc_x: int = x.inc()  // 43
```

#### `dec() -> int`
Returns the value decremented by 1.

```tjlang
x: int = 42
dec_x: int = x.dec()  // 41
```

#### `is_even() -> bool`
Checks if the value is even.

```tjlang
x: int = 42
even: bool = x.is_even()  // true

y: int = 43
even2: bool = y.is_even() // false
```

#### `is_odd() -> bool`
Checks if the value is odd.

```tjlang
x: int = 43
odd: bool = x.is_odd()  // true

y: int = 42
odd2: bool = y.is_odd() // false
```

#### `is_positive() -> bool`
Checks if the value is positive.

```tjlang
x: int = 42
positive: bool = x.is_positive()  // true

y: int = -42
positive2: bool = y.is_positive() // false
```

#### `is_negative() -> bool`
Checks if the value is negative.

```tjlang
x: int = -42
negative: bool = x.is_negative()  // true

y: int = 42
negative2: bool = y.is_negative() // false
```

#### `is_zero() -> bool`
Checks if the value is zero.

```tjlang
x: int = 0
zero: bool = x.is_zero()  // true

y: int = 42
zero2: bool = y.is_zero() // false
```

## Float Methods

These methods are only available on `float` values.

#### `abs() -> float`
Returns the absolute value.

```tjlang
f: float = -3.14
abs_f: float = f.abs()  // 3.14
```

#### `neg() -> float`
Returns the negated value.

```tjlang
f: float = 3.14
neg_f: float = f.neg()  // -3.14
```

#### `ceil() -> float`
Returns the ceiling of the value.

```tjlang
f: float = 3.14
ceil_f: float = f.ceil()  // 4.0

g: float = -3.14
ceil_g: float = g.ceil()  // -3.0
```

#### `floor() -> float`
Returns the floor of the value.

```tjlang
f: float = 3.14
floor_f: float = f.floor()  // 3.0

g: float = -3.14
floor_g: float = g.floor()  // -4.0
```

#### `round() -> float`
Returns the rounded value.

```tjlang
f: float = 3.14
round_f: float = f.round()  // 3.0

g: float = 3.6
round_g: float = g.round()  // 4.0
```

#### `trunc() -> float`
Returns the truncated value (removes decimal part).

```tjlang
f: float = 3.14
trunc_f: float = f.trunc()  // 3.0

g: float = -3.14
trunc_g: float = g.trunc()  // -3.0
```

#### `is_finite() -> bool`
Checks if the value is finite.

```tjlang
f: float = 3.14
finite: bool = f.is_finite()  // true

inf: float = 1.0 / 0.0  // infinity
finite2: bool = inf.is_finite() // false
```

#### `is_infinite() -> bool`
Checks if the value is infinite.

```tjlang
f: float = 3.14
infinite: bool = f.is_infinite()  // false

inf: float = 1.0 / 0.0  // infinity
infinite2: bool = inf.is_infinite() // true
```

#### `is_nan() -> bool`
Checks if the value is NaN (Not a Number).

```tjlang
f: float = 3.14
nan: bool = f.is_nan()  // false

n: float = 0.0 / 0.0  // NaN
nan2: bool = n.is_nan() // true
```

#### `is_positive() -> bool`
Checks if the value is positive.

```tjlang
f: float = 3.14
positive: bool = f.is_positive()  // true

g: float = -3.14
positive2: bool = g.is_positive() // false
```

#### `is_negative() -> bool`
Checks if the value is negative.

```tjlang
f: float = -3.14
negative: bool = f.is_negative()  // true

g: float = 3.14
negative2: bool = g.is_negative() // false
```

#### `is_zero() -> bool`
Checks if the value is zero.

```tjlang
f: float = 0.0
zero: bool = f.is_zero()  // true

g: float = 3.14
zero2: bool = g.is_zero() // false
```

## Boolean Methods

These methods are only available on `bool` values.

#### `not() -> bool`
Returns the logical NOT of the value.

```tjlang
b: bool = true
not_b: bool = b.not()  // false

c: bool = false
not_c: bool = c.not()  // true
```

## String Methods

These methods are only available on `str` values.

#### `length() -> int`
Returns the length of the string.

```tjlang
s: str = "hello"
len: int = s.length()  // 5
```

#### `is_empty() -> bool`
Checks if the string is empty.

```tjlang
s: str = "hello"
empty: bool = s.is_empty()  // false

s2: str = ""
empty2: bool = s2.is_empty() // true
```

#### `is_not_empty() -> bool`
Checks if the string is not empty.

```tjlang
s: str = "hello"
not_empty: bool = s.is_not_empty()  // true

s2: str = ""
not_empty2: bool = s2.is_not_empty() // false
```

#### `trim() -> str`
Returns the string with leading and trailing whitespace removed.

```tjlang
s: str = "  hello  "
trimmed: str = s.trim()  // "hello"
```

#### `upper() -> str`
Returns the string in uppercase.

```tjlang
s: str = "hello"
upper: str = s.upper()  // "HELLO"
```

#### `lower() -> str`
Returns the string in lowercase.

```tjlang
s: str = "HELLO"
lower: str = s.lower()  // "hello"
```

#### `capitalize() -> str`
Returns the string with the first character capitalized and the rest lowercase.

```tjlang
s: str = "hello"
cap: str = s.capitalize()  // "Hello"

s2: str = "HELLO"
cap2: str = s2.capitalize() // "Hello"
```

#### `reverse() -> str`
Returns the string with characters in reverse order.

```tjlang
s: str = "hello"
rev: str = s.reverse()  // "olleh"
```

## None Methods

These methods are only available on `None` values.

#### `is_none() -> bool`
Always returns `true` for `None` values.

```tjlang
n: Option<int> = None
is_none: bool = n.is_none()  // true
```

#### `is_not_none() -> bool`
Always returns `false` for `None` values.

```tjlang
n: Option<int> = None
is_not_none: bool = n.is_not_none()  // false
```

## Type Conversion

These methods allow converting between different primitive types.

### `to_int() -> int`
Converts the value to an integer.

```tjlang
f: float = 3.14
i: int = f.to_int()  // 3

b: bool = true
i2: int = b.to_int() // 1

s: str = "42"
i3: int = s.to_int() // 42

n: Option<int> = None
i4: int = n.to_int() // 0
```

### `to_float() -> float`
Converts the value to a float.

```tjlang
i: int = 42
f: float = i.to_float()  // 42.0

b: bool = true
f2: float = b.to_float() // 1.0

s: str = "3.14"
f3: float = s.to_float() // 3.14

n: Option<int> = None
f4: float = n.to_float() // 0.0
```

### `to_bool() -> bool`
Converts the value to a boolean.

```tjlang
i: int = 42
b: bool = i.to_bool()  // true

i2: int = 0
b2: bool = i2.to_bool() // false

f: float = 3.14
b3: bool = f.to_bool() // true

f2: float = 0.0
b4: bool = f2.to_bool() // false

s: str = "hello"
b5: bool = s.to_bool() // true

s2: str = ""
b6: bool = s2.to_bool() // false

n: Option<int> = None
b7: bool = n.to_bool() // false
```

### `to_str() -> str`
Converts the value to a string (same as `to_string()`).

```tjlang
i: int = 42
s: str = i.to_str()  // "42"

b: bool = true
s2: str = b.to_str() // "true"
```

## Examples

Here are some comprehensive examples showing how to use these methods:

### Basic Usage

```tjlang
// Integer operations
x: int = 42
IO.print("x is even: " + x.is_even().to_string())
IO.print("x + 1 = " + x.inc().to_string())
IO.print("x is positive: " + x.is_positive().to_string())
// Float operations
f: float = 3.14
IO.print("f rounded: " + f.round().to_string())
IO.print("f ceiling: " + f.ceil().to_string())
IO.print("f is finite: " + f.is_finite().to_string())
// String operations
s: str = "  Hello World  "
IO.print("Original: '" + s + "'")
IO.print("Trimmed: '" + s.trim() + "'")
IO.print("Uppercase: '" + s.upper() + "'")
IO.print("Length: " + s.length().to_string())
// Boolean operations
b: bool = true
IO.print("b: " + b.to_string())
IO.print("not b: " + b.not().to_string())
```

### Type Checking and Conversion

```tjlang
value: int = 42
// Type checking
IO.print("Is int: " + value.is_int().to_string())
IO.print("Is float: " + value.is_float().to_string())
IO.print("Type name: " + value.type_name())
// Conversion
as_float: float = value.to_float()
IO.print("As float: " + as_float.to_string())
as_bool: bool = value.to_bool()
IO.print("As bool: " + as_bool.to_string())
as_string: str = value.to_string()
IO.print("As string: " + as_string)
```

### String Manipulation

```tjlang
text: str = "  hello world  "
// Basic string methods
IO.print("Original: '" + text + "'")
IO.print("Trimmed: '" + text.trim() + "'")
IO.print("Uppercase: '" + text.upper() + "'")
IO.print("Lowercase: '" + text.lower() + "'")
IO.print("Capitalized: '" + text.capitalize() + "'")
IO.print("Reversed: '" + text.reverse() + "'")
IO.print("Length: " + text.length().to_string())
IO.print("Is empty: " + text.is_empty().to_string())
```

### Mathematical Operations

```tjlang
num: float = -3.14
// Mathematical methods
IO.print("Number: " + num.to_string())
IO.print("Absolute: " + num.abs().to_string())
IO.print("Negated: " + num.neg().to_string())
IO.print("Ceiling: " + num.ceil().to_string())
IO.print("Floor: " + num.floor().to_string())
IO.print("Rounded: " + num.round().to_string())
IO.print("Truncated: " + num.trunc().to_string())
IO.print("Is finite: " + num.is_finite().to_string())
IO.print("Is positive: " + num.is_positive().to_string())
```

This comprehensive set of primitive methods provides powerful functionality for working with basic data types in TJLang, enabling rich operations on integers, floats, booleans, strings, and None values.
