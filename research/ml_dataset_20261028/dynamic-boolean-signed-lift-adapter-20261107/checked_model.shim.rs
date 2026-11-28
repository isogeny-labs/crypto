//! Independently admitted checked-map model and signed relation verification.
//! No native Solver, constructor witness, scalar logs or fresh budget.
use super::*;
#[derive(Debug, Default)]
pub struct Progress {
    pub formula_rows: usize,
    pub blocks: Vec<u64>,
    pub chain_steps: usize,
    pub lift_choices: Vec<[Option<(u64, u64)>; 2]>,
    pub path: Vec<(u64, u64)>,
    pub sum: Option<(u64, u64)>,
    pub sign_mask: u32,
    pub signed_relation: bool,
}
pub trait VerificationAdmission: Admission {
    fn addition(&mut self) -> Result<(), Error>;
}
fn charge(o: &mut impl Admission, w: u64, b: u64) -> Result<(), Error> {
    o.reserve(Quote { work: w, bytes: b })?;
    o.check()
}
fn grow<T>(v: &mut Vec<T>, n: usize, o: &mut impl Admission) -> Result<(), Error> {
    if n > v.capacity() {
        let bytes = n.checked_mul(std::mem::size_of::<T>()).ok_or(Error::Cap)?;
        charge(o, v.len() as u64, bytes as u64)?;
        v.try_reserve_exact(n - v.len()).map_err(|_| Error::Cap)?;
        o.check()?;
    }
    Ok(())
}
fn word(limbs: &[u32]) -> Result<u64, Error> {
    if limbs.len() > 2 {
        return Err(Error::Semantics);
    };
    Ok(limbs.first().copied().unwrap_or(0) as u64
        | ((limbs.get(1).copied().unwrap_or(0) as u64) << 32))
}
struct Field {
    n: u32,
    low: u64,
}
impl Field {
    fn mul(&self, mut a: u64, mut b: u64, o: &mut impl Admission) -> Result<u64, Error> {
        let mut r = 0;
        for _ in 0..self.n {
            charge(o, 12, 0)?;
            if b & 1 != 0 {
                r ^= a;
            }
            b >>= 1;
            let carry = a >> (self.n - 1);
            a = (a << 1) & ((1u64 << self.n) - 1);
            if carry != 0 {
                a ^= self.low;
            }
        }
        Ok(r)
    }
    fn inv(&self, a: u64, o: &mut impl Admission) -> Result<u64, Error> {
        if a == 0 {
            return Err(Error::Semantics);
        };
        let mut r = 1;
        let e = (1u64 << self.n) - 2;
        for i in (0..self.n).rev() {
            r = self.mul(r, r, o)?;
            if e >> i & 1 != 0 {
                r = self.mul(r, a, o)?;
            }
        }
        Ok(r)
    }
    fn on_curve(
        &self,
        p: (u64, u64),
        a: u64,
        b: u64,
        o: &mut impl Admission,
    ) -> Result<bool, Error> {
        let (x, y) = p;
        let xx = self.mul(x, x, o)?;
        Ok(
            self.mul(y, y, o)? ^ self.mul(x, y, o)?
                == self.mul(xx, x, o)? ^ self.mul(a, xx, o)? ^ b,
        )
    }
    fn add(
        &self,
        p: Option<(u64, u64)>,
        q: (u64, u64),
        a: u64,
        o: &mut impl VerificationAdmission,
    ) -> Result<Option<(u64, u64)>, Error> {
        o.addition()?;
        charge(o, 16, 0)?;
        let Some((x, y)) = p else {
            return Ok(Some(q));
        };
        let (u, v) = q;
        if x == u {
            if y != v || x == 0 {
                return Ok(None);
            }
            let inv = self.inv(x, o)?;
            let lambda = x ^ self.mul(y, inv, o)?;
            let z = self.mul(lambda, lambda, o)? ^ lambda ^ a;
            return Ok(Some((z, self.mul(x, x, o)? ^ self.mul(lambda ^ 1, z, o)?)));
        }
        let inv = self.inv(x ^ u, o)?;
        let lambda = self.mul(y ^ v, inv, o)?;
        let z = self.mul(lambda, lambda, o)? ^ lambda ^ x ^ u ^ a;
        Ok(Some((z, self.mul(lambda, x ^ z, o)? ^ z ^ y)))
    }
}
fn prefix_matches(sum: Option<(u64, u64)>, expected_x: Option<u64>) -> bool {
    sum.map(|point| point.0) == expected_x
}

/// Caller owns model and Progress before entry; failure retains every buffer.
/// Success is signed PDP existence for one pattern, never scalar recovery.
pub fn verify(
    map: &CheckedMap,
    model: &[bool],
    signed_domain: &[(u64, u64)],
    target: Option<(u64, u64)>,
    p: &mut Progress,
    o: &mut impl VerificationAdmission,
) -> Result<(), Error> {
    charge(o, 64, 0)?;
    if p.formula_rows != 0
        || !p.blocks.is_empty()
        || p.chain_steps != 0
        || !p.lift_choices.is_empty()
        || !p.path.is_empty()
        || p.sum.is_some()
        || p.sign_mask != 0
        || p.signed_relation
    {
        return Err(Error::Identity);
    }
    let m = &map.0;
    let id = &m.identity;
    if model.len() != m.variables as usize || !(2..=15).contains(&id.m) || !(2..=61).contains(&id.n)
    {
        return Err(Error::Semantics);
    }
    let bound = 1u64 << id.n;
    let a = word(&id.a)?;
    let b = word(&id.b)?;
    if a >= bound || b == 0 || b >= bound {
        return Err(Error::Semantics);
    }
    if target.map(|v| v.0) != id.target.as_ref().map(|v| word(v)).transpose()? {
        return Err(Error::Identity);
    }
    let mut low = 0;
    for &t in &id.low_terms {
        charge(o, 4, 0)?;
        if t >= id.n {
            return Err(Error::Semantics);
        }
        low |= 1 << t;
    }
    let f = Field { n: id.n, low };
    let bit = |w: u32| -> Result<bool, Error> {
        if w == 0 {
            return Err(Error::Formula);
        }
        model.get(w as usize - 1).copied().ok_or(Error::Formula)
    };
    for row in &m.formula {
        charge(o, 4, 0)?;
        match row {
            Constraint::Clause { literals } => {
                let mut holds = false;
                for &l in literals {
                    charge(o, 4, 0)?;
                    let v = bit(l.unsigned_abs())?;
                    holds |= if l > 0 { v } else { !v };
                }
                if !holds {
                    return Err(Error::Formula);
                }
            }
            Constraint::Xor { wires, rhs } => {
                let mut parity = false;
                for &w in wires {
                    charge(o, 3, 0)?;
                    parity ^= bit(w)?;
                }
                if parity != *rhs {
                    return Err(Error::Formula);
                }
            }
        }
        p.formula_rows += 1;
    }
    if m.blocks.len() != 2 * id.m - 1 {
        return Err(Error::Semantics);
    }
    grow(&mut p.blocks, m.blocks.len(), o)?;
    for block in &m.blocks {
        if block.len() != id.n as usize {
            return Err(Error::Semantics);
        }
        let mut x = 0;
        for (i, &w) in block.iter().enumerate() {
            charge(o, 4, 0)?;
            if bit(w)? {
                x |= 1 << i;
            }
        }
        p.blocks.push(x);
    }
    for &x in &p.blocks[..id.m] {
        let mut member = false;
        for c in &id.codes {
            charge(o, 8, 0)?;
            member |= word(c)? == x;
        }
        if !member {
            return Err(Error::Semantics);
        }
    }
    if p.blocks[2 * id.m - 2] != target.map(|v| v.0).unwrap_or(0) {
        return Err(Error::Semantics);
    }
    let mut previous = p.blocks[0];
    for step in 1..id.m {
        charge(o, 16, 0)?;
        let x = p.blocks[step];
        let z = p.blocks[if step == id.m - 1 {
            2 * id.m - 2
        } else {
            id.m + step - 1
        }];
        let pi = step > 1 && id.pattern & (1 << (step - 2)) != 0;
        let zi = if step == id.m - 1 {
            target.is_none()
        } else {
            id.pattern & (1 << (step - 1)) != 0
        };
        let valid = match (pi, zi) {
            (true, true) => false,
            (true, false) => z == x,
            (false, true) => previous == x,
            (false, false) => {
                let s = f.mul(previous ^ x, previous ^ x, o)?;
                let product = f.mul(previous, x, o)?;
                let zz = f.mul(z, z, o)?;
                f.mul(s, zz, o)? ^ f.mul(product, z, o)? ^ f.mul(product, product, o)? ^ b == 0
            }
        };
        if !valid || (step < id.m - 1 && zi && z != 0) {
            return Err(Error::Semantics);
        }
        previous = z;
        p.chain_steps += 1;
    }
    if let Some(t) = target {
        if t.0 >= bound || t.1 >= bound || !f.on_curve(t, a, b, o)? {
            return Err(Error::Semantics);
        }
    }
    grow(&mut p.lift_choices, id.m, o)?;
    grow(&mut p.path, id.m, o)?;
    for &x in &p.blocks[..id.m] {
        let mut choices = [None, None];
        for &q in signed_domain {
            charge(o, 8, 0)?;
            if q.0 != x {
                continue;
            }
            if q.0 >= bound || q.1 >= bound || !f.on_curve(q, a, b, o)? {
                return Err(Error::Semantics);
            }
            if choices.contains(&Some(q)) {
                continue;
            }
            if choices[0].is_none() {
                choices[0] = Some(q)
            } else if choices[1].is_none() {
                choices[1] = Some(q)
            } else {
                return Err(Error::Semantics);
            }
        }
        if choices[0].is_none() {
            return Err(Error::Semantics);
        }
        p.lift_choices.push(choices);
    }
    for mask in 0..(1u32 << id.m) {
        charge(o, 16, 0)?;
        p.sign_mask = mask;
        p.path.clear();
        p.sum = None;
        let mut complete = true;
        for i in 0..id.m {
            let Some(q) = p.lift_choices[i][((mask >> i) & 1) as usize] else {
                complete = false;
                break;
            };
            p.path.push(q);
            p.sum = f.add(p.sum, q, a, o)?;
            // Bind each actual signed prefix to this exact authenticated model.
            // Failed comparisons keep the actual path/sum until the next mask.
            charge(o, 12, 0)?;
            let expected = if i == 0 {
                Some(p.blocks[0])
            } else if i == id.m - 1 {
                target.map(|point| point.0)
            } else if id.pattern & (1 << (i - 1)) != 0 {
                None
            } else {
                Some(p.blocks[id.m + i - 1])
            };
            if !prefix_matches(p.sum, expected) {
                complete = false;
                break;
            }
        }
        if complete && p.sum == target {
            o.check()?;
            p.signed_relation = true;
            return Ok(());
        }
    }
    Err(Error::Semantics)
}

