# Rustl
This is my first attempt at both doing something with Rust and making an interpreter for a simple language.

The actual language is something that *looks like lisp* but it definitely isn't, it was just a way of keeping things simple enough.

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
	- `set` : will set a variable name.
	- `if`: will evaluate an expression and execute another if that is true.
	
It doesn't do much, but you can write expressions using the *convenient* polish notation.

	(+ 12 (- 1 2 3))
	:: 8
	
Also set variables and use them later

	(set half 21)
	:: nil
	(set answer (* half 2))
	:: nil
	answer
	:: 42
	
Or do *wonderfully pointless* stuff with conditions!

	(if (= answer 42) (#t))
	:: [#t]
