use crate::fields::*;
use num_bigint::BigInt;

#[derive(Clone)]
pub struct Polynomial {
    pub coefficients: Vec<FieldElement>,
}

impl Polynomial {
    pub fn new(coef: Vec<FieldElement>) -> Polynomial {
        Polynomial { coefficients: coef }
    }

    pub fn degree(&self) -> i32 {
        if self.coefficients.is_empty() {
            return -1;
        }
        let zero = self.coefficients[0].field.zero();
        if self
            .coefficients
            .iter()
            .all(|c| c == &self.coefficients[0].field.zero())
        {
            return -1;
        }
        let mut maxindex: i32 = 0;
        for (i, coef) in self.coefficients.iter().enumerate() {
            if coef != &zero {
                maxindex = i as i32;
            }
        }
        maxindex
    }

    pub fn neg(&mut self) -> Polynomial {
        let mut negs = self.coefficients.clone();
        let negs = negs
            .iter_mut()
            .map(|c| c.neg())
            .collect::<Vec<FieldElement>>();
        Polynomial { coefficients: negs }
    }

    pub fn add(&mut self, other: &mut Polynomial) -> Polynomial {
        if self.degree() == -1 {
            return other.clone();
        } else if other.degree() == -1 {
            return self.clone();
        }
        let field = self.coefficients[0].field.clone();
        let degree = self.degree().max(other.degree()) + 1;
        let mut coeffs: Vec<FieldElement> = vec![field.zero(); degree.try_into().unwrap()];
        for (i, coeff) in self.coefficients.iter_mut().enumerate() {
            coeffs[i] = coeffs[i].add(coeff);
        }
        for (i, coeff) in other.coefficients.iter_mut().enumerate() {
            coeffs[i] = coeffs[i].add(coeff);
        }
        Polynomial {
            coefficients: coeffs,
        }
    }

    pub fn sub(&mut self, other: &mut Polynomial) -> Polynomial {
        self.add(&mut other.neg())
    }

    pub fn mul(&mut self, other: &mut Polynomial) -> Polynomial {
        if self.coefficients == [] || other.coefficients == [] {
            return Polynomial {
                coefficients: Vec::new(),
            };
        }
        let zero = self.coefficients[0].field.zero();
        let field = self.coefficients[0].field.clone();
        let degree = self.degree().checked_add(other.degree()).unwrap();
        let mut buf: Vec<FieldElement> = vec![field.zero(); degree as usize + 2];
        for (i, s_coeff) in self.coefficients.iter_mut().enumerate() {
            if s_coeff.is_zero() {
                continue;
            }
            for (j, o_coeff) in other.coefficients.iter_mut().enumerate() {
                buf[i + j] = buf[i + j].add(&mut s_coeff.mul(o_coeff));
            }
        }
        Polynomial { coefficients: buf }
    }
    pub fn eq(&mut self, other: &mut Polynomial) -> bool {
        if self.degree() != other.degree() {
            return false;
        }
        if self.degree() == -1 {
            return true;
        }
        self.coefficients.eq(&other.coefficients)
    }
    pub fn neq(&mut self, other: &mut Polynomial) -> bool {
        !self.eq(other)
    }
    pub fn is_zero(&self) -> bool {
        if self.degree() == -1 {
            return true;
        }
        false
    }
    pub fn leading_coefficient(&self) -> FieldElement {
        self.coefficients[self.degree() as usize].clone()
    }
    pub fn divide(
        &mut self,
        denominator: &mut Polynomial,
    ) -> Result<(Polynomial, Polynomial), &'static str> {
        if denominator.degree() == -1 {
            println!("w");
            return Err("dividing with zero polynomial");
        }
        if self.degree() < denominator.degree() {
            return Ok((
                Polynomial {
                    coefficients: [].to_vec(),
                },
                self.clone(),
            ));
        }
        let field = denominator.coefficients[0].field.clone();
        let mut remainder = Polynomial {
            coefficients: self.coefficients.clone(),
        };
        let mut quotient_coefficients =
            vec![field.zero().clone(); (self.degree() - denominator.degree() + 1) as usize];
        for (i, coeff) in quotient_coefficients.clone().iter_mut().enumerate() {
            if remainder.degree() < denominator.degree() {
                break;
            }
            let coefficient = remainder
                .leading_coefficient()
                .div(&mut denominator.leading_coefficient());
            let shift = remainder.degree() - denominator.degree();
            let mut subtractee = Polynomial {
                coefficients: vec![field.zero().clone(); shift as usize],
            };
            subtractee.coefficients.push(coefficient.clone());
            subtractee.mul(denominator);
            quotient_coefficients[shift as usize] = coefficient.clone();
            remainder = remainder.sub(&mut subtractee);
        }
        let quotient = Polynomial {
            coefficients: quotient_coefficients,
        };
        Ok((quotient, remainder))
    }
    pub fn truediv(&mut self, other: &mut Polynomial) -> Polynomial {
        let (quo, rem) = self.divide(other).unwrap();
        assert!(rem.is_zero());
        quo
    }
    pub fn modulo(&mut self, other: &mut Polynomial) -> Polynomial {
        let (quo, rem) = self.divide(other).unwrap();
        rem
    }
    pub fn xor(&mut self, exponent: &mut Polynomial) -> Polynomial {
        if self.is_zero() {
            return Polynomial {
                coefficients: Vec::new(),
            };
        }
        if exponent.is_zero() {
            return Polynomial {
                coefficients: [self.coefficients[0].field.one()].to_vec(),
            };
        }
        let mut acc = Polynomial {
            coefficients: [self.coefficients[0].field.one()].to_vec(),
        };
        acc
    }
}
