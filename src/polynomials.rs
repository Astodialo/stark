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
}
