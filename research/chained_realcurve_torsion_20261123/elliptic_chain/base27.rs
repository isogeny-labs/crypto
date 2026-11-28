//! Prior qualified F27 arithmetic; selected bodies unchanged.
use crate::inherited::Fld;

pub(crate) struct F27;

impl F27 {
    pub(crate) fn pow(&self, mut a: u32, mut n: u32) -> u32 {
        let mut value = 1;
        while n != 0 {
            if n & 1 == 1 {
                value = self.mul(&value, &a);
            }
            n >>= 1;
            if n != 0 {
                a = self.mul(&a, &a);
            }
        }
        value
    }
}

impl Fld for F27 {
    type E = u32;
    fn zero(&self) -> u32 {
        0
    }
    fn one(&self) -> u32 {
        1
    }
    fn add(&self, a: &u32, b: &u32) -> u32 {
        (a % 3 + b % 3) % 3 + 3 * ((a / 3 % 3 + b / 3 % 3) % 3) + 9 * ((a / 9 + b / 9) % 3)
    }
    fn sub(&self, a: &u32, b: &u32) -> u32 {
        self.add(a, &self.neg(b))
    }
    fn neg(&self, a: &u32) -> u32 {
        self.add(a, a)
    }
    fn mul(&self, a: &u32, b: &u32) -> u32 {
        let aa = [a % 3, a / 3 % 3, a / 9];
        let bb = [b % 3, b / 3 % 3, b / 9];
        let mut product = [0; 5];
        for (i, x) in aa.iter().enumerate() {
            for (j, y) in bb.iter().enumerate() {
                product[i + j] = (product[i + j] + x * y) % 3;
            }
        }
        for k in (3..5).rev() {
            let c = product[k];
            product[k - 3] = (product[k - 3] + c) % 3;
            product[k - 2] = (product[k - 2] + c) % 3;
            product[k] = 0;
        }
        product[0] + 3 * product[1] + 9 * product[2]
    }
    fn inv(&self, a: &u32) -> u32 {
        assert_ne!(*a, 0);
        self.pow(*a, 25)
    }
}
