# Expected Error Outputs

## Static Analysis Errors

### test_literal_index_bounds.tj
```
error[A2800]: Array index 5 is out of bounds for array of length 3
  ┌─ errors/static/test_literal_index_bounds.tj:4:12
  │
4 │ IO.println(a.at(5).to_string())
  │            ^^^^^^^
  │
  = note: Valid indices are 0..2
```

## Runtime Errors

(To be added as runtime error span tracking is implemented)

