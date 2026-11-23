//! Short-box hashed factor bases: 2D Gauss-reduced general slots (`REPORT.md` §1)
//! and 1D prime-inverse-slope `[A/2, A]` slots (`FOLLOWUP.md` §1), plus bounded
//! prime-curve coverage and unique-witness measurement.

use crate::arithmetic::{floor_root, inverse, is_prime, random_below, residue, RandomSource};
use crate::curve::{Curve, Point};
use crate::reduction::{FactorBase, HashSeed};
use crate::{integrity, invalid, overflow, BigInt, BigUint, Meter, Result};
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};

/// Exact 2D Lagrange–Gauss reduced lattice short box for a general Bochum
/// affine hash slot `0 <= H(2x + e) < tau` (`REPORT.md` §1).
///
/// Every integer pair `(t, z)` satisfying `z == a * t + d (mod sigma)` with
/// `0 <= t < tau` and `0 <= z < 2p` lies inside the non-negative box
/// `(u_1, u_2) in [0, width_1) x [0, width_2)` of area `O(tau)`, via
/// `t = origin_t + u_1 * basis_1.0 + u_2 * basis_2.0` and
/// `z = origin_z + u_1 * basis_1.1 + u_2 * basis_2.1`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GaussShortBox {
    /// Curve field prime `p`.
    pub p: BigUint,
    /// Certified hash prime `sigma in [2p, 4p]`.
    pub sigma: BigUint,
    /// Hash threshold `tau in [1, sigma]`.
    pub tau: BigUint,
    /// Inverse slope `a = alpha^{-1} mod sigma`.
    pub inv_slope: BigUint,
    /// Inverse intercept `d = (-a * beta) mod sigma`.
    pub inv_offset: BigUint,
    /// Shortest reduced basis vector `v_1 = (t_1, z_1)` of `Z*(1, a) + Z*(0, sigma)`.
    pub basis_1: (BigInt, BigInt),
    /// Second reduced basis vector `v_2 = (t_2, z_2)` with `t_1*z_2 - t_2*z_1 = +sigma`.
    pub basis_2: (BigInt, BigInt),
    /// Shifted box origin `t_0` at `(u_1, u_2) = (0, 0)`.
    pub origin_t: BigInt,
    /// Shifted box origin `z_0` at `(u_1, u_2) = (0, 0)`.
    pub origin_z: BigInt,
    /// Exclusive bound `W_1` on `u_1 in [0, W_1)`.
    pub width_1: BigUint,
    /// Exclusive bound `W_2` on `u_2 in [0, W_2)`.
    pub width_2: BigUint,
    /// Modular coefficient `(2^{-1} * z_1) mod p` for `u_1`.
    pub alpha_1_mod_p: BigUint,
    /// Modular coefficient `(2^{-1} * z_2) mod p` for `u_2`.
    pub alpha_2_mod_p: BigUint,
    /// Modular constant `(2^{-1} * z_0) mod p` for even-ordinate branch `e = 0`.
    pub beta_even_mod_p: BigUint,
    /// Modular constant `(2^{-1} * (z_0 - 1)) mod p` for odd-ordinate branch `e = 1`.
    pub beta_odd_mod_p: BigUint,
}

fn ceil_div(num: &BigInt, den: &BigInt) -> BigInt {
    (num + den - BigInt::one()).div_floor(den)
}

impl GaussShortBox {
    /// Construct and certify the 2D Gauss-reduced short box for `base` on `curve`.
    pub fn from_factor_base(curve: &Curve, base: &FactorBase, meter: &mut Meter) -> Result<Self> {
        let p = curve.modulus().clone();
        let sigma = base.prime().clone();
        let tau = base.threshold().clone();
        if sigma < (&p << 1) || sigma > (&p << 2) || tau.is_zero() || tau > sigma {
            return Err(invalid("GaussShortBox factor-base bounds"));
        }
        meter.bits(
            usize::try_from(sigma.bits().saturating_mul(4)).map_err(|_| overflow("box bits"))?,
            "Gauss short-box reduction",
        )?;

        let inv_slope = inverse(&base.seed().alpha, &sigma)?;
        let inv_offset = (&inv_slope * (&sigma - &base.seed().beta)) % &sigma;

        let sigma_i = BigInt::from(sigma.clone());
        let tau_i = BigInt::from(tau.clone());
        let sigma_sq = &sigma_i * &sigma_i;
        let tau_sq = &tau_i * &tau_i;

        let dot_metric = |u: &(BigInt, BigInt), v: &(BigInt, BigInt)| -> BigInt {
            &sigma_sq * &u.0 * &v.0 + &tau_sq * &u.1 * &v.1
        };

        let mut v1 = (BigInt::one(), BigInt::from(inv_slope.clone()));
        let mut v2 = (BigInt::zero(), sigma_i.clone());

        loop {
            meter.charge(1, "Gauss 2D lattice reduction step")?;
            let mut n1 = dot_metric(&v1, &v1);
            let mut n2 = dot_metric(&v2, &v2);
            if n1 > n2 {
                std::mem::swap(&mut v1, &mut v2);
                std::mem::swap(&mut n1, &mut n2);
            }
            let dot = dot_metric(&v1, &v2);
            let two_n1 = &n1 << 1usize;
            let mu = ((&dot << 1usize) + &n1).div_floor(&two_n1);
            if mu.is_zero() {
                break;
            }
            let cand = (&v2.0 - &mu * &v1.0, &v2.1 - &mu * &v1.1);
            let n_cand = dot_metric(&cand, &cand);
            if n_cand >= n2 {
                break;
            }
            v2 = cand;
        }

        if dot_metric(&v1, &v1) > dot_metric(&v2, &v2) {
            std::mem::swap(&mut v1, &mut v2);
        }
        let det = &v1.0 * &v2.1 - &v1.1 * &v2.0;
        if det == -&sigma_i {
            v2 = (-v2.0, -v2.1);
        } else if det != sigma_i {
            return Err(integrity("Gauss basis lost lattice determinant"));
        }

        let n1 = dot_metric(&v1, &v1);
        let n2 = dot_metric(&v2, &v2);
        let dot = dot_metric(&v1, &v2);
        if (&dot.abs() << 1usize) > n1 || (&n1 * &n2 - &dot * &dot) * 4i32 < (&n1 * &n2) * 3i32 {
            return Err(integrity("Gauss basis failed sin(theta) >= sqrt(3)/2"));
        }

        let d_i = BigInt::from(inv_offset.clone());
        let tau_hi = &tau_i - BigInt::one();
        let z_hi = (BigInt::from(p.clone()) << 1usize) - BigInt::one();

        let corners = [
            (BigInt::zero(), BigInt::zero()),
            (BigInt::zero(), z_hi.clone()),
            (tau_hi.clone(), BigInt::zero()),
            (tau_hi, z_hi),
        ];

        let mut min_num1: Option<BigInt> = None;
        let mut max_num1: Option<BigInt> = None;
        let mut min_num2: Option<BigInt> = None;
        let mut max_num2: Option<BigInt> = None;

        for (tc, zc) in &corners {
            let dz = zc - &d_i;
            let num1 = &v2.1 * tc - &v2.0 * &dz;
            let num2 = -&v1.1 * tc + &v1.0 * &dz;
            min_num1 = Some(match min_num1 {
                Some(cur) => cur.min(num1.clone()),
                None => num1.clone(),
            });
            max_num1 = Some(match max_num1 {
                Some(cur) => cur.max(num1),
                None => num1,
            });
            min_num2 = Some(match min_num2 {
                Some(cur) => cur.min(num2.clone()),
                None => num2.clone(),
            });
            max_num2 = Some(match max_num2 {
                Some(cur) => cur.max(num2),
                None => num2,
            });
        }

        let u1_min = ceil_div(&min_num1.unwrap(), &sigma_i);
        let u1_max = max_num1.unwrap().div_floor(&sigma_i);
        let u2_min = ceil_div(&min_num2.unwrap(), &sigma_i);
        let u2_max = max_num2.unwrap().div_floor(&sigma_i);

        let (width_1, width_2) = if u1_min <= u1_max && u2_min <= u2_max {
            (
                (&u1_max - &u1_min + BigInt::one()).to_biguint().unwrap(),
                (&u2_max - &u2_min + BigInt::one()).to_biguint().unwrap(),
            )
        } else {
            (BigUint::zero(), BigUint::zero())
        };

        let origin_t = &u1_min * &v1.0 + &u2_min * &v2.0;
        let origin_z = &d_i + &u1_min * &v1.1 + &u2_min * &v2.1;

        let inv2 = BigInt::from((&p + BigUint::one()) >> 1usize);
        let alpha_1_mod_p = residue(&(&inv2 * &v1.1), &p);
        let alpha_2_mod_p = residue(&(&inv2 * &v2.1), &p);
        let beta_even_mod_p = residue(&(&inv2 * &origin_z), &p);
        let beta_odd_mod_p = residue(&(&inv2 * (&origin_z - BigInt::one())), &p);

        Ok(Self {
            p,
            sigma,
            tau,
            inv_slope,
            inv_offset,
            basis_1: v1,
            basis_2: v2,
            origin_t,
            origin_z,
            width_1,
            width_2,
            alpha_1_mod_p,
            alpha_2_mod_p,
            beta_even_mod_p,
            beta_odd_mod_p,
        })
    }

    /// Number of integer pairs `width_1 * width_2` in the enclosing reduced box (`O(tau)`).
    pub fn box_area(&self) -> BigUint {
        &self.width_1 * &self.width_2
    }

    /// Evaluate the per-parity modular affine abscissa formula `x(u_1, u_2) mod p`.
    pub fn abscissa_mod_p(&self, u1: &BigUint, u2: &BigUint, parity: u8) -> BigUint {
        let beta = if parity == 0 {
            &self.beta_even_mod_p
        } else {
            &self.beta_odd_mod_p
        };
        (&self.alpha_1_mod_p * (u1 % &self.p) + &self.alpha_2_mod_p * (u2 % &self.p) + beta)
            % &self.p
    }

    /// Enumerate all raw accepted encodings `z in [0, 2p)` from the 2D short box,
    /// checking exact range, parity, and affine modular coordinate consistency.
    pub fn raw_encodings(&self, meter: &mut Meter) -> Result<Vec<BigUint>> {
        let w1 = self
            .width_1
            .to_usize()
            .ok_or_else(|| overflow("GaussShortBox width_1"))?;
        let w2 = self
            .width_2
            .to_usize()
            .ok_or_else(|| overflow("GaussShortBox width_2"))?;
        let area = w1
            .checked_mul(w2)
            .ok_or_else(|| overflow("GaussShortBox area"))?;
        meter.vector(area, "Gauss short-box scan")?;

        let tau_i = BigInt::from(self.tau.clone());
        let two_p_i = BigInt::from(&self.p << 1usize);
        let zero = BigInt::zero();
        let mut out = Vec::new();

        for i1 in 0..w1 {
            let u1 = BigInt::from(i1);
            let base_t = &self.origin_t + &u1 * &self.basis_1.0;
            let base_z = &self.origin_z + &u1 * &self.basis_1.1;
            for i2 in 0..w2 {
                meter.charge(1, "Gauss short-box point")?;
                let u2 = BigInt::from(i2);
                let t = &base_t + &u2 * &self.basis_2.0;
                if t < zero || t >= tau_i {
                    continue;
                }
                let z = &base_z + &u2 * &self.basis_2.1;
                if z < zero || z >= two_p_i {
                    continue;
                }
                let z_u = z.to_biguint().unwrap();
                let parity = u8::from(z_u.bit(0));
                let expected_x = &z_u >> 1usize;
                let formula_x = self.abscissa_mod_p(&BigUint::from(i1), &BigUint::from(i2), parity);
                if formula_x != expected_x {
                    return Err(integrity("Gauss short-box affine x formula mismatch"));
                }
                out.push(z_u);
            }
        }
        out.sort();
        if out.windows(2).any(|w| w[0] == w[1]) {
            return Err(integrity("duplicate encoding in Gauss short box"));
        }
        Ok(out)
    }

    /// Decode all certified curve subgroup members from the 2D Gauss short box.
    pub fn enumerate_points(
        &self,
        curve: &Curve,
        rng: &mut impl RandomSource,
        meter: &mut Meter,
    ) -> Result<Vec<Point>> {
        let encodings = self.raw_encodings(meter)?;
        let mut decoder = curve.decoder()?;
        let mut points = Vec::new();
        for z in encodings {
            if let Some(point) = decoder.decode(&z, rng, meter)? {
                if curve.in_subgroup(&point, meter)? {
                    points.push(point);
                }
            }
        }
        points.sort();
        Ok(points)
    }
}

/// One disjoint 1D short integer interval `u in [0, len)` produced by fixing
/// wrap `h in {0, 1}` and hash parity `delta in {0, 1}` in the prime-slope family
/// (`FOLLOWUP.md` §1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShortInterval1D {
    /// Modular wrap bit `h in {0, 1}` in `z = a * t + d - sigma * h`.
    pub wrap_h: u8,
    /// Hash output parity `delta = t mod 2 in {0, 1}` (`t = 2 * (u_min + u) + delta`).
    pub t_parity: u8,
    /// Constant point ordinate parity `e = z mod 2 in {0, 1}` across the entire interval.
    pub y_parity: u8,
    /// Number of integer points `u in [0, len)` (`len <= ceil(tau / 2)`).
    pub len: BigUint,
    /// Initial hash value `t(0)` at `u = 0` (`t(u) = t_start + 2 * u`).
    pub t_start: BigUint,
    /// Exact integer initial abscissa `x(0) in [0, p)` (`x(u) = x_start + x_step * u < p`).
    pub x_start: BigUint,
    /// Exact integer abscissa step `x_step = a` (`x(u) = x_start + a * u` without wrap).
    pub x_step: BigUint,
    /// Modular slope `alpha = a mod p` in `x(u) == alpha * u + beta (mod p)`.
    pub alpha_mod_p: BigUint,
    /// Modular offset `beta = x_start mod p` in `x(u) == alpha * u + beta (mod p)`.
    pub beta_mod_p: BigUint,
}

impl ShortInterval1D {
    /// Evaluate `x(u) mod p = (alpha * u + beta) mod p` for `u in [0, len)`.
    pub fn abscissa_mod_p(&self, u: &BigUint, p: &BigUint) -> BigUint {
        (&self.alpha_mod_p * (u % p) + &self.beta_mod_p) % p
    }

    /// Evaluate the exact compressed point encoding `z(u) = 2 * (x_start + x_step * u) + y_parity`.
    pub fn encoding_at(&self, u: &BigUint) -> BigUint {
        ((&self.x_start + &self.x_step * u) << 1usize) + BigUint::from(self.y_parity)
    }
}

/// Prime-inverse-slope hashed factor base `a in [ceil(A/2), A]` (`A = floor(sigma / tau)`)
/// whose accepted encodings split into `<= 4` disjoint 1D short intervals (`FOLLOWUP.md` §1).
#[derive(Clone, Debug)]
pub struct PrimeSlopeBase {
    base: FactorBase,
    inv_slope_a: BigUint,
    offset_d: BigUint,
    slope_cap_a: BigUint,
}

impl PrimeSlopeBase {
    /// Certify a prime-inverse-slope slot with prime `inv_slope_a in [ceil(A/2), A]`,
    /// `A = floor(sigma / tau)`, and `offset_d in [0, sigma)`.
    pub fn new(
        curve: &Curve,
        sigma: BigUint,
        tau: BigUint,
        inv_slope_a: BigUint,
        offset_d: BigUint,
        meter: &mut Meter,
    ) -> Result<Self> {
        if tau.is_zero() || tau > sigma || offset_d >= sigma {
            return Err(invalid("PrimeSlopeBase range parameters"));
        }
        let slope_cap_a = &sigma / &tau;
        let min_a = slope_cap_a.div_ceil(&BigUint::from(2u8));
        if inv_slope_a < min_a
            || inv_slope_a > slope_cap_a
            || inv_slope_a >= sigma
            || !is_prime(&inv_slope_a, meter)?
        {
            return Err(invalid(
                "PrimeSlopeBase requires prime inverse slope a in [ceil(A/2), A] with a < sigma",
            ));
        }
        let alpha = inverse(&inv_slope_a, &sigma)?;
        let beta = (&alpha * (&sigma - &offset_d)) % &sigma;
        let base = FactorBase::new(curve, sigma, tau, HashSeed { alpha, beta }, meter)?;
        Ok(Self {
            base,
            inv_slope_a,
            offset_d,
            slope_cap_a,
        })
    }

    /// Sample a prime inverse slope `a` uniformly from primes in `[ceil(A/2), A]`
    /// and an independent uniform offset `d in [0, sigma)`.
    pub fn sample(
        curve: &Curve,
        sigma: &BigUint,
        tau: &BigUint,
        rng: &mut impl RandomSource,
        meter: &mut Meter,
    ) -> Result<Self> {
        if tau.is_zero() || tau > sigma {
            return Err(invalid("PrimeSlopeBase threshold"));
        }
        let slope_cap_a = (sigma / tau).min(sigma - BigUint::one());
        let min_a = (sigma / tau)
            .div_ceil(&BigUint::from(2u8))
            .max(BigUint::from(2u8));
        if min_a > slope_cap_a {
            return Err(invalid("threshold too large for prime inverse slope range"));
        }
        let span = &slope_cap_a - &min_a + BigUint::one();
        let inv_slope_a = loop {
            let candidate = &min_a + random_below(&span, rng, meter)?;
            if is_prime(&candidate, meter)? {
                break candidate;
            }
        };
        let offset_d = random_below(sigma, rng, meter)?;
        Self::new(
            curve,
            sigma.clone(),
            tau.clone(),
            inv_slope_a,
            offset_d,
            meter,
        )
    }

    /// Underlying certified Bochum `FactorBase` (`H(z) = (alpha * z + beta) mod sigma < tau`).
    pub fn factor_base(&self) -> &FactorBase {
        &self.base
    }

    /// Certified prime inverse slope `a`.
    pub fn inv_slope(&self) -> &BigUint {
        &self.inv_slope_a
    }

    /// Inverse offset `d in [0, sigma)`.
    pub fn inv_offset(&self) -> &BigUint {
        &self.offset_d
    }

    /// Upper bound `A = floor(sigma / tau)`.
    pub fn slope_cap(&self) -> &BigUint {
        &self.slope_cap_a
    }

    /// Decompose all accepted encodings `z in [0, 2p)` with `0 <= H(z) < tau`
    /// into at most `4` disjoint 1D short integer intervals `u in [0, O(tau))`.
    pub fn intervals(&self, curve: &Curve) -> Result<Vec<ShortInterval1D>> {
        let p = curve.modulus();
        if self.base.prime() < &(p << 1usize) || self.base.prime() > &(p << 2usize) {
            return Err(invalid("PrimeSlopeBase curve modulus mismatch"));
        }
        let a_i = BigInt::from(self.inv_slope_a.clone());
        let two_a = &a_i << 1usize;
        let d_i = BigInt::from(self.offset_d.clone());
        let sigma_i = BigInt::from(self.base.prime().clone());
        let tau_minus_one = BigInt::from(self.base.threshold().clone()) - BigInt::one();
        let two_p_minus_one = (BigInt::from(p.clone()) << 1usize) - BigInt::one();
        let two = BigInt::from(2u8);

        let mut branches = Vec::with_capacity(4);
        for wrap_h in [0u8, 1u8] {
            for t_parity in [0u8, 1u8] {
                let delta_i = BigInt::from(t_parity);
                if tau_minus_one < delta_i {
                    continue;
                }
                let u_upper_t = (&tau_minus_one - &delta_i).div_floor(&two);
                let c = &a_i * &delta_i + &d_i - &sigma_i * BigInt::from(wrap_h);
                let u_lower_z = ceil_div(&(-&c), &two_a);
                let u_upper_z = (&two_p_minus_one - &c).div_floor(&two_a);

                let u_min = BigInt::zero().max(u_lower_z);
                let u_max = u_upper_t.min(u_upper_z);
                if u_min > u_max {
                    continue;
                }

                let len = (&u_max - &u_min + BigInt::one()).to_biguint().unwrap();
                let y_parity = c
                    .mod_floor(&two)
                    .to_u8()
                    .ok_or_else(|| integrity("parity residue"))?;
                let c_even = &c - BigInt::from(y_parity);
                let x_start_i = &a_i * &u_min + c_even.div_floor(&two);
                if x_start_i.is_negative() {
                    return Err(integrity("negative x_start in PrimeSlopeBase"));
                }
                let x_start = x_start_i.to_biguint().unwrap();
                let t_start = ((&u_min << 1usize) + &delta_i).to_biguint().unwrap();

                branches.push(ShortInterval1D {
                    wrap_h,
                    t_parity,
                    y_parity,
                    len,
                    t_start,
                    alpha_mod_p: &self.inv_slope_a % p,
                    beta_mod_p: &x_start % p,
                    x_start,
                    x_step: self.inv_slope_a.clone(),
                });
            }
        }
        if branches.len() > 4 {
            return Err(integrity("PrimeSlopeBase emitted more than 4 intervals"));
        }
        Ok(branches)
    }

    /// Enumerate all raw accepted encodings `z in [0, 2p)` from the `<= 4` 1D intervals,
    /// asserting pairwise disjointness and exact affine modular `x` consistency.
    pub fn raw_encodings(&self, curve: &Curve, meter: &mut Meter) -> Result<Vec<BigUint>> {
        let p = curve.modulus();
        let branches = self.intervals(curve)?;
        let mut out = Vec::new();
        for branch in &branches {
            let count = branch
                .len
                .to_usize()
                .ok_or_else(|| overflow("1D interval length"))?;
            meter.vector(out.len().saturating_add(count), "1D short-box encodings")?;
            for idx in 0..count {
                meter.charge(1, "1D short-box point")?;
                let u = BigUint::from(idx);
                let x_exact = &branch.x_start + &branch.x_step * &u;
                if &x_exact >= p {
                    return Err(integrity("1D short-box abscissa exceeded p"));
                }
                if branch.abscissa_mod_p(&u, p) != x_exact {
                    return Err(integrity("1D short-box modular affine formula mismatch"));
                }
                out.push(branch.encoding_at(&u));
            }
        }
        let raw_len = out.len();
        out.sort();
        out.dedup();
        if out.len() != raw_len {
            return Err(integrity("1D short-box intervals were not disjoint"));
        }
        Ok(out)
    }

    /// Decode all certified subgroup points via the `<= 4` 1D short intervals.
    pub fn enumerate_points(
        &self,
        curve: &Curve,
        rng: &mut impl RandomSource,
        meter: &mut Meter,
    ) -> Result<Vec<Point>> {
        let encodings = self.raw_encodings(curve, meter)?;
        let mut decoder = curve.decoder()?;
        let mut points = Vec::new();
        for z in encodings {
            if let Some(point) = decoder.decode(&z, rng, meter)? {
                if curve.in_subgroup(&point, meter)? {
                    points.push(point);
                }
            }
        }
        points.sort();
        Ok(points)
    }
}

/// Construct the Cartesian product of non-empty 1D short-box intervals across `m`
/// prime-slope slots, certifying at most `4^m` disjoint rectangular `m`-boxes.
pub fn product_short_boxes(
    curve: &Curve,
    slots: &[PrimeSlopeBase],
    meter: &mut Meter,
) -> Result<Vec<Vec<ShortInterval1D>>> {
    if slots.is_empty() {
        return Err(invalid("empty slot slice"));
    }
    let mut boxes: Vec<Vec<ShortInterval1D>> = vec![Vec::new()];
    for slot in slots {
        let branches = slot.intervals(curve)?;
        let next_len = boxes
            .len()
            .checked_mul(branches.len())
            .ok_or_else(|| overflow("product box count"))?;
        meter.vector(next_len, "Cartesian short-box branches")?;
        let mut next = Vec::with_capacity(next_len);
        for prefix in &boxes {
            for branch in &branches {
                meter.charge(1, "Cartesian short-box branch")?;
                let mut tuple = prefix.clone();
                tuple.push(branch.clone());
                next.push(tuple);
            }
        }
        boxes = next;
    }
    let cap = 4usize.checked_pow(slots.len() as u32).unwrap_or(usize::MAX);
    if boxes.len() > cap {
        return Err(integrity("product short boxes exceeded 4^m"));
    }
    Ok(boxes)
}

/// Compute the integer threshold `tau in [1, sigma]` targeting expected slot tuple
/// mass `L^m ≈ (theta_num / theta_den) * N` where `L = (N - 1) * tau / sigma`
/// (`EXACT.md`, "A smaller parity obligation for relation sampling").
pub fn threshold_for_theta(
    group_order: &BigUint,
    sigma: &BigUint,
    slots_m: usize,
    theta_num: u64,
    theta_den: u64,
    meter: &mut Meter,
) -> Result<BigUint> {
    if slots_m < 2
        || theta_num == 0
        || theta_den == 0
        || group_order <= &BigUint::from(2u8)
        || sigma.is_zero()
    {
        return Err(invalid("threshold_for_theta parameters"));
    }
    let m_u32 = u32::try_from(slots_m).map_err(|_| overflow("slot exponent"))?;
    let scaled = (BigUint::from(theta_num) * group_order * sigma.pow(m_u32)
        + BigUint::from(theta_den / 2))
        / BigUint::from(theta_den);
    let root = floor_root(&scaled, slots_m, meter)?;
    let n_minus_one = group_order - BigUint::one();
    let half = &n_minus_one >> 1usize;
    let tau = ((root + half) / n_minus_one)
        .max(BigUint::one())
        .min(sigma.clone());
    Ok(tau)
}

/// Hash-seed family used for empirical coverage and unique-witness measurement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashFamily {
    /// Unrestricted Bochum affine hashes `alpha in [1, sigma), beta in [0, sigma)`.
    GeneralBochum,
    /// Prime-inverse-slope family `a in [ceil(A/2), A]` (`FOLLOWUP.md` §1).
    PrimeSlope,
}

/// Empirical witness-multiplicity statistics for one `(curve, m, theta, family)` cell.
#[derive(Clone, Debug)]
pub struct CoverageMeasurement {
    /// Arity `m`.
    pub slots: usize,
    /// Target `theta` (`theta_num / theta_den`).
    pub theta_target: f64,
    /// Realized expected tuple density `((N - 1) * tau / sigma)^m / N`.
    pub theta_realized: f64,
    /// Integer threshold `tau` used across slots.
    pub tau: u64,
    /// Certified hash prime `sigma in [2p, 4p]`.
    pub sigma: u64,
    /// Prime slope cap `A = floor(sigma / tau)`.
    pub slope_cap_a: u64,
    /// Number of independent seed trials averaged.
    pub seed_trials: usize,
    /// Whether every group target `R in E(F_p)` was counted (`true`) or uniformly sampled (`false`).
    pub exact_full_group: bool,
    /// Number of targets evaluated per seed (`N` when `exact_full_group`, else sample count).
    pub targets_per_seed: u64,
    /// Mean factor-base size `|F_i|` across slots and seeds.
    pub mean_base_size: f64,
    /// Mean number of nonempty `m`-dimensional short boxes (`<= 4^m` for `PrimeSlope`).
    pub mean_branch_boxes: f64,
    /// Maximum nonempty `m`-dimensional short boxes observed across seeds.
    pub max_branch_boxes: usize,
    /// Empirical first moment `E[X]`.
    pub mean_witnesses: f64,
    /// Empirical factorial second moment `E[X * (X - 1)]`.
    pub mean_ordered_pairs: f64,
    /// Empirical coverage `Pr(X > 0)`.
    pub coverage_prob: f64,
    /// Empirical unique-witness probability `Pr(X = 1)` (detected by one mod-2 parity query).
    pub unique_witness_prob: f64,
    /// Empirical bounded-multiplicity probability `Pr(1 <= X <= 32)` (5-bit pin sweep reach).
    pub up_to_32_prob: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FastPoint {
    infinity: bool,
    x: u64,
    y: u64,
}

struct FastCurve64 {
    p: u64,
    a: u64,
    b: u64,
    r: u64,
    gen: FastPoint,
}

impl FastCurve64 {
    fn from_certified(curve: &Curve) -> Result<Self> {
        if !curve.cofactor().is_one() {
            return Err(invalid("FastCurve64 requires prime-order curve"));
        }
        let p = curve
            .modulus()
            .to_u64()
            .ok_or_else(|| overflow("curve modulus > u64"))?;
        if p >= (1u64 << 62) {
            return Err(invalid("FastCurve64 modulus >= 2^62"));
        }
        let a = curve.a().to_u64().unwrap();
        let b = curve.b().to_u64().unwrap();
        let r = curve.order().to_u64().unwrap();
        let gen = match curve.generator() {
            Point::Infinity => return Err(invalid("generator is infinity")),
            Point::Affine { x, y } => FastPoint {
                infinity: false,
                x: x.to_u64().unwrap(),
                y: y.to_u64().unwrap(),
            },
        };
        let out = Self { p, a, b, r, gen };
        if !out.on_curve(gen) {
            return Err(integrity("FastCurve64 generator failed curve equation"));
        }
        Ok(out)
    }

    fn on_curve(&self, pt: FastPoint) -> bool {
        if pt.infinity {
            return true;
        }
        let lhs = self.mul_mod(pt.y, pt.y);
        let x2 = self.mul_mod(pt.x, pt.x);
        let x3 = self.mul_mod(x2, pt.x);
        let rhs = self.add_mod(self.add_mod(x3, self.mul_mod(self.a, pt.x)), self.b);
        lhs == rhs
    }

    #[inline]
    fn add_mod(&self, u: u64, v: u64) -> u64 {
        let s = u + v;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }

    #[inline]
    fn sub_mod(&self, u: u64, v: u64) -> u64 {
        if u >= v {
            u - v
        } else {
            self.p - (v - u)
        }
    }

    #[inline]
    fn mul_mod(&self, u: u64, v: u64) -> u64 {
        ((u128::from(u) * u128::from(v)) % u128::from(self.p)) as u64
    }

    fn inv_mod(&self, u: u64) -> u64 {
        let mut t = 0i128;
        let mut new_t = 1i128;
        let mut r = i128::from(self.p);
        let mut new_r = i128::from(u % self.p);
        while new_r != 0 {
            let q = r / new_r;
            (t, new_t) = (new_t, t - q * new_t);
            (r, new_r) = (new_r, r - q * new_r);
        }
        debug_assert_eq!(r, 1);
        if t < 0 {
            (t + i128::from(self.p)) as u64
        } else {
            t as u64
        }
    }

    fn add(&self, left: FastPoint, right: FastPoint) -> FastPoint {
        if left.infinity {
            return right;
        }
        if right.infinity {
            return left;
        }
        if left.x == right.x {
            if self.add_mod(left.y, right.y) == 0 {
                return FastPoint {
                    infinity: true,
                    x: 0,
                    y: 0,
                };
            }
            let num = self.add_mod(self.mul_mod(3, self.mul_mod(left.x, left.x)), self.a);
            let den = self.add_mod(left.y, left.y);
            let lam = self.mul_mod(num, self.inv_mod(den));
            let nx = self.sub_mod(self.sub_mod(self.mul_mod(lam, lam), left.x), right.x);
            let ny = self.sub_mod(self.mul_mod(lam, self.sub_mod(left.x, nx)), left.y);
            return FastPoint {
                infinity: false,
                x: nx,
                y: ny,
            };
        }
        let num = self.sub_mod(right.y, left.y);
        let den = self.sub_mod(right.x, left.x);
        let lam = self.mul_mod(num, self.inv_mod(den));
        let nx = self.sub_mod(self.sub_mod(self.mul_mod(lam, lam), left.x), right.x);
        let ny = self.sub_mod(self.mul_mod(lam, self.sub_mod(left.x, nx)), left.y);
        FastPoint {
            infinity: false,
            x: nx,
            y: ny,
        }
    }

    fn negate(&self, pt: FastPoint) -> FastPoint {
        if pt.infinity || pt.y == 0 {
            pt
        } else {
            FastPoint {
                infinity: false,
                x: pt.x,
                y: self.p - pt.y,
            }
        }
    }

    fn scalar_mul(&self, mut pt: FastPoint, mut k: u64) -> FastPoint {
        let mut acc = FastPoint {
            infinity: true,
            x: 0,
            y: 0,
        };
        while k != 0 {
            if k & 1 != 0 {
                acc = self.add(acc, pt);
            }
            k >>= 1;
            if k != 0 {
                pt = self.add(pt, pt);
            }
        }
        acc
    }

    /// Add one fixed point `left` to an entire slice `rights` using Montgomery batch inversion.
    fn batch_add_one_to_many(&self, left: FastPoint, rights: &[FastPoint], out: &mut Vec<u64>) {
        if left.infinity {
            for &r in rights {
                out.push(pack_point(r));
            }
            return;
        }
        let n = rights.len();
        let mut dens = Vec::with_capacity(n);
        let mut prefix = Vec::with_capacity(n);
        let mut prod = 1u64;
        for &r in rights {
            let d = if r.infinity || r.x == left.x {
                1u64
            } else {
                self.sub_mod(r.x, left.x)
            };
            dens.push(d);
            prefix.push(prod);
            prod = self.mul_mod(prod, d);
        }
        let mut inv_prod = self.inv_mod(prod);
        let mut invs = vec![0u64; n];
        for i in (0..n).rev() {
            invs[i] = self.mul_mod(inv_prod, prefix[i]);
            inv_prod = self.mul_mod(inv_prod, dens[i]);
        }
        for (i, &r) in rights.iter().enumerate() {
            let sum = if r.infinity || r.x == left.x {
                self.add(left, r)
            } else {
                let lam = self.mul_mod(self.sub_mod(r.y, left.y), invs[i]);
                let nx = self.sub_mod(self.sub_mod(self.mul_mod(lam, lam), left.x), r.x);
                let ny = self.sub_mod(self.mul_mod(lam, self.sub_mod(left.x, nx)), left.y);
                FastPoint {
                    infinity: false,
                    x: nx,
                    y: ny,
                }
            };
            out.push(pack_point(sum));
        }
    }
}

#[inline]
fn pack_point(pt: FastPoint) -> u64 {
    if pt.infinity {
        u64::MAX
    } else {
        (pt.x << 1) | (pt.y & 1)
    }
}

fn to_fast(pt: &Point) -> FastPoint {
    match pt {
        Point::Infinity => FastPoint {
            infinity: true,
            x: 0,
            y: 0,
        },
        Point::Affine { x, y } => FastPoint {
            infinity: false,
            x: x.to_u64().unwrap(),
            y: y.to_u64().unwrap(),
        },
    }
}

/// Measure empirical coverage `Pr(X > 0)` and unique-witness rate `Pr(X = 1)`
/// at `L^m = theta * N` across `seed_trials` independent seeds for `curve`.
///
/// Uses an exact full-group census over all `N` targets in `E(F_p)` whenever
/// `L^m <= exact_tuple_cap`, and uniform subgroup target sampling (`sample_targets`
/// random multiples `[k]G` per seed) above that cap.
pub fn measure_coverage(
    curve: &Curve,
    sigma: &BigUint,
    slots_m: usize,
    theta_num: u64,
    theta_den: u64,
    family: HashFamily,
    seed_trials: usize,
    exact_tuple_cap: usize,
    sample_targets: usize,
    rng: &mut impl RandomSource,
    meter: &mut Meter,
) -> Result<CoverageMeasurement> {
    if !(2..=3).contains(&slots_m) || seed_trials == 0 || sample_targets == 0 {
        return Err(invalid(
            "measure_coverage supports m in {2, 3} and positive trials",
        ));
    }
    let fast = FastCurve64::from_certified(curve)?;
    let tau_big = threshold_for_theta(
        curve.order(),
        sigma,
        slots_m,
        theta_num,
        theta_den,
        meter,
    )?;
    let tau = tau_big
        .to_u64()
        .ok_or_else(|| overflow("tau exceeds u64"))?;
    let sigma_u64 = sigma
        .to_u64()
        .ok_or_else(|| overflow("sigma exceeds u64"))?;
    let slope_cap_a = sigma_u64 / tau;

    let l_expected = ((fast.r - 1) as f64) * (tau as f64) / (sigma_u64 as f64);
    let theta_realized = l_expected.powi(slots_m as i32) / (fast.r as f64);
    let use_exact = (l_expected.powi(slots_m as i32) as usize) <= exact_tuple_cap;

    let mut sum_base_sizes = 0.0f64;
    let mut sum_branch_boxes = 0.0f64;
    let mut max_branch_boxes = 0usize;
    let mut sum_mean_x = 0.0f64;
    let mut sum_ordered_pairs = 0.0f64;
    let mut sum_cov = 0.0f64;
    let mut sum_unique = 0.0f64;
    let mut sum_le_32 = 0.0f64;

    for trial in 0..seed_trials {
        let mut bases_points: Vec<Vec<FastPoint>> = Vec::with_capacity(slots_m);
        let mut bases_raw: Vec<FactorBase> = Vec::with_capacity(slots_m);
        let mut prime_slots: Vec<PrimeSlopeBase> = Vec::with_capacity(slots_m);

        for _ in 0..slots_m {
            match family {
                HashFamily::PrimeSlope => {
                    let ps = PrimeSlopeBase::sample(curve, sigma, &tau_big, rng, meter)?;
                    let pts = ps.enumerate_points(curve, rng, meter)?;
                    bases_points.push(pts.iter().map(to_fast).collect());
                    bases_raw.push(ps.factor_base().clone());
                    prime_slots.push(ps);
                }
                HashFamily::GeneralBochum => {
                    let alpha = BigUint::one()
                        + random_below(&(sigma - BigUint::one()), rng, meter)?;
                    let beta = random_below(sigma, rng, meter)?;
                    let fb = FactorBase::new(
                        curve,
                        sigma.clone(),
                        tau_big.clone(),
                        HashSeed { alpha, beta },
                        meter,
                    )?;
                    let pts = fb.enumerate(curve, rng, meter)?;
                    bases_points.push(pts.iter().map(to_fast).collect());
                    bases_raw.push(fb);
                }
            }
        }

        // Spot-check FastCurve64 addition against certified Curve::add on the first seed.
        if trial == 0 && !bases_points[0].is_empty() && !bases_points[1].is_empty() {
            let p0 = bases_points[0][0];
            let p1 = bases_points[1][0];
            let fast_sum = fast.add(p0, p1);
            let cert_sum = curve.add(
                &Point::Affine {
                    x: BigUint::from(p0.x),
                    y: BigUint::from(p0.y),
                },
                &Point::Affine {
                    x: BigUint::from(p1.x),
                    y: BigUint::from(p1.y),
                },
                meter,
            )?;
            if pack_point(fast_sum) != pack_point(to_fast(&cert_sum)) {
                return Err(integrity("FastCurve64 group addition disagreed with Curve"));
            }
        }

        for pts in &bases_points {
            sum_base_sizes += pts.len() as f64;
        }
        let branch_boxes = if family == HashFamily::PrimeSlope {
            product_short_boxes(curve, &prime_slots, meter)?.len()
        } else {
            1usize
        };
        sum_branch_boxes += branch_boxes as f64;
        if branch_boxes > max_branch_boxes {
            max_branch_boxes = branch_boxes;
        }

        let tuple_count: usize = bases_points.iter().map(Vec::len).product();
        if use_exact && tuple_count <= exact_tuple_cap * 4 {
            let mut packed_sums = Vec::with_capacity(tuple_count);
            if slots_m == 2 {
                for &p1 in &bases_points[0] {
                    fast.batch_add_one_to_many(p1, &bases_points[1], &mut packed_sums);
                }
            } else {
                let mut pairs = Vec::with_capacity(bases_points[0].len() * bases_points[1].len());
                for &p1 in &bases_points[0] {
                    for &p2 in &bases_points[1] {
                        pairs.push(fast.add(p1, p2));
                    }
                }
                for &p3 in &bases_points[2] {
                    fast.batch_add_one_to_many(p3, &pairs, &mut packed_sums);
                }
            }
            packed_sums.sort_unstable();

            let mut distinct_positive = 0u64;
            let mut unique_hits = 0u64;
            let mut le_32_hits = 0u64;
            let mut ordered_pairs = 0u64;

            let mut idx = 0usize;
            while idx < packed_sums.len() {
                let val = packed_sums[idx];
                let mut end = idx + 1;
                while end < packed_sums.len() && packed_sums[end] == val {
                    end += 1;
                }
                let mult = (end - idx) as u64;
                distinct_positive += 1;
                if mult == 1 {
                    unique_hits += 1;
                }
                if mult <= 32 {
                    le_32_hits += 1;
                }
                ordered_pairs += mult * (mult - 1);
                idx = end;
            }

            let n_f64 = fast.r as f64;
            sum_mean_x += (packed_sums.len() as f64) / n_f64;
            sum_ordered_pairs += (ordered_pairs as f64) / n_f64;
            sum_cov += (distinct_positive as f64) / n_f64;
            sum_unique += (unique_hits as f64) / n_f64;
            sum_le_32 += (le_32_hits as f64) / n_f64;
        } else {
            // Sample `sample_targets` uniform subgroup elements R = [k]G.
            let mut targets = Vec::with_capacity(sample_targets);
            for _ in 0..sample_targets {
                let k = random_below(curve.order(), rng, meter)?.to_u64().unwrap();
                targets.push(fast.scalar_mul(fast.gen, k));
            }

            let mut counts = vec![0u64; sample_targets];
            if slots_m == 2 {
                // Check R - P_1 in F_2 via O(1) affine hash evaluation on the packed result!
                let neg_f1: Vec<FastPoint> =
                    bases_points[0].iter().map(|&p| fast.negate(p)).collect();
                let alpha2 = bases_raw[1].seed().alpha.to_u64().unwrap();
                let beta2 = bases_raw[1].seed().beta.to_u64().unwrap();
                let mut diffs = Vec::with_capacity(neg_f1.len());
                for (ti, &target) in targets.iter().enumerate() {
                    diffs.clear();
                    fast.batch_add_one_to_many(target, &neg_f1, &mut diffs);
                    let mut x_count = 0u64;
                    for &packed in &diffs {
                        if packed == u64::MAX {
                            continue;
                        }
                        let h = ((u128::from(alpha2) * u128::from(packed) + u128::from(beta2))
                            % u128::from(sigma_u64)) as u64;
                        if h < tau {
                            x_count += 1;
                        }
                    }
                    counts[ti] = x_count;
                }
            } else {
                // Precompute pair-sum multiplicity map for F_1 + F_2, then probe R - P_3!
                let mut pair_sums =
                    Vec::with_capacity(bases_points[0].len() * bases_points[1].len());
                for &p1 in &bases_points[0] {
                    fast.batch_add_one_to_many(p1, &bases_points[1], &mut pair_sums);
                }
                pair_sums.sort_unstable();
                let mut unique_pairs: Vec<(u64, u32)> = Vec::new();
                let mut idx = 0usize;
                while idx < pair_sums.len() {
                    let val = pair_sums[idx];
                    let mut end = idx + 1;
                    while end < pair_sums.len() && pair_sums[end] == val {
                        end += 1;
                    }
                    unique_pairs.push((val, (end - idx) as u32));
                    idx = end;
                }

                let neg_f3: Vec<FastPoint> =
                    bases_points[2].iter().map(|&p| fast.negate(p)).collect();
                let mut diffs = Vec::with_capacity(neg_f3.len());
                for (ti, &target) in targets.iter().enumerate() {
                    diffs.clear();
                    fast.batch_add_one_to_many(target, &neg_f3, &mut diffs);
                    let mut x_count = 0u64;
                    for &packed in &diffs {
                        if let Ok(pos) =
                            unique_pairs.binary_search_by_key(&packed, |&(k, _)| k)
                        {
                            x_count += u64::from(unique_pairs[pos].1);
                        }
                    }
                    counts[ti] = x_count;
                }
            }

            let t_f64 = sample_targets as f64;
            let total_x: u64 = counts.iter().sum();
            let ordered: u64 = counts.iter().map(|&c| c * c.saturating_sub(1)).sum();
            let pos = counts.iter().filter(|&&c| c > 0).count() as f64;
            let uniq = counts.iter().filter(|&&c| c == 1).count() as f64;
            let le32 = counts.iter().filter(|&&c| (1..=32).contains(&c)).count() as f64;

            sum_mean_x += (total_x as f64) / t_f64;
            sum_ordered_pairs += (ordered as f64) / t_f64;
            sum_cov += pos / t_f64;
            sum_unique += uniq / t_f64;
            sum_le_32 += le32 / t_f64;
        }
    }

    let trials_f64 = seed_trials as f64;
    Ok(CoverageMeasurement {
        slots: slots_m,
        theta_target: (theta_num as f64) / (theta_den as f64),
        theta_realized,
        tau,
        sigma: sigma_u64,
        slope_cap_a,
        seed_trials,
        exact_full_group: use_exact,
        targets_per_seed: if use_exact {
            fast.r
        } else {
            sample_targets as u64
        },
        mean_base_size: sum_base_sizes / ((seed_trials * slots_m) as f64),
        mean_branch_boxes: sum_branch_boxes / trials_f64,
        max_branch_boxes,
        mean_witnesses: sum_mean_x / trials_f64,
        mean_ordered_pairs: sum_ordered_pairs / trials_f64,
        coverage_prob: sum_cov / trials_f64,
        unique_witness_prob: sum_unique / trials_f64,
        up_to_32_prob: sum_le_32 / trials_f64,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arithmetic::Seeded;
    use crate::Limits;

    #[test]
    fn gauss_2d_short_box_and_prime_slope_1d_intervals_match_factor_base_exhaustively() {
        let mut meter = Meter::new(Limits {
            steps: 500_000_000,
            ..Limits::default()
        });
        let curve = Curve::new(
            BigUint::from(827u32),
            BigUint::from(1u32),
            BigUint::from(15u32),
            BigUint::from(823u32),
            Point::Affine {
                x: BigUint::from(2u32),
                y: BigUint::from(822u32),
            },
            &mut meter,
        )
        .unwrap();
        let sigma = BigUint::from(1657u32);
        let mut rng = Seeded(42);

        // 1. Test 2D Gauss short box across 32 general Bochum seeds and thresholds.
        for trial in 1..=32u64 {
            let tau = BigUint::from(1u64 + (trial * 19) % 250);
            let alpha = BigUint::from(1u64 + (trial * 73) % 1656);
            let beta = BigUint::from((trial * 131) % 1657);
            let fb = FactorBase::new(
                &curve,
                sigma.clone(),
                tau.clone(),
                HashSeed { alpha, beta },
                &mut meter,
            )
            .unwrap();
            let gbox = GaussShortBox::from_factor_base(&curve, &fb, &mut meter).unwrap();
            assert!(gbox.box_area() <= &tau * BigUint::from(6u8) + BigUint::from(4u8));
            let got_pts = gbox.enumerate_points(&curve, &mut rng, &mut meter).unwrap();
            let ref_pts = fb.enumerate(&curve, &mut rng, &mut meter).unwrap();
            assert_eq!(got_pts, ref_pts, "GaussShortBox mismatch at trial={trial}");
        }

        // 2. Test PrimeSlopeBase 1D short intervals across 32 sampled prime slopes.
        for tau_u64 in [12u64, 29, 45, 80, 200] {
            let tau = BigUint::from(tau_u64);
            for _ in 0..8 {
                let ps =
                    PrimeSlopeBase::sample(&curve, &sigma, &tau, &mut rng, &mut meter).unwrap();
                let intervals = ps.intervals(&curve).unwrap();
                assert!(intervals.len() <= 4);
                let half_tau = tau.div_ceil(&BigUint::from(2u8));
                for iv in &intervals {
                    assert!(iv.len <= half_tau);
                }
                let got_pts = ps.enumerate_points(&curve, &mut rng, &mut meter).unwrap();
                let ref_pts = ps
                    .factor_base()
                    .enumerate(&curve, &mut rng, &mut meter)
                    .unwrap();
                assert_eq!(got_pts, ref_pts);
            }
        }
    }
}
