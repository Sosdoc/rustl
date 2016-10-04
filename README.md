# Rustl
This is an interpreter for a simple Lisp-like language. It supports evaluation of expressions, defining and running closures. This was done as part of my efforts in learning Rust. 

## Usage

Just clone the repository to a folder and run

    cargo run

You'll see the REPL:

	Lispr interpreter - v 0.1
	^C to exit

Supported operations are

- Simple math: `+ - * /`
- Simple comparisons on numbers: `< > <= >= =`
- The keywords
  - `list`: returns a list with the arguments provided
  - `do`: executes the following list of expressions and returns the last one.
  - `def!`: will set a variable.
  - `if`: will evaluate an expression and execute another if that is true.
  - `lambda`: creates a new closure with the parameters and specified body.

You can write expressions using the *convenient* polish notation.

```
>> (+ 12 (- 5 2))
15
```

Set variables and use them later

```
>> (def! half 21)
nil
>> (def! answer (* half 2))
nil
>> answer
42
```

Do *wonderfully pointless* stuff with conditions!
```
>> (if (= answer 42) #t)
#t
```

Or, you know, declare and use functions with the lambda keyword:

```
>> ( def! max (lambda (a b) (if (> a b) a b)) )
nil
>> (max 21 42)
42
```

Run multiple expressions (a list of expressions) using the `do` keyword.
Consider this program:

```
(do list 
    (def! x 10) 
    (def! add_x (
        lambda (a) (
             + a x 
        ) 
    ))
    (add_x 90)
)
```
It defines the binding `x` and the lambda `add_x`, which will reference `x` when executed.
When run in the interpreter (note that the REPL doesn't parse over multiple lines):

```
>> (do list (def! x 10) (def! add_x (lambda (a) (+ a x))) (add_x 90))
100
>> (add_x -10)
0
```
