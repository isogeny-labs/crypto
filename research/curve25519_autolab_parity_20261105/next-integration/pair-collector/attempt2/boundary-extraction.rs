use serde::{Serialize,Deserialize}; use num_bigint::BigUint; use num_traits::ToPrimitive;
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct FieldOps {
    pub muls: u64,
    pub sqrs: u64,
    pub invs: u64,
}
impl FieldOps {
    pub fn merge(&mut self, other: FieldOps) {
        self.muls += other.muls;
        self.sqrs += other.sqrs;
        self.invs += other.invs;
    }
    /// The tally of several ledgers together.
    pub fn total(ledgers: &[GroupOps]) -> FieldOps {
        let mut f = FieldOps::default();
        for l in ledgers {
            f.merge(l.field_ops);
        }
        f
    }
}
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct GroupOps {
    pub adds: u64,
    pub doubles: u64,
    pub scalar_mults: u64,
    /// The field operations behind the additions and doublings, where
    /// the group counts them ([`CountedGroup::counts_field_ops`]); zero
    /// and meaningless elsewhere.  Not serialised with the ledger, so
    /// every report that writes a `GroupOps` is byte for byte what it
    /// was and a binary-curve ledger never prints a zero that would read
    /// as "no field work".  `ecbench` carries the tally as its own
    /// optional record block, written only when the group counted it.
    #[serde(skip)]
    pub field_ops: FieldOps,
    /// Native `from_mont` reductions; kept separate from true modular
    /// multiplications.  Existing serialised group ledgers are unchanged.
    #[serde(skip)]
    pub montgomery_reductions: u64,
}
impl GroupOps {
    /// Group-addition equivalents: additions plus doublings.
    pub fn gae(&self) -> f64 {
        (self.adds + self.doubles) as f64
    }
    /// Add another ledger's operations to this one.  Public so a
    /// framework plug-in can charge a precomputation it performed on
    /// its own ledger to the pipeline's total.
    pub fn merge(&mut self, other: GroupOps) {
        self.adds += other.adds;
        self.doubles += other.doubles;
        self.scalar_mults += other.scalar_mults;
        self.field_ops.merge(other.field_ops);
        self.montgomery_reductions += other.montgomery_reductions;
    }
}
pub trait CountedGroup {
    type Elt: Copy + PartialEq + std::fmt::Debug;
    fn identity(&self) -> Self::Elt;
    fn is_identity(&self, p: &Self::Elt) -> bool;
    fn add(&self, ops: &mut GroupOps, p: Self::Elt, q: Self::Elt) -> Self::Elt;
    fn double(&self, ops: &mut GroupOps, p: Self::Elt) -> Self::Elt;
    fn neg(&self, p: Self::Elt) -> Self::Elt;
    /// A key that identifies the element (both walks must agree on it).
    fn key(&self, p: &Self::Elt) -> u64;
    /// An opt-in addition path for the relation walk.  The closure lives for
    /// one solve, so an implementation may retain native field state between
    /// steps.  Other group operations keep their established accounting.
    fn native_walk_adder(
        &self,
    ) -> Option<Box<dyn FnMut(&mut GroupOps, Self::Elt, Self::Elt) -> Self::Elt + '_>> {
        None
    }
    /// One synchronized round of independent lanes.  Each tuple is
    /// `(lane_index, current_point, addend, addend_slot)`; outputs retain
    /// tuple order.  Slots 0..16 are jumps, 16..32 are restart offsets.
    /// The serial and batched closures must consume the same tuples.  The
    /// caller owns the output buffer so repeated rounds can reuse it.
    #[allow(clippy::type_complexity)]
    fn native_lane_walk_adder(
        &self,
        _batched: bool,
    ) -> Option<
        Box<
            dyn FnMut(&mut GroupOps, &[(usize, Self::Elt, Self::Elt, usize)], &mut Vec<Self::Elt>)
                + '_,
        >,
    > {
        None
    }
    fn supports_native_lane_walk(&self) -> bool {
        false
    }
    /// Whether `add` and `double` tally the field operations behind them
    /// into the ledger's `field_ops`.  `false` by default, and then the
    /// tally stays at zero and means *unknown*, never "no field work":
    /// a report must leave the figure absent rather than write the zero.
    fn counts_field_ops(&self) -> bool {
        false
    }
    /// `[k]P` by double-and-add, counted exactly.
    fn mul(&self, ops: &mut GroupOps, p: Self::Elt, k: u64) -> Self::Elt {
        ops.scalar_mults += 1;
        if k == 0 || self.is_identity(&p) {
            return self.identity();
        }
        let bits = 64 - k.leading_zeros();
        let mut acc = p;
        for i in (0..bits - 1).rev() {
            acc = self.double(ops, acc);
            if (k >> i) & 1 == 1 {
                acc = self.add(ops, acc, p);
            }
        }
        acc
    }

    /// `[k]P` for a scalar wider than one word.  The default converts when it
    /// fits a `u64`; a group whose subgroup order (or cofactor) needs more
    /// overrides this with its wide multiplication.
    fn mul_biguint(&self, ops: &mut GroupOps, p: Self::Elt, k: &BigUint) -> Self::Elt {
        let k = k
            .to_u64()
            .expect("this group's scalar multiplication is one word");
        self.mul(ops, p, k)
    }
}