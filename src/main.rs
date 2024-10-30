use num_bigint::{BigInt, RandomBits, ToBigInt};
use rand::Rng;

mod fields;

use crate::fields::*;

fn main() {
    let field = Field::new(BigInt::from(1 + 407) * (BigInt::from(2).pow(119)));
    let mut element = FieldElement::new(
        rand::thread_rng().gen_range(BigInt::ZERO..field.p.clone()),
        Field { p: field.p.clone() },
    );
    let mut element2 = FieldElement::new(
        rand::thread_rng().gen_range(BigInt::ZERO..field.p.clone()),
        Field { p: field.p.clone() },
    );

    println!("Field: {}", element.field.p);
    println!("  Value1: {}", element.value);
    println!("  Value2: {}", element2.value);

    element.add(&mut element2);
    println!("    add: {}", element.add(&mut element2).value);
    println!("    mul: {}", element.mul(&mut element2).value);
    println!("    sub: {}", element.sub(&mut element2).value);
    println!("    div: {}", element.div(&mut element2).value);
    println!("    xor: {}", element.xor(&mut element2).value);
    println!("    neg: {}", element.neg().value);
    println!("    inv: {}", element.inverse().value);
    println!("    eq: {}", element.eq(&mut element2));
    println!("    neq: {}", element.neq(&mut element2));
    println!("    str: {}", element.str());
    println!("    bytes: {}", element.bytes());
    println!("    generator: {}", element.field.generator().value);
    println!(
        "    2nd root of unity: {}",
        element.field.primitive_nth_root(BigInt::from(2)).value
    );
    println!(
        "    16th root of unity: {}",
        element.field.primitive_nth_root(BigInt::from(16)).value
    );
    println!(
        "    sample: {}",
        element
            .field
            .sample(&[
                rand::thread_rng().gen_range(BigInt::ZERO..field.p.clone()),
                rand::thread_rng().gen_range(BigInt::ZERO..field.p.clone()),
                rand::thread_rng().gen_range(BigInt::ZERO..field.p.clone()),
                rand::thread_rng().gen_range(BigInt::ZERO..field.p.clone()),
            ])
            .value
    );
}
