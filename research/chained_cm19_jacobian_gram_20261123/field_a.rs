// CM19 single F7 fixture: digit-field engine parameterized from frozen F5 C.
#[derive(Clone)]
pub struct Field {
    p: usize,
    degree: usize,
    reduction: Vec<usize>,
    pub q: usize,
    plus: Vec<usize>,
    times: Vec<usize>,
}

impl Field {
    fn digits(&self, mut a: usize) -> Vec<usize> {
        let mut out = vec![0; self.degree];
        for value in &mut out {
            *value = a % self.p;
            a /= self.p;
        }
        assert_eq!(a, 0);
        out
    }

    fn encode(&self, values: &[usize]) -> usize {
        values.iter().rev().fold(0, |a, &b| a * self.p + b)
    }

    fn add_reference(&self, a: usize, b: usize) -> usize {
        let aa = self.digits(a);
        let bb = self.digits(b);
        self.encode(
            &aa.iter()
                .zip(bb)
                .map(|(x, y)| (x + y) % self.p)
                .collect::<Vec<_>>(),
        )
    }

    fn product_reference(&self, a: usize, b: usize) -> usize {
        let aa = self.digits(a);
        let bb = self.digits(b);
        let mut product = vec![0; 2 * self.degree - 1];
        for (i, x) in aa.iter().enumerate() {
            for (j, y) in bb.iter().enumerate() {
                product[i + j] = (product[i + j] + x * y) % self.p;
            }
        }
        for k in (self.degree..product.len()).rev() {
            let coefficient = product[k];
            for (i, &r) in self.reduction.iter().enumerate() {
                product[k - self.degree + i] =
                    (product[k - self.degree + i] + coefficient * r) % self.p;
            }
            product[k] = 0;
        }
        self.encode(&product[..self.degree])
    }

    pub fn new(p: usize, degree: usize, reduction: Vec<usize>) -> Self {
        assert_eq!(reduction.len(), degree);
        let q = p.pow(degree as u32);
        let mut field = Self {
            p,
            degree,
            reduction,
            q,
            plus: vec![0; q * q],
            times: vec![0; q * q],
        };
        for a in 0..q {
            for b in 0..q {
                field.plus[a * q + b] = field.add_reference(a, b);
                field.times[a * q + b] = field.product_reference(a, b);
            }
        }
        field
    }

    pub fn add(&self, a: usize, b: usize) -> usize {
        self.plus[a * self.q + b]
    }

    pub fn mul(&self, a: usize, b: usize) -> usize {
        self.times[a * self.q + b]
    }

    pub fn neg(&self, a: usize) -> usize {
        self.mul(self.p - 1, a)
    }

    pub fn sub(&self, a: usize, b: usize) -> usize {
        self.add(a, self.neg(b))
    }

    pub fn square(&self, a: usize) -> usize {
        self.mul(a, a)
    }

    pub fn pow(&self, mut a: usize, mut exponent: usize) -> usize {
        let mut out = 1;
        while exponent != 0 {
            if exponent & 1 == 1 {
                out = self.mul(out, a);
            }
            a = self.square(a);
            exponent >>= 1;
        }
        out
    }

    pub fn div(&self, a: usize, b: usize) -> usize {
        assert_ne!(b, 0);
        self.mul(a, self.pow(b, self.q - 2))
    }
}
