use num_bigint::{BigInt, RandomBits, ToBigInt};
use rand::Rng;

mod fields;
mod polynomials;

use crate::fields::*;
use crate::polynomials::*;

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
    let mut poly = Polynomial::new([element.clone(), element2.clone(), element2.clone()].to_vec());
    let mut poly_zero = Polynomial::new([element.field.zero(), element.field.zero()].to_vec());
    let mut poly_2 =
        Polynomial::new([element.field.zero(), element.clone(), element.field.zero()].to_vec());

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
    println!(
        "    poly coefficients: {}, {}",
        poly.coefficients[0].value, poly.coefficients[1].value
    );
    println!("    zero poly degree: {}", poly_zero.degree());
    println!("    poly degree: {}", poly_2.degree());
    println!("    poly neg: {}", poly.neg().coefficients[0].value);
    println!(
        "    poly add: {}",
        poly.add(&mut poly_2).coefficients[1].value
    );
    println!(
        "    poly sub: {}",
        poly.sub(&mut poly_2).coefficients[1].value
    );
    println!(
        "    poly mul: {}",
        poly.mul(&mut poly_2).coefficients[1].value
    );
    println!("    poly neq: {}", poly.neq(&mut poly_2));
    println!("    poly eq: {}", poly.eq(&mut poly.clone()));
    println!("    poly is_zero: {}", poly_zero.is_zero());
    println!(
        "    poly leading coefficient: {}",
        poly.leading_coefficient().value
    );
    println!(
        "    poly division: {},{}",
        poly.divide(&mut poly_2.clone()).unwrap().1.coefficients[0].value,
        poly.divide(&mut poly_2.clone()).unwrap().1.is_zero(),
    );
    println!(
        "    poly xor: {}",
        poly.xor(FieldElement {
            value: 333.to_bigint().unwrap(),
            field: field.clone()
        })
        .coefficients
        .len(),
    );
    println!("    poly eval: {:?}", poly.evaluate(&mut element));
    println!(
        "    poly domain_eval: {:?}",
        poly.evaluate_domain(&mut [element.clone(), element2.clone()].to_vec())
    );
    println!(
        "    poly interpolation: {:?}",
        Polynomial::interpolate_domain(
            &mut poly.coefficients.clone(),
            &mut poly.evaluate_domain(&mut poly.coefficients.clone())
        )
        .coefficients
        .len()
    );
    println!(
        "    poly zerofier: {:?}",
        Polynomial::zerofier_domain(&mut poly.coefficients.clone())
            .coefficients
            .len()
    );
    println!(
        "    poly scale: {:?}",
        poly.scale(&mut FieldElement::new(
            333.to_bigint().unwrap(),
            field.clone()
        ))
        .leading_coefficient()
        .value
    );
    println!(
        "    poly colinearity: {}",
        Polynomial::test_colinearity(
            &mut [
                (element.clone(), element2.clone()),
                (element2.clone(), element.clone())
            ]
            .to_vec()
        )
    )
}
