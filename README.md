# Rustl
This is my first attempt at both doing something with Rust and making an interpreter for a simple language.

The actual language could be defined as a LISP dialect.

## Usage

Just clone the repository to a folder and run

    cargo run

You'll see the REPL:

	Lispr interpreter - v 0.1
	^C to exit

At the moment the language is pretty basic, it can only evaluate single
expressions wrapped by parenthesis.

There is no proper grammar, nor a proper parser, both are things that I
might add in the future.

Supported operations are

- Simple math: `+ - * /`
- Simple comparisons on numbers: `< > <= >= =`
- The keywords
  - `do`: executes the following expressions and returns the last one.
  - `def!`: will set a variable.
  - `if`: will evaluate an expression and execute another if that is true.

You can write expressions using the *convenient* polish notation.

	(+ 12 (- 1 2 3))
	:: 8

Set variables and use them later

	(def! half 21)
	:: nil
	(def! answer (* half 2))
	:: nil
	answer
	:: 42

Or do *wonderfully pointless* stuff with conditions!

	(if (= answer 42) (#t))
	:: [#t]
