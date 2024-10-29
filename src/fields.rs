use num_bigint::BigInt;

pub fn xgcd(x: i128, y: i128) -> (i128, i128, i128) {
    let (mut old_r, mut r) = (x, y);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let quotient = old_r / r;

        update_step(&mut r, &mut old_r, quotient);
        update_step(&mut s, &mut old_s, quotient);
        update_step(&mut t, &mut old_t, quotient);
    }

    (old_s, old_t, old_r)
}

fn update_step(a: &mut i128, old_a: &mut i128, q: i128) {
    let temp = *a;
    *a = *old_a - q * temp;
    *old_a = temp;
}

#[derive(Clone)]
pub struct FieldElement {
    pub value: i128,
    pub field: Field,
}

#[derive(Clone)]
pub struct Field {
    pub p: i128,
}

impl FieldElement {
    pub fn new(value: i128, field: Field) -> FieldElement {
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
        let mut acc = FieldElement::new(1, Field::new(self.field.p.clone()));
        let mut val = FieldElement::new(self.value.clone(), self.field.clone());

        let binary_len = format!("{:b}", exponent.value).len();

        for i in (0..binary_len).rev() {
            acc = acc.mul(&mut acc.clone());
            if 1 << i & exponent.value != 0 {
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
        if self.value == 0 {
            true
        } else {
            false
        }
    }
}

impl Field {
    pub fn new(p: i128) -> Field {
        Field { p }
    }

    pub fn zero(&self) -> FieldElement {
        FieldElement {
            value: 0,
            field: self.clone(),
        }
    }

    pub fn one(&self) -> FieldElement {
        FieldElement {
            value: 1,
            field: self.clone(),
        }
    }

    pub fn multiply(&mut self, left: &mut FieldElement, right: &mut FieldElement) -> FieldElement {
        FieldElement {
            value: (left.value * right.value) % self.p,
            field: self.clone(),
        }
    }

    pub fn add(&mut self, left: &mut FieldElement, right: &mut FieldElement) -> FieldElement {
        FieldElement {
            value: (left.value + right.value) % self.p,
            field: self.clone(),
        }
    }

    pub fn subtract(&mut self, left: &mut FieldElement, right: &mut FieldElement) -> FieldElement {
        FieldElement {
            value: (self.p + left.value - right.value) % self.p,
            field: self.clone(),
        }
    }

    pub fn negate(&mut self, operand: &mut FieldElement) -> FieldElement {
        FieldElement {
            value: (self.p - operand.value) % self.p,
            field: self.clone(),
        }
    }

    pub fn inverse(&mut self, operand: &mut FieldElement) -> FieldElement {
        FieldElement {
            value: xgcd(operand.value, self.p).0,
            field: self.clone(),
        }
    }

    pub fn divide(&mut self, left: &mut FieldElement, right: &mut FieldElement) -> FieldElement {
        assert!(!right.is_zero());
        let (a, _, _) = xgcd(right.value, self.p);
        FieldElement {
            value: left.value * a % self.p,
            field: self.clone(),
        }
    }

    pub fn generator(&self) -> FieldElement {
        assert_eq!(self.p, (1 + 407 * (2 ^ 119)));

        FieldElement::new(85408008396924667383611388730472331217, self.clone())
    }

    pub fn primitive_nth_root(&self, n: BigInt) -> FieldElement {
        if self.p == 1 + 407 * (2 ^ 119) {
            let x: i128 = 1 << 119;
            let r: i128 = 85408008396924667383611388730472331217;
            assert!(n <= BigInt::from(x) && (n.clone() & (n.clone() - 1)) == BigInt::ZERO);
            let mut root = BigInt::from(r);
            let mut order: BigInt = BigInt::from(x);

            while order != n {
                root = (root ^ BigInt::from(2)) % BigInt::from(self.p);
                order = order / 2;
            }
            FieldElement {
                value: root.to_string().parse().unwrap(),
                field: self.clone(),
            }
        } else {
            assert!(false);
            FieldElement::new(0, self.clone())
        }
    }

    pub fn sample(&self, ba: &[i128]) -> FieldElement {
        let mut acc = 0;
        for &b in ba {
            acc = ((acc << 8) ^ b) % self.p;
        }
        FieldElement::new(acc, self.clone())
    }
}
