//Diffie-Hellman Starter 2
// Every element of a finite field Fp can be used to make a subgroup H under repeated action of multiplication.
//In other words, for an element g: H = {g, g^2, g^3, ...}

// A primitive element of Fp is an element whose subgroup H = Fp, i.e., every element of Fp,
//can be written as g^n mod p for some integer n. Because of this,
//primitive elements are sometimes called generators of the finite field.

// For the finite field with p = 28151 find the smallest element g which is a primitive element of Fp.
extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;

fn order(g: &BigUint, p: &BigUint) -> BigUint {
    for i in num_iter::range_inclusive(BigUint::from(2u32), p.clone()) {
        if g.modpow(&i, &p) == g.clone() {
            return i;
        }
    }
    return p.clone();
}

fn main() {
    let p = BigUint::from(28151u128);
    for g in num_iter::range_inclusive(BigUint::from(2u32), p.clone()) {
        let o: BigUint = order(&g, &p);
        if o == p {
            println!("{} is a generator", g);
            break;
        }
    }
}
