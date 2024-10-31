use num_bigint::BigInt;

pub fn xgcd(x: BigInt, y: BigInt) -> (BigInt, BigInt, BigInt) {
    let (mut old_r, mut r) = (x, y);
    let (mut old_s, mut s) = (BigInt::from(1), BigInt::ZERO);
    let (mut old_t, mut t) = (BigInt::ZERO, BigInt::from(1));

    while r != BigInt::ZERO {
        let quotient = old_r.clone() / r.clone();

        update_step(&mut r, &mut old_r, quotient.clone());
        update_step(&mut s, &mut old_s, quotient.clone());
        update_step(&mut t, &mut old_t, quotient);
    }

    (old_s, old_t, old_r)
}

fn update_step(a: &mut BigInt, old_a: &mut BigInt, q: BigInt) {
    let temp = a.clone();
    *a = old_a.clone() - q * temp.clone();
    *old_a = temp;
}

#[derive(Clone, PartialEq, Eq)]
pub struct FieldElement {
    pub value: BigInt,
    pub field: Field,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Field {
    pub p: BigInt,
}

impl FieldElement {
    pub fn new(value: BigInt, field: Field) -> FieldElement {
        FieldElement { value, field }
    }

    pub fn add(&mut self, right: &mut FieldElement) -> FieldElement {
        self.field.add(&mut self.clone(), right)
    }

    pub fn mul(&mut self, right: &mut FieldElement) -> FieldElement {
        self.field.multiply(&mut self.clone(), right)
    }

    pub fn sub(&mut self, right: &mut FieldElement) -> FieldElement {
        self.field.subtract(&mut self.clone(), right)
    }

    pub fn div(&mut self, right: &mut FieldElement) -> FieldElement {
        self.field.divide(&mut self.clone(), right)
    }

    pub fn neg(&mut self) -> FieldElement {
        self.field.negate(&mut self.clone())
    }

    pub fn inverse(&mut self) -> FieldElement {
        self.field.inverse(&mut self.clone())
    }

    pub fn xor(&self, exponent: &mut FieldElement) -> FieldElement {
        let mut acc = FieldElement::new(BigInt::from(1), Field::new(self.field.p.clone()));
        let mut val = FieldElement::new(self.value.clone(), self.field.clone());

        let binary_len = format!("{:b}", exponent.value).len();

        for i in (0..binary_len).rev() {
            acc = acc.mul(&mut acc.clone());
            if BigInt::from(2).pow(i as u32) & exponent.value.clone() != BigInt::ZERO {
                acc = acc.mul(&mut val);
            }
        }
        acc
    }

    pub fn eq(&self, other: &mut FieldElement) -> bool {
        self.value == other.value
    }

    pub fn neq(&self, other: &mut FieldElement) -> bool {
        self.value != other.value
    }

    pub fn str(&self) -> String {
        self.value.to_string()
    }

    pub fn bytes(&self) -> String {
        format!("{:b}", self.value)
    }

    pub fn is_zero(&self) -> bool {
        if self.value == BigInt::ZERO {
            true
        } else {
            false
        }
    }
}

impl Field {
    pub fn new(p: BigInt) -> Field {
        Field { p }
    }

    pub fn zero(&self) -> FieldElement {
        FieldElement {
            value: BigInt::ZERO,
            field: self.clone(),
        }
    }

    pub fn one(&self) -> FieldElement {
        FieldElement {
            value: BigInt::from(1),
            field: self.clone(),
        }
    }

    pub fn multiply(&mut self, left: &mut FieldElement, right: &mut FieldElement) -> FieldElement {
        FieldElement {
            value: (left.value.clone() * right.value.clone()) % self.p.clone(),
            field: self.clone(),
        }
    }

    pub fn add(&mut self, left: &mut FieldElement, right: &mut FieldElement) -> FieldElement {
        FieldElement {
            value: (left.value.clone() + right.value.clone()) % self.p.clone(),
            field: self.clone(),
        }
    }

    pub fn subtract(&mut self, left: &mut FieldElement, right: &mut FieldElement) -> FieldElement {
        FieldElement {
            value: (self.p.clone() + left.value.clone() - right.value.clone()) % self.p.clone(),
            field: self.clone(),
        }
    }

    pub fn negate(&mut self, operand: &mut FieldElement) -> FieldElement {
        FieldElement {
            value: (self.p.clone() - operand.value.clone()) % self.p.clone(),
            field: self.clone(),
        }
    }

    pub fn inverse(&mut self, operand: &mut FieldElement) -> FieldElement {
        FieldElement {
            value: xgcd(operand.value.clone(), self.p.clone()).0,
            field: self.clone(),
        }
    }

    pub fn divide(&mut self, left: &mut FieldElement, right: &mut FieldElement) -> FieldElement {
        assert!(!right.is_zero());
        let (a, _, _) = xgcd(right.value.clone(), self.p.clone());
        FieldElement {
            value: left.value.clone() * a % self.p.clone(),
            field: self.clone(),
        }
    }

    pub fn generator(&self) -> FieldElement {
        assert_eq!(
            self.p.clone(),
            BigInt::from((1 + 407) * (BigInt::from(2).pow(119)))
        );

        FieldElement::new(
            "85408008396924667383611388730472331217"
                .parse::<BigInt>()
                .unwrap(),
            self.clone(),
        )
    }

    pub fn primitive_nth_root(&self, n: BigInt) -> FieldElement {
        if self.p == BigInt::from((1 + 407) * (BigInt::from(2).pow(119))) {
            let x: BigInt = BigInt::from(2).pow(119);
            let r: BigInt = "85408008396924667383611388730472331217"
                .parse::<BigInt>()
                .unwrap();
            assert!(n <= BigInt::from(x.clone()) && (n.clone() & (n.clone() - 1)) == BigInt::ZERO);
            let mut root = BigInt::from(r);
            let mut order: BigInt = BigInt::from(x);

            while order != n {
                root = (root ^ BigInt::from(2)) % BigInt::from(self.p.clone());
                order = order / 2;
            }
            FieldElement {
                value: root.to_string().parse().unwrap(),
                field: self.clone(),
            }
        } else {
            assert!(false);
            FieldElement::new(BigInt::ZERO, self.clone())
        }
    }

    pub fn sample(&self, ba: &[BigInt]) -> FieldElement {
        let mut acc = BigInt::from(1);
        for b in ba {
            acc = BigInt::from(acc << 8).modpow(b, &self.p.clone());
        }
        FieldElement::new(acc, self.clone())
    }
}
