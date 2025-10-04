# Expected Error Outputs

This document describes the expected diagnostic output for each test file in the `errors/` directory.

## Static Analysis Errors

### `errors/static/test_literal_index_bounds.tj`

**Expected Output:**
```
error[A2800]: Array index 5 is out of bounds for array of length 3
  ┌─ errors/static/test_literal_index_bounds.tj:4:24
  │
4 │ IO.println([1, 2, 3].at(5).to_string())
  │                        ^^^
  │
  = Valid indices are 0..3
```
**Explanation:** The static analyzer should detect that accessing index `5` on a literal array `[1, 2, 3]` (which has length 3, valid indices 0-2) is an out-of-bounds error. This should be caught before runtime.

### `errors/static/test_literal_div_by_zero.tj`

**Expected Output:**
```
error[A2801]: Literal division by zero detected
  ┌─ errors/static/test_literal_div_by_zero.tj:4:15
  │
4 │ x: int = 10 / 0
  │               ^
  │
  = Division by zero will cause a runtime panic
```
**Explanation:** The static analyzer should detect that `10` is being divided by `0` and report a static error.

### `errors/static/test_literal_modulo_by_zero.tj`

**Expected Output:**
```
error[A2801]: Literal modulo by zero detected
  ┌─ errors/static/test_literal_modulo_by_zero.tj:4:10
  │
4 │ y: int = 15 % 0
  │          ^^^^^^
  │
  = Modulo by zero will cause a runtime panic
```
**Explanation:** The static analyzer should detect that `15` is being modulo by `0` and report a static error.

### `errors/static/test_float_div_by_zero.tj`

**Expected Output:**
```
error[A2801]: Literal division by zero detected
  ┌─ errors/static/test_float_div_by_zero.tj:4:17
  │
4 │ f: float = 3.14 / 0.0
  │                 ^^^^
  │
  = Division by zero will cause a runtime panic
```
**Explanation:** The static analyzer should detect that `3.14` is being divided by `0.0` and report a static error.

### `errors/static/test_undefined_variable.tj`

**Expected Output:**
```
error[A2803]: Variable 'undefined_var' is used before being declared
  ┌─ errors/static/test_undefined_variable.tj:4:15
  │
4 │ result: int = undefined_var + 10
  │               ^^^^^^^^^^^^^
  │
  = Variable 'undefined_var' must be declared before use.
```
**Explanation:** The static analyzer should detect that `undefined_var` is used before being declared.

**Known Limitations**:
- The AST doesn't currently track spans for `Expression::Variable` nodes, so the exact location may not be precise.
- Built-in modules like `IO` are currently flagged as undefined (this will be fixed by adding a whitelist for stdlib names).

### `errors/static/test_undefined_var_in_function.tj`

**Expected Output:**
```
error[A2803]: Variable 'undefined_param' is used before being declared
  ┌─ errors/static/test_undefined_var_in_function.tj:5:15
  │
5 │     y: int = x + undefined_param
  │              ^^^^^^^^^^^^^^^^^
  │
  = Variable 'undefined_param' must be declared before use.
```
**Explanation:** The static analyzer should detect that `undefined_param` is used inside the function body but never declared.

## Runtime Errors

(To be added as runtime error system is refactored)
