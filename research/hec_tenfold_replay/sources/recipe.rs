use crate::linear::{put, Sparse};
use hec_native_ic::{fp::Fp, jac::Div};

enum Image {
    Ready(u64),
    Pending {
        u: [u64; 4],
        v: [u64; 4],
        slope: u64,
    },
}
pub struct Recipe {
    hits: [i32; 4],
    g: usize,
    image: Image,
    large: usize,
    n: u64,
    target: bool,
}
impl Recipe {
    pub fn new(
        known: &[(u64, u64)],
        i: usize,
        fb: &[(u64, u64)],
        index: &[usize],
        a: &Div,
        t: u64,
        n: u64,
        target: bool,
        lazy: bool,
        fp: &Fp,
    ) -> Self {
        let g = known.len();
        assert!(g == 2 || g == 4);
        let mut hits = [0; 4];
        for (k, &(x, y)) in known.iter().enumerate() {
            let id = index[x as usize];
            let code = (id + 1) as i32;
            hits[k] = if fb[id].1 == y { code } else { -code };
        }
        let x = fb[i].0;
        let image = if lazy {
            let mut u = [0; 4];
            let mut v = [0; 4];
            u[..g].copy_from_slice(&a.u.c[..g]);
            v[..g].copy_from_slice(&a.v.c[..g]);
            Image::Pending { u, v, slope: t }
        } else {
            Image::Ready(fp.add(a.v.eval(x, fp), fp.mul(t, a.u.eval(x, fp))))
        };
        Self {
            hits,
            g,
            image,
            large: i,
            n,
            target,
        }
    }
    pub fn build(
        &self,
        fb: &[(u64, u64)],
        step: usize,
        m: usize,
        fp: &Fp,
        f: &Fp,
    ) -> (Sparse, u64) {
        let mut row = Sparse::new();
        for &code in &self.hits[..self.g] {
            put(
                &mut row,
                code.unsigned_abs() as usize - 1,
                if code > 0 { 1 } else { f.p - 1 },
                f,
            );
        }
        let (x, canonical) = fb[self.large];
        let y = match &self.image {
            Image::Ready(y) => *y,
            Image::Pending { u, v, slope } => {
                let (mut a, mut b) = (1, 0);
                for j in (0..self.g).rev() {
                    a = fp.add(fp.mul(a, x), u[j]);
                    b = fp.add(fp.mul(b, x), v[j]);
                }
                fp.add(b, fp.mul(*slope, a))
            }
        };
        let sign = if y == canonical {
            1
        } else {
            assert_eq!(y, fp.neg(canonical));
            f.p - 1
        };
        put(&mut row, self.large + 1, sign, f);
        put(&mut row, step, self.n, f);
        if self.target {
            put(&mut row, m, 1, f);
        }
        (row, if self.target { 0 } else { f.p - 1 })
    }
}
