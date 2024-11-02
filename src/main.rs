use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{Zero, One, ToPrimitive};

#[derive(Debug, Clone)]
struct Polynomial {
    coeffs: Vec<BigInt>, // Polynomial coefficients, starting from the constant term
}

impl Polynomial {
    // Evaluate the polynomial at a given x modulo p
    fn eval(&self, x: &BigInt, p: &BigInt) -> BigInt {
        let mut result = BigInt::zero();
        let mut x_pow = BigInt::one(); // x^i for i = 0 initially
        for coeff in &self.coeffs {
            result = (result + coeff * &x_pow) % p;
            x_pow = (x_pow * x) % p;
        }
        result
    }

    // Find linear factors modulo p (i.e., check for roots)
    fn factors_mod_p(&self, p: &BigInt) -> Vec<Polynomial> {
        let mut factors = Vec::new();

        // Check for roots in the range 0 to p-1
        for r in 0..p.to_u32().expect("p should fit in u32") {
            let r_bigint = BigInt::from(r);
            let eval_result = self.eval(&r_bigint, p);
            println!("Evaluating at x = {}: {}", r, eval_result); // Debugging line
            
            if eval_result.is_zero() {
                // If r is a root, create the linear factor (x - r)
                let factor = Polynomial {
                    coeffs: vec![(-&r_bigint).mod_floor(p), BigInt::one()],
                };
                factors.push(factor);
                println!("Found root: {}", r); // Debugging line
            }
        }

        if factors.is_empty() {
            println!("No linear factors found modulo {}", p);
        }

        factors
    }

    // Function to divide the polynomial by a linear factor (x - r)
    fn divide_by_linear_factor(&self, factor: &Polynomial, p: &BigInt) -> Polynomial {
        let r = -&factor.coeffs[0]; // The root from (x - r)

        let mut new_coeffs = vec![BigInt::zero(); self.coeffs.len() - 1];
        let mut remainder = BigInt::zero();

        for (i, coeff) in self.coeffs.iter().rev().enumerate() {
            if i == 0 {
                // For the last coefficient, just add it to the remainder
                remainder = (remainder * r.clone()) + coeff;
            } else {
                remainder = (remainder * r.clone()) + coeff;
                new_coeffs[i - 1] = (remainder.clone() % p + p) % p; // Ensure coefficients are in range [0, p)
            }
        }

        // Remove leading zeroes from the new coefficients
        while new_coeffs.len() > 1 && new_coeffs.last() == Some(&BigInt::zero()) {
            new_coeffs.pop();
        }

        Polynomial {
            coeffs: new_coeffs,
        }
    }
}

fn main() {
    let p = BigInt::from(17); // Example modulus p
    let poly = Polynomial {
        coeffs: vec![BigInt::from(2), BigInt::from(8), BigInt::zero(), BigInt::one()], // Represents x^3 + 8x + 8
    };

    println!("Evaluating polynomial at x = 2 mod {}: {}", p, poly.eval(&BigInt::from(2), &p));

    let factors = poly.factors_mod_p(&p);
    println!("Factors modulo {}: {:?}", p, factors);

    // Perform division for each factor found
    for factor in &factors {
        let reduced_poly = poly.divide_by_linear_factor(factor, &p);
        println!("Reduced polynomial after dividing by {:?}: {:?}", factor, reduced_poly);
    }
}
