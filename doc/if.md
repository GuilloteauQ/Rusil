# If statements

In Rusil, an ``if`` statement has an ``else`` clause, however it can be empty if not needed.

```
(if CONDITION TRUE_STATEMENT FALSE_STATEMENT)
```

Example:

```
(if (> x 0) (set x (+ x 1)) (set x (- x 1)))
```
