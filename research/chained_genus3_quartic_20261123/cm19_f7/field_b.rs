// CM19 single F7 fixture: digit-field engine parameterized from frozen F5 C.
#[derive(Clone)]
pub struct Field {
    p: usize,
    degree: usize,
    reduction: Vec<usize>,
    pub q: usize,
    plus: Vec<usize>,
    times: Vec<usize>,
    roots: Vec<usize>,
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

    fn horner_product(&self, a: usize, b: usize) -> usize {
        let aa = self.digits(a);
        let bb = self.digits(b);
        let mut accum = vec![0; self.degree];
        for &coefficient in bb.iter().rev() {
            let top = accum[self.degree - 1];
            let mut shifted = vec![0; self.degree];
            shifted[1..].copy_from_slice(&accum[..self.degree - 1]);
            for (i, value) in shifted.iter_mut().enumerate() {
                *value = (*value + top * self.reduction[i] + coefficient * aa[i]) % self.p;
            }
            accum = shifted;
        }
        self.encode(&accum)
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
            roots: vec![0; q],
        };
        for a in 0..q {
            for b in 0..q {
                field.plus[a * q + b] = field.add_reference(a, b);
                field.times[a * q + b] = field.product_reference(a, b);
            }
        }
        for a in 0..q {
            let square = field.mul(a, a);
            field.roots[square] += 1;
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

    pub fn check(&self) {
        for a in 0..self.q {
            assert_eq!(self.pow(a, self.q), a);
            assert_eq!(self.add(a, self.neg(a)), 0);
            if a != 0 {
                assert_eq!(self.mul(a, self.pow(a, self.q - 2)), 1);
            }
            for b in 0..self.q {
                assert_eq!(self.mul(a, b), self.horner_product(a, b));
                assert_eq!(self.add(a, b), self.add(b, a));
                assert_eq!(self.mul(a, b), self.mul(b, a));
                assert_eq!(
                    self.pow(self.add(a, b), self.p),
                    self.add(self.pow(a, self.p), self.pow(b, self.p))
                );
                assert_eq!(
                    self.pow(self.mul(a, b), self.p),
                    self.mul(self.pow(a, self.p), self.pow(b, self.p))
                );
                for c in 0..self.q {
                    assert_eq!(self.add(self.add(a, b), c), self.add(a, self.add(b, c)));
                    assert_eq!(self.mul(self.mul(a, b), c), self.mul(a, self.mul(b, c)));
                    assert_eq!(
                        self.mul(a, self.add(b, c)),
                        self.add(self.mul(a, b), self.mul(a, c))
                    );
                }
            }
        }
        println!(
            "FIELD\tq={}\tdegree={}\treduction={:?}\tpairs={}\ttriples={}\tPASS",
            self.q,
            self.degree,
            self.reduction,
            self.q * self.q,
            self.q * self.q * self.q
        );
    }
}
