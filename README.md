# Polynomial Factorization Modulo p in Rust

This Rust project demonstrates polynomial factorization over finite fields, specifically modulo a prime number $p$. The program identifies roots of a polynomial modulo $p$, constructs linear factors, and performs polynomial division to simplify the polynomial iteratively.

## Features

- **Polynomial Evaluation**: Computes polynomial values at specific points modulo $p$.
- **Root Finding**: Identifies roots of the polynomial in the range { $0, 1, \dots, p-1$ }.
- **Factorization**: Constructs linear factors based on found roots.
- **Division**: Divides the polynomial by linear factors to reduce its degree.

## Example

Consider the polynomial $f(x) = x^3 + 8x + 8$ modulo $p = 17$. The program finds roots and factors as follows:

1. **Input Polynomial**: $x^3 + 8x + 8$
2. **Root Found**: $x = 8$ (i.e., $f(8) \equiv 0 \pmod{17}$ )
3. **Factor**: $x - 8 \equiv x + 9 \pmod{17}$ , represented as `Polynomial { coeffs: [9, 1] }`
4. **Reduced Polynomial**: After dividing by $x + 9$, the polynomial reduces to `Polynomial { coeffs: [8, 4] }`.

### Output

The output from running the code is as follows:
```shell
Evaluating polynomial at x = 2 mod 17: 9
Evaluating at x = 0: 2
Evaluating at x = 1: 11
Evaluating at x = 2: 9
...
Found root: 8
Factors modulo 17: [Polynomial { coeffs: [9, 1] }]
Reduced polynomial after dividing by Polynomial { coeffs: [9, 1] }: Polynomial { coeffs: [8, 4] }
```
## Code Overview
The code consists of the following key components:

- `Polynomial` struct: Represents a polynomial with coefficients stored in a `Vec<BigInt>`.
- `eval` method: Evaluates the polynomial at a given `x` modulo `p`.
- `factors_mod_p` method: Finds linear factors by identifying roots modulo `p`.
- `divide_by_linear_factor` method: Divides the polynomial by a linear factor $(x-r)$ to simplify it.
## Dependencies
The project utilizes the following crates:

- `num-bigint` for arbitrary-precision integers.
- `num-integer` for integer operations.
- `num-traits` for traits like `Zero`, `One`, and `ToPrimitive`.
  
Add the dependencies to your Cargo.toml:
```toml
[dependencies]
num-bigint = "0.4"
num-integer = "0.1"
num-traits = "0.2"
