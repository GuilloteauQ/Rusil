# Variables

## Defining a variable

```
(let VAR_NAME EXPRESSION)
```

Example:

```
(let x 1)
```

## Changing the value of a variable

```
(set VAR_NAME NEW_EXPRESSION)
```

Example:

```
((let x 1) (set x (+ x 2)))
```

