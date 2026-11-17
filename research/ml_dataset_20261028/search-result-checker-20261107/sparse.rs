//! Sparse F2[v]/(v_i^2+v_i) source expansion; no authentication in this kernel.
//! Wires in the borrowed source are DIMACS 1-based; output indices are 0-based.
pub type Monomial = Vec<u32>;
pub type Polynomial = Vec<Monomial>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unknown {
    Cap,
    Overflow,
    Allocation,
    Owner,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Failure {
    Unknown(Unknown),
    InvalidWire,
}
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub variables: u32,
    pub generators: usize,
    pub terms: usize,
    pub degree: usize,
    pub work: u64,
    pub bytes: u64,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct Charge {
    pub work: u64,
    pub bytes: u64,
}
pub trait Owner {
    fn reserve(&mut self, charge: Charge) -> Result<(), ()>;
    fn check(&mut self) -> Result<(), ()>;
}
pub struct Source<'a> {
    pub variables: u32,
    pub ands: &'a [(u32, u32, u32)],
    pub xors: &'a [(Vec<u32>, bool)],
    pub units: &'a [(u32, bool)],
    pub impossible: bool,
    pub domain: &'a [Vec<i32>],
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    And,
    Xor,
    Unit,
    Impossible,
    Domain,
    Done,
}
#[derive(Debug)]
pub struct Attempt {
    pub result: Result<(), Failure>,
    pub equations: Vec<Polynomial>,
    pub partial: Polynomial,
    pub scratch: Polynomial,
    pub stage: Stage,
    pub row: usize,
    pub literal: usize,
    pub charge: Charge,
}
// Charge the entire requested replacement buffer, even when capacity is reused.
// Prior buffer allocations remain charged: old+new coexistence is covered.
fn allocation_bytes<T>(capacity: usize) -> Result<u64, Failure> {
    let bytes = capacity
        .checked_mul(std::mem::size_of::<T>())
        .and_then(|n| n.checked_add(64))
        .ok_or(Failure::Unknown(Unknown::Overflow))?;
    u64::try_from(bytes).map_err(|_| Failure::Unknown(Unknown::Overflow))
}
struct Context<'a, T: Owner> {
    owner: &'a mut T,
    limits: Limits,
    charge: Charge,
}
impl<T: Owner> Context<'_, T> {
    fn pay(&mut self, work: u64, bytes: u64) -> Result<(), Failure> {
        let q = Charge {
            work: self
                .charge
                .work
                .checked_add(work)
                .ok_or(Failure::Unknown(Unknown::Overflow))?,
            bytes: self
                .charge
                .bytes
                .checked_add(bytes)
                .ok_or(Failure::Unknown(Unknown::Overflow))?,
        };
        if q.work > self.limits.work || q.bytes > self.limits.bytes {
            return Err(Failure::Unknown(Unknown::Cap));
        }
        self.owner
            .reserve(Charge { work, bytes })
            .map_err(|_| Failure::Unknown(Unknown::Owner))?;
        self.charge = q;
        self.owner
            .check()
            .map_err(|_| Failure::Unknown(Unknown::Owner))
    }
    fn wire(&mut self, w: u32, n: u32) -> Result<u32, Failure> {
        self.pay(1, 0)?;
        if w == 0 || w > n {
            Err(Failure::InvalidWire)
        } else {
            Ok(w - 1)
        }
    }
    fn term(&mut self, base: &[u32], extra: Option<u32>) -> Result<Monomial, Failure> {
        let n = base
            .len()
            .checked_add(usize::from(extra.is_some()))
            .ok_or(Failure::Unknown(Unknown::Overflow))?;
        self.pay(
            (n as u64).saturating_mul(4) + 4,
            allocation_bytes::<u32>(n)?,
        )?;
        let mut t = Vec::new();
        t.try_reserve_exact(n)
            .map_err(|_| Failure::Unknown(Unknown::Allocation))?;
        t.extend_from_slice(base);
        if let Some(v) = extra {
            match t.binary_search(&v) {
                Ok(_) => {}
                Err(i) => t.insert(i, v),
            }
        }
        if t.len() > self.limits.degree {
            return Err(Failure::Unknown(Unknown::Cap));
        }
        Ok(t)
    }
    fn toggle(&mut self, p: &mut Polynomial, t: Monomial) -> Result<(), Failure> {
        // Charge a full linear comparison/move envelope, not only binary-search probes.
        let work = (p.len() as u64 + 1)
            .checked_mul(
                (t.len() as u64)
                    .checked_add(self.limits.degree as u64)
                    .and_then(|n| n.checked_add(4))
                    .ok_or(Failure::Unknown(Unknown::Overflow))?,
            )
            .ok_or(Failure::Unknown(Unknown::Overflow))?;
        self.pay(work, 0)?;
        match p.binary_search(&t) {
            Ok(i) => {
                p.remove(i);
            }
            Err(i) => {
                if p.len() >= self.limits.terms {
                    return Err(Failure::Unknown(Unknown::Cap));
                }
                let requested = p
                    .len()
                    .checked_add(1)
                    .ok_or(Failure::Unknown(Unknown::Overflow))?;
                self.pay(requested as u64, allocation_bytes::<Monomial>(requested)?)?;
                p.try_reserve_exact(1)
                    .map_err(|_| Failure::Unknown(Unknown::Allocation))?;
                p.insert(i, t);
            }
        }
        Ok(())
    }
    fn add(&mut self, p: &mut Polynomial, base: &[u32], extra: Option<u32>) -> Result<(), Failure> {
        let t = self.term(base, extra)?;
        self.toggle(p, t)
    }
    fn finish(&mut self, out: &mut Attempt) -> Result<(), Failure> {
        if out.equations.len() >= self.limits.generators {
            return Err(Failure::Unknown(Unknown::Cap));
        }
        let requested = out
            .equations
            .len()
            .checked_add(1)
            .ok_or(Failure::Unknown(Unknown::Overflow))?;
        self.pay(requested as u64, allocation_bytes::<Polynomial>(requested)?)?;
        out.equations
            .try_reserve_exact(1)
            .map_err(|_| Failure::Unknown(Unknown::Allocation))?;
        out.equations.push(std::mem::take(&mut out.partial));
        Ok(())
    }
}
pub fn expand(source: Source<'_>, limits: Limits, owner: &mut impl Owner) -> Attempt {
    let mut out = Attempt {
        result: Ok(()),
        equations: Vec::new(),
        partial: Vec::new(),
        scratch: Vec::new(),
        stage: Stage::And,
        row: 0,
        literal: 0,
        charge: Charge::default(),
    };
    let mut c = Context {
        owner,
        limits,
        charge: Charge::default(),
    };
    out.result = (|| {
        c.pay(1, 0)?;
        if source.variables > limits.variables {
            return Err(Failure::Unknown(Unknown::Cap));
        }
        for (i, &(g, l, r)) in source.ands.iter().enumerate() {
            out.stage = Stage::And;
            out.row = i;
            let g = c.wire(g, source.variables)?;
            let l = c.wire(l, source.variables)?;
            let r = c.wire(r, source.variables)?;
            c.add(&mut out.partial, &[], Some(g))?;
            c.add(&mut out.partial, &[l], Some(r))?;
            c.finish(&mut out)?;
        }
        for (i, (wires, rhs)) in source.xors.iter().enumerate() {
            out.stage = Stage::Xor;
            out.row = i;
            for &w in wires {
                let w = c.wire(w, source.variables)?;
                c.add(&mut out.partial, &[], Some(w))?;
            }
            if *rhs {
                c.add(&mut out.partial, &[], None)?;
            }
            c.finish(&mut out)?;
        }
        for (i, &(w, rhs)) in source.units.iter().enumerate() {
            out.stage = Stage::Unit;
            out.row = i;
            let w = c.wire(w, source.variables)?;
            c.add(&mut out.partial, &[], Some(w))?;
            if rhs {
                c.add(&mut out.partial, &[], None)?;
            }
            c.finish(&mut out)?;
        }
        if source.impossible {
            out.stage = Stage::Impossible;
            out.row = 0;
            c.add(&mut out.partial, &[], None)?;
            c.finish(&mut out)?;
        }
        for (i, clause) in source.domain.iter().enumerate() {
            out.stage = Stage::Domain;
            out.row = i;
            c.add(&mut out.partial, &[], None)?;
            for (j, &lit) in clause.iter().enumerate() {
                out.literal = j;
                let w = c.wire(lit.unsigned_abs(), source.variables)?;
                // Positive x: unsatisfied=1+x; negative !x: unsatisfied=x.
                for term in &out.partial {
                    if lit > 0 {
                        c.add(&mut out.scratch, term, None)?;
                    }
                    c.add(&mut out.scratch, term, Some(w))?;
                }
                out.partial = std::mem::take(&mut out.scratch);
            }
            c.finish(&mut out)?;
        }
        out.stage = Stage::Done;
        Ok(())
    })();
    out.charge = c.charge;
    out
}
#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Default)]
    struct Ledger {
        work: u64,
        bytes: u64,
        checks: usize,
        stop: bool,
        deadline: Option<usize>,
    }
    impl Owner for Ledger {
        fn reserve(&mut self, q: Charge) -> Result<(), ()> {
            if self.stop {
                return Err(());
            }
            self.work += q.work;
            self.bytes += q.bytes;
            Ok(())
        }
        fn check(&mut self) -> Result<(), ()> {
            self.checks += 1;
            if self.deadline.is_some_and(|d| self.checks >= d) {
                self.stop = true;
                Err(())
            } else {
                Ok(())
            }
        }
    }
    fn limits() -> Limits {
        Limits {
            variables: u32::MAX,
            generators: 128,
            terms: 128,
            degree: 128,
            work: 1_000_000,
            bytes: 1_000_000,
        }
    }
    fn eval(p: &Polynomial, assignment: &[bool]) -> bool {
        p.iter()
            .fold(false, |a, t| a ^ t.iter().all(|&v| assignment[v as usize]))
    }
    fn source<'a>(domain: &'a [Vec<i32>]) -> Source<'a> {
        Source {
            variables: 3,
            ands: &[],
            xors: &[],
            units: &[],
            impossible: false,
            domain,
        }
    }
    #[test]
    fn exhaustive_clauses_duplicates_opposites() {
        let clauses = vec![
            vec![],
            vec![1],
            vec![-1],
            vec![1, 1],
            vec![1, -1],
            vec![1, -2, 3],
            vec![-2, -2],
        ];
        let a = expand(source(&clauses), limits(), &mut Ledger::default());
        assert!(a.result.is_ok());
        for mask in 0..8 {
            let bits = [mask & 1 != 0, mask & 2 != 0, mask & 4 != 0];
            for (cl, p) in clauses.iter().zip(&a.equations) {
                let satisfied = cl
                    .iter()
                    .any(|&l| bits[l.unsigned_abs() as usize - 1] == (l > 0));
                assert_eq!(eval(p, &bits), !satisfied);
            }
        }
    }
    #[test]
    fn and_xor_unit_impossible_semantics() {
        let ands = [(3, 1, 2), (1, 1, 1)];
        let xors = [(vec![1, 2, 1, 3], true), (vec![2, 2], false)];
        let units = [(2, true)];
        let a = expand(
            Source {
                variables: 3,
                ands: &ands,
                xors: &xors,
                units: &units,
                impossible: true,
                domain: &[],
            },
            limits(),
            &mut Ledger::default(),
        );
        assert!(a.result.is_ok());
        assert_eq!(a.equations.len(), 6);
        assert!(a.equations[1].is_empty());
        assert!(a.equations[3].is_empty());
        for m in 0..8 {
            let b = [m & 1 != 0, m & 2 != 0, m & 4 != 0];
            assert_eq!(eval(&a.equations[0], &b), b[2] ^ (b[0] & b[1]));
            assert_eq!(eval(&a.equations[2], &b), b[1] ^ b[2] ^ true);
            assert_eq!(eval(&a.equations[4], &b), !b[1]);
            assert!(eval(&a.equations[5], &b));
        }
    }
    #[test]
    fn high_indices_sparse() {
        let ands = [(100_000, 99999, 70)];
        let a = expand(
            Source {
                variables: 100_000,
                ands: &ands,
                ..source(&[])
            },
            limits(),
            &mut Ledger::default(),
        );
        assert!(a.result.is_ok());
        assert_eq!(a.equations[0], vec![vec![69, 99998], vec![99999]]);
        assert!(a.charge.bytes < 4096);
    }
    #[test]
    fn caps_retain_partial() {
        let d = [vec![1, 2, 3]];
        let mut l = limits();
        l.terms = 2;
        let a = expand(source(&d), l, &mut Ledger::default());
        assert_eq!(a.result, Err(Failure::Unknown(Unknown::Cap)));
        assert_eq!(a.stage, Stage::Domain);
        assert!(!a.partial.is_empty());
        assert!(!a.scratch.is_empty());
    }
    #[test]
    fn deadline_and_refusal_never_reset() {
        let d = [vec![1, 2, 3]];
        let mut owner = Ledger {
            deadline: Some(5),
            ..Ledger::default()
        };
        let a = expand(source(&d), limits(), &mut owner);
        assert_eq!(a.result, Err(Failure::Unknown(Unknown::Owner)));
        let old = owner.work;
        let b = expand(source(&d), limits(), &mut owner);
        assert!(b.result.is_err());
        assert_eq!(owner.work, old);
        assert!(old > 0);
    }
    #[test]
    fn invalid_wire_and_zero_limits() {
        let d = [vec![0]];
        assert_eq!(
            expand(source(&d), limits(), &mut Ledger::default()).result,
            Err(Failure::InvalidWire)
        );
        let mut l = limits();
        l.work = 0;
        assert_eq!(
            expand(source(&[]), l, &mut Ledger::default()).result,
            Err(Failure::Unknown(Unknown::Cap))
        );
    }
    #[test]
    fn vector_growth_reserves_full_replacement_before_mutation() {
        let mut owner = Ledger::default();
        let mut c = Context {
            owner: &mut owner,
            limits: limits(),
            charge: Charge::default(),
        };
        let mut p = vec![vec![0], vec![1], vec![2], vec![3]];
        c.toggle(&mut p, vec![4]).unwrap();
        assert_eq!(c.charge.bytes, allocation_bytes::<Monomial>(5).unwrap());
        let mut tiny = limits();
        tiny.bytes = 64;
        let mut owner = Ledger::default();
        let mut c = Context {
            owner: &mut owner,
            limits: tiny,
            charge: Charge::default(),
        };
        let snapshot = p.clone();
        assert_eq!(
            c.toggle(&mut p, vec![5]),
            Err(Failure::Unknown(Unknown::Cap))
        );
        assert_eq!(p, snapshot);
        assert_eq!(c.charge.bytes, 0);
        let mut out = Attempt {
            result: Ok(()),
            equations: vec![vec![], vec![], vec![]],
            partial: vec![vec![1]],
            scratch: vec![],
            stage: Stage::Xor,
            row: 0,
            literal: 0,
            charge: Charge::default(),
        };
        assert_eq!(c.finish(&mut out), Err(Failure::Unknown(Unknown::Cap)));
        assert_eq!(out.equations.len(), 3);
        assert_eq!(out.partial, vec![vec![1]]);
        assert_eq!(
            allocation_bytes::<Monomial>(usize::MAX),
            Err(Failure::Unknown(Unknown::Overflow))
        );
    }
}
