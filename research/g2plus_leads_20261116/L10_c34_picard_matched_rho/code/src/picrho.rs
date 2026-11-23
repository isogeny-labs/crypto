//! rho on PicClass; random classes.
use crate::fp::{Cnt, Fp};
use crate::plane_quartic_pic::{xorshift_next, PicClass, PlaneQuartic, Pt};

pub struct Rho {
    pub zeta: u64,
    pub zeta_inv: u64,
    pub zeta_inv2: u64,
    pub fp: Fp,
}
impl Rho {
    pub fn new(p: u32, zeta: u64) -> Rho {
        let fp = Fp::new(p as u64);
        let zi = fp.inv_u(zeta);
        Rho { zeta, zeta_inv: zi, zeta_inv2: zi * zi % p as u64, fp }
    }
    pub fn rho_point(&self, pq: &PlaneQuartic, pt: Pt) -> Pt {
        pq.normalize_point([pt[0], pt[1], (self.zeta * pt[2] as u64 % self.fp.p) as u32])
    }
    pub fn rho_class(&self, pq: &PlaneQuartic, x: &PicClass, own: &mut Cnt) -> PicClass {
        match x {
            PicClass::Exc(pt) => {
                let z = self.fp.mul(self.zeta, pt[2] as u64, own) as u32;
                PicClass::Exc(pq.normalize_point([pt[0], pt[1], z]))
            }
            PicClass::Typ(m) => {
                let cexp = [0u8, 0, 1, 0, 1, 2];
                let mut rows: Vec<Vec<u32>> = Vec::new();
                for r in 0..3 {
                    let mut row = m[r].to_vec();
                    for j in 0..6 {
                        if cexp[j] == 1 {
                            row[j] = self.fp.mul(row[j] as u64, self.zeta_inv, own) as u32;
                        } else if cexp[j] == 2 {
                            row[j] = self.fp.mul(row[j] as u64, self.zeta_inv2, own) as u32;
                        }
                    }
                    rows.push(row);
                }
                pq.class_from_conics(rows)
            }
        }
    }
}

pub fn random_class(pq: &PlaneQuartic, pts: &[Pt], rng: &mut u64) -> PicClass {
    let mut ps = Vec::new();
    for _ in 0..3 {
        ps.push(pts[(xorshift_next(rng) % pts.len() as u64) as usize]);
    }
    let a = pq.add(&pq.point_class(ps[0]), &pq.point_class(ps[1]));
    pq.add(&a, &pq.point_class(ps[2]))
}

/// T5 identities (rho^3=id, x+rho x+rho^2 x=0, rho additive, rho(point_class P)=point_class(rho P)).
pub fn check_t5(pq: &PlaneQuartic, rho: &Rho, pts: &[Pt], rng: &mut u64, nclass: usize, npts: usize) -> bool {
    let mut own = Cnt::default();
    let mut ok = true;
    for _ in 0..nclass {
        let x = random_class(pq, pts, rng);
        let y = random_class(pq, pts, rng);
        let rx = rho.rho_class(pq, &x, &mut own);
        let r2x = rho.rho_class(pq, &rx, &mut own);
        let r3x = rho.rho_class(pq, &r2x, &mut own);
        ok &= r3x == x;
        let s = pq.add(&pq.add(&x, &rx), &r2x);
        ok &= pq.is_zero(&s);
        let sum = pq.add(&x, &y);
        let ry = rho.rho_class(pq, &y, &mut own);
        ok &= rho.rho_class(pq, &sum, &mut own) == pq.add(&rx, &ry);
    }
    for _ in 0..npts {
        let pt = pts[(xorshift_next(rng) % pts.len() as u64) as usize];
        let a = rho.rho_class(pq, &pq.point_class(pt), &mut own);
        let b = pq.point_class(rho.rho_point(pq, pt));
        ok &= a == b;
    }
    ok
}
