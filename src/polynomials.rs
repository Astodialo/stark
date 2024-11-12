use crate::fields::*;
use num_bigint::BigInt;

#[derive(Clone, Debug)]
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
    pub fn xor(&mut self, exponent: FieldElement) -> Polynomial {
        if self.is_zero() {
            return Polynomial {
                coefficients: Vec::new(),
            };
        }
        if exponent.value == BigInt::ZERO {
            return Polynomial {
                coefficients: [self.coefficients[0].field.one()].to_vec(),
            };
        }
        let mut acc = Polynomial {
            coefficients: [self.coefficients[0].field.one()].to_vec(),
        };

        let bytes = exponent.value.to_signed_bytes_be();

        for byte in bytes {
            for i in (0..8).rev() {
                acc = acc.mul(&mut acc.clone());
                if (byte >> i) & 1 == 1 {
                    acc = acc.mul(self);
                }
            }
        }
        acc
    }

    pub fn evaluate(&mut self, point: &mut FieldElement) -> FieldElement {
        let mut xi = point.field.one();
        let mut value = point.field.zero();

        for mut c in self.coefficients.clone() {
            value = value.clone().add(&mut c.mul(&mut xi));
            xi = xi.clone().mul(point);
        }
        value
    }

    pub fn evaluate_domain(&mut self, domain: &mut Vec<FieldElement>) -> Vec<FieldElement> {
        domain
            .iter_mut()
            .map(|mut p| self.evaluate(&mut p))
            .collect()
    }

    pub fn interpolate_domain(
        domain: &mut Vec<FieldElement>,
        values: &mut Vec<FieldElement>,
    ) -> Polynomial {
        assert!(domain.len() == values.len());
        assert!(domain.len() > 0);

        let field = domain[0].field.clone();
        let mut x = Polynomial {
            coefficients: [field.zero(), field.one()].to_vec(),
        };
        let mut acc = Polynomial {
            coefficients: Vec::new(),
        };
        println!("{}", domain.len());
        for i in 0..domain.len() {
            println!("i{}", i);
            let mut prod = Polynomial {
                coefficients: [values[i].clone()].to_vec(),
            };
            for j in 0..domain.len() {
                println!("j{}", j);
                if j == i {
                    continue;
                }
                prod = prod
                    .mul(&mut x.sub(&mut Polynomial {
                        coefficients: [domain[j].clone()].to_vec(),
                    }))
                    .mul(&mut Polynomial {
                        coefficients: [domain[i].clone().sub(&mut domain[j].clone()).inverse()]
                            .to_vec(),
                    })
            }
            acc = acc.add(&mut prod);
        }
        acc
    }

    pub fn zerofier_domain(domain: &mut Vec<FieldElement>) -> Polynomial {
        let field = domain[0].field.clone();
        let mut x = Polynomial {
            coefficients: [field.zero(), field.one()].to_vec(),
        };
        let mut acc = Polynomial {
            coefficients: [field.one()].to_vec(),
        };
        for d in domain {
            acc = acc.mul(&mut x.sub(&mut Polynomial {
                coefficients: [d.clone()].to_vec(),
            }));
        }
        acc
    }

    pub fn scale(&mut self, factor: &mut FieldElement) -> Polynomial {
        let field = self.coefficients[0].field.clone();
        let mut scaled: Vec<FieldElement> = Vec::new();
        for i in 0..self.coefficients.len() {
            let value = factor
                .xor(&mut FieldElement::new(i.into(), field.clone()))
                .mul(&mut self.coefficients[i]);

            scaled.push(value);
        }
        Polynomial {
            coefficients: scaled,
        }
    }

    pub fn test_colinearity(points: &mut Vec<(FieldElement, FieldElement)>) -> bool {
        let mut domain: Vec<FieldElement> = points.iter().map(|point| point.0.clone()).collect();
        let mut values: Vec<FieldElement> = points.iter().map(|point| point.1.clone()).collect();
        println!("{:?}", domain);
        println!("{:?}", domain.len());
        println!("{:?}", values);
        println!("{:?}", values.len());

        let poly = Polynomial::interpolate_domain(&mut domain, &mut values);
        println!("{:?}", poly);
        poly.degree() <= 1
    }
}
