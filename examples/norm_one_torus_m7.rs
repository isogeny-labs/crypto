//! Exact cold single-target m=7 decomposition over the production p+1
//! norm-one-torus factor base.
//!
//! Every arm rebuilds its retained table for the same one public target. The
//! runner compares target-centered enumeration, a conventional sum-then-
//! subtract loop, Rust hashing, and an exact compact 16-bit prefix directory.

#[path = "support/norm_one_torus_m7.rs"]
mod common;

#[allow(dead_code)]
mod engine {
    include!("support/norm_one_torus_compare_kernel.rs");

    use std::fs;
    use std::io::Write;
    use std::mem::size_of;

    use super::common::{ArmCertificate, OperationRecord};

    const PREFIX_BITS: u32 = 16;
    const PREFIX_BUCKETS: usize = 1usize << PREFIX_BITS;
    const FINITE_X_BITS: u32 = 31;
    const RETAINED_THREE_STATES: usize = 27_370;
    const RETAINED_FOUR_STATES: usize = 486_115;

    impl From<OpCounts> for OperationRecord {
        fn from(value: OpCounts) -> Self {
            Self {
                multiplications: value.multiplications,
                squarings: value.squarings,
                inversions: value.inversions,
                additions: value.additions,
                doublings: value.doublings,
            }
        }
    }

    impl Jacobian {
        fn negated(self, modulus: u64) -> Self {
            if self.z == 0 || self.y == 0 {
                self
            } else {
                Self {
                    y: modulus - self.y,
                    ..self
                }
            }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub(super) enum Arm {
        Retain4HashSum,
        Retain4HashCentered,
        Retain4PrefixCentered,
        Retain3PrefixCentered,
        Retain3PrefixAffineLeaf,
    }

    impl Arm {
        pub(super) const ALL: [Self; 5] = [
            Self::Retain4HashSum,
            Self::Retain4HashCentered,
            Self::Retain4PrefixCentered,
            Self::Retain3PrefixCentered,
            Self::Retain3PrefixAffineLeaf,
        ];

        fn name(self) -> &'static str {
            match self {
                Self::Retain4HashSum => "retain4-hash-sum",
                Self::Retain4HashCentered => "retain4-hash-centered",
                Self::Retain4PrefixCentered => "retain4-prefix16-centered",
                Self::Retain3PrefixCentered => "retain3-prefix16-centered",
                Self::Retain3PrefixAffineLeaf => "retain3-prefix16-affine-leaf",
            }
        }

        fn retained_arity(self) -> usize {
            match self {
                Self::Retain3PrefixCentered | Self::Retain3PrefixAffineLeaf => 3,
                _ => 4,
            }
        }

        fn uses_prefix(self) -> bool {
            matches!(
                self,
                Self::Retain4PrefixCentered
                    | Self::Retain3PrefixCentered
                    | Self::Retain3PrefixAffineLeaf
            )
        }

        fn centered(self) -> bool {
            !matches!(self, Self::Retain4HashSum)
        }
    }

    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    struct CompactValue {
        y: u32,
        witness: u32,
    }

    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    struct CompactEntry {
        x: u32,
        y: u32,
        witness: u32,
    }

    #[derive(Debug)]
    struct HashTable {
        finite: HashMap<u32, CompactValue>,
        infinity: Option<CompactValue>,
        retained_arity: usize,
    }

    #[derive(Debug)]
    struct PrefixTable {
        offsets: Vec<u32>,
        entries: Vec<CompactEntry>,
        infinity: Option<CompactValue>,
        retained_arity: usize,
    }

    #[derive(Debug)]
    enum RetainedTable {
        Hash(HashTable),
        Prefix(PrefixTable),
    }

    #[derive(Clone, Copy, Debug, Default)]
    struct LookupCounts {
        probes: u64,
        comparisons: u64,
    }

    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    struct AffineBranchCounts {
        generic: u64,
        inverse: u64,
        doubling: u64,
        identity: u64,
    }

    impl AffineBranchCounts {
        fn total(self) -> u64 {
            self.generic + self.inverse + self.doubling + self.identity
        }
    }

    impl RetainedTable {
        fn distinct_keys(&self) -> usize {
            match self {
                Self::Hash(table) => table.finite.len() + usize::from(table.infinity.is_some()),
                Self::Prefix(table) => table.entries.len() + usize::from(table.infinity.is_some()),
            }
        }

        fn payload_bytes(&self) -> usize {
            match self {
                Self::Hash(table) => {
                    table.finite.len() * (size_of::<u32>() + size_of::<CompactValue>())
                        + usize::from(table.infinity.is_some()) * size_of::<CompactValue>()
                }
                Self::Prefix(table) => {
                    table.entries.len() * size_of::<CompactEntry>()
                        + usize::from(table.infinity.is_some()) * size_of::<CompactValue>()
                }
            }
        }

        fn index_bytes(&self) -> usize {
            match self {
                Self::Hash(_) => 0,
                Self::Prefix(table) => table.offsets.len() * size_of::<u32>(),
            }
        }

        fn container_capacity(&self) -> usize {
            match self {
                Self::Hash(table) => table.finite.capacity(),
                Self::Prefix(table) => table.entries.capacity(),
            }
        }

        fn lookup(
            &self,
            point: PrimePoint,
            fixture: &Fixture,
            counts: &mut LookupCounts,
        ) -> Option<Vec<usize>> {
            counts.probes += 1;
            match self {
                Self::Hash(table) => {
                    let value = if point.infinity {
                        table.infinity?
                    } else {
                        *table.finite.get(&(point.x as u32))?
                    };
                    Some(oriented_witness(
                        value,
                        point,
                        table.retained_arity,
                        fixture,
                    ))
                }
                Self::Prefix(table) => {
                    if point.infinity {
                        return table.infinity.map(|value| {
                            oriented_witness(value, point, table.retained_arity, fixture)
                        });
                    }
                    let x = point.x as u32;
                    let bucket = prefix_bucket(x);
                    let start = table.offsets[bucket] as usize;
                    let end = table.offsets[bucket + 1] as usize;
                    for entry in &table.entries[start..end] {
                        counts.comparisons += 1;
                        if entry.x == x {
                            return Some(oriented_witness(
                                CompactValue {
                                    y: entry.y,
                                    witness: entry.witness,
                                },
                                point,
                                table.retained_arity,
                                fixture,
                            ));
                        }
                    }
                    None
                }
            }
        }
    }

    #[derive(Debug)]
    pub(super) struct TimedArm {
        pub certificate: ArmCertificate,
        pub setup: Duration,
        pub query: Duration,
    }

    fn prefix_bucket(x: u32) -> usize {
        (x >> (FINITE_X_BITS - PREFIX_BITS)) as usize
    }

    fn pack_witness(indices: &[u8]) -> u32 {
        assert!((1..=4).contains(&indices.len()));
        indices
            .iter()
            .enumerate()
            .fold(0u32, |packed, (slot, &index)| {
                assert!(index < 128);
                packed | (u32::from(index) << (7 * slot))
            })
    }

    fn unpack_witness(packed: u32, len: usize) -> Vec<usize> {
        (0..len)
            .map(|slot| ((packed >> (7 * slot)) & 0x7f) as usize)
            .collect()
    }

    fn negate_indices<const N: usize>(mut witness: [u8; N], neg_index: &[usize]) -> [u8; N] {
        for index in &mut witness {
            *index = neg_index[usize::from(*index)] as u8;
        }
        witness.sort_unstable();
        witness
    }

    fn oriented_witness(
        value: CompactValue,
        point: PrimePoint,
        retained_arity: usize,
        fixture: &Fixture,
    ) -> Vec<usize> {
        let mut witness = unpack_witness(value.witness, retained_arity);
        if !point.infinity && u64::from(value.y) != point.y {
            assert_eq!(u64::from(value.y), fixture.instance.curve.p - point.y);
            for index in &mut witness {
                *index = fixture.neg_index[*index];
            }
            witness.sort_unstable();
        }
        witness
    }

    fn retained_three_states(fixture: &Fixture, ops: &mut OpCounts) -> Vec<(Jacobian, u32)> {
        let mut states = Vec::with_capacity(RETAINED_THREE_STATES);
        for first in 0..fixture.base.len() {
            let left = Jacobian::from_affine(fixture.base[first]);
            for second in first..fixture.base.len() {
                let pair = left.add_mixed(
                    fixture.base[second],
                    fixture.instance.curve.a,
                    fixture.instance.curve.p,
                    ops,
                );
                for third in second..fixture.base.len() {
                    let witness = [first as u8, second as u8, third as u8];
                    if witness > negate_indices(witness, &fixture.neg_index) {
                        continue;
                    }
                    states.push((
                        pair.add_mixed(
                            fixture.base[third],
                            fixture.instance.curve.a,
                            fixture.instance.curve.p,
                            ops,
                        ),
                        pack_witness(&witness),
                    ));
                }
            }
        }
        assert_eq!(states.len(), RETAINED_THREE_STATES);
        states
    }

    fn retained_four_states(fixture: &Fixture, ops: &mut OpCounts) -> Vec<(Jacobian, u32)> {
        let mut states = Vec::with_capacity(RETAINED_FOUR_STATES);
        for first in 0..fixture.base.len() {
            let left = Jacobian::from_affine(fixture.base[first]);
            for second in first..fixture.base.len() {
                let pair = left.add_mixed(
                    fixture.base[second],
                    fixture.instance.curve.a,
                    fixture.instance.curve.p,
                    ops,
                );
                for third in second..fixture.base.len() {
                    let triple = pair.add_mixed(
                        fixture.base[third],
                        fixture.instance.curve.a,
                        fixture.instance.curve.p,
                        ops,
                    );
                    for fourth in third..fixture.base.len() {
                        let witness = [first as u8, second as u8, third as u8, fourth as u8];
                        if witness > negate_indices(witness, &fixture.neg_index) {
                            continue;
                        }
                        states.push((
                            triple.add_mixed(
                                fixture.base[fourth],
                                fixture.instance.curve.a,
                                fixture.instance.curve.p,
                                ops,
                            ),
                            pack_witness(&witness),
                        ));
                    }
                }
            }
        }
        assert_eq!(states.len(), RETAINED_FOUR_STATES);
        states
    }

    fn hash_table(points: Vec<(PrimePoint, u32)>, retained_arity: usize) -> RetainedTable {
        let mut finite = HashMap::with_capacity(points.len());
        let mut infinity = None;
        for (point, witness) in points {
            let value = CompactValue {
                y: point.y as u32,
                witness,
            };
            if point.infinity {
                infinity.get_or_insert(value);
            } else {
                finite.entry(point.x as u32).or_insert(value);
            }
        }
        RetainedTable::Hash(HashTable {
            finite,
            infinity,
            retained_arity,
        })
    }

    fn prefix_table(points: Vec<(PrimePoint, u32)>, retained_arity: usize) -> RetainedTable {
        let mut finite = Vec::with_capacity(points.len());
        let mut infinity = None;
        for (point, witness) in points {
            let value = CompactValue {
                y: point.y as u32,
                witness,
            };
            if point.infinity {
                infinity.get_or_insert(value);
            } else {
                finite.push(CompactEntry {
                    x: point.x as u32,
                    y: value.y,
                    witness,
                });
            }
        }

        let mut counts = vec![0usize; PREFIX_BUCKETS];
        for entry in &finite {
            counts[prefix_bucket(entry.x)] += 1;
        }
        let mut starts = vec![0usize; PREFIX_BUCKETS + 1];
        for bucket in 0..PREFIX_BUCKETS {
            starts[bucket + 1] = starts[bucket] + counts[bucket];
        }
        let mut cursors = starts[..PREFIX_BUCKETS].to_vec();
        let mut placed = vec![CompactEntry::default(); finite.len()];
        for entry in finite {
            let bucket = prefix_bucket(entry.x);
            placed[cursors[bucket]] = entry;
            cursors[bucket] += 1;
        }

        let mut offsets = Vec::with_capacity(PREFIX_BUCKETS + 1);
        let mut entries = Vec::with_capacity(placed.len());
        for bucket in 0..PREFIX_BUCKETS {
            offsets.push(entries.len() as u32);
            let slice = &mut placed[starts[bucket]..starts[bucket + 1]];
            slice.sort_unstable_by_key(|entry| entry.x);
            let mut previous = None;
            for &entry in slice.iter() {
                if previous != Some(entry.x) {
                    entries.push(entry);
                    previous = Some(entry.x);
                }
            }
        }
        offsets.push(entries.len() as u32);
        assert_eq!(offsets.len(), PREFIX_BUCKETS + 1);

        RetainedTable::Prefix(PrefixTable {
            offsets,
            entries,
            infinity,
            retained_arity,
        })
    }

    fn build_table(fixture: &Fixture, arm: Arm) -> (RetainedTable, usize, OpCounts) {
        let mut ops = OpCounts::default();
        let states = match arm.retained_arity() {
            3 => retained_three_states(fixture, &mut ops),
            4 => retained_four_states(fixture, &mut ops),
            _ => unreachable!(),
        };
        let retained_states = states.len();
        let points = normalize_batch(&states, fixture.instance.curve.p, &mut ops);
        let table = if arm.uses_prefix() {
            prefix_table(points, arm.retained_arity())
        } else {
            hash_table(points, arm.retained_arity())
        };
        let expected_additions = if arm.retained_arity() == 4 {
            543_201
        } else {
            29_716
        };
        assert_eq!(ops.additions, expected_additions);
        (table, retained_states, ops)
    }

    fn probe_block(
        block: &[(Jacobian, u32)],
        stream_arity: usize,
        table: &RetainedTable,
        fixture: &Fixture,
        ops: &mut OpCounts,
        lookup: &mut LookupCounts,
        result: &mut Option<Vec<usize>>,
    ) {
        for (point, packed_stream) in normalize_batch(block, fixture.instance.curve.p, ops) {
            if let Some(retained) = table.lookup(point, fixture, lookup) {
                if result.is_none() {
                    let mut witness = unpack_witness(packed_stream, stream_arity);
                    witness.extend(retained);
                    *result = Some(witness);
                }
            }
        }
    }

    fn push_candidate(
        block: &mut Vec<(Jacobian, u32)>,
        candidate: (Jacobian, u32),
        block_size: usize,
        stream_arity: usize,
        table: &RetainedTable,
        fixture: &Fixture,
        ops: &mut OpCounts,
        lookup: &mut LookupCounts,
        result: &mut Option<Vec<usize>>,
        candidates: &mut u64,
    ) {
        block.push(candidate);
        *candidates += 1;
        if block.len() == block_size {
            probe_block(block, stream_arity, table, fixture, ops, lookup, result);
            block.clear();
        }
    }

    fn query_retain4_sum(
        fixture: &Fixture,
        table: &RetainedTable,
        block_size: usize,
        ops: &mut OpCounts,
        lookup: &mut LookupCounts,
    ) -> (Option<Vec<usize>>, u64) {
        let mut block = Vec::with_capacity(block_size);
        let mut result = None;
        let mut candidates = 0;
        for first in 0..fixture.base.len() {
            let left = Jacobian::from_affine(fixture.base[first]);
            for second in first..fixture.base.len() {
                let pair = left.add_mixed(
                    fixture.base[second],
                    fixture.instance.curve.a,
                    fixture.instance.curve.p,
                    ops,
                );
                for third in second..fixture.base.len() {
                    let triple = pair.add_mixed(
                        fixture.base[third],
                        fixture.instance.curve.a,
                        fixture.instance.curve.p,
                        ops,
                    );
                    let complement = triple.negated(fixture.instance.curve.p).add_mixed(
                        fixture.target,
                        fixture.instance.curve.a,
                        fixture.instance.curve.p,
                        ops,
                    );
                    push_candidate(
                        &mut block,
                        (
                            complement,
                            pack_witness(&[first as u8, second as u8, third as u8]),
                        ),
                        block_size,
                        3,
                        table,
                        fixture,
                        ops,
                        lookup,
                        &mut result,
                        &mut candidates,
                    );
                }
            }
        }
        if !block.is_empty() {
            probe_block(&block, 3, table, fixture, ops, lookup, &mut result);
        }
        assert_eq!(ops.additions, 111_826);
        assert_eq!(candidates, 54_740);
        (result, candidates)
    }

    fn query_retain4_centered(
        fixture: &Fixture,
        table: &RetainedTable,
        block_size: usize,
        ops: &mut OpCounts,
        lookup: &mut LookupCounts,
    ) -> (Option<Vec<usize>>, u64) {
        let mut block = Vec::with_capacity(block_size);
        let mut result = None;
        let mut candidates = 0;
        for first in 0..fixture.base.len() {
            let after_first = Jacobian::from_affine(fixture.target).add_mixed(
                fixture.base[fixture.neg_index[first]],
                fixture.instance.curve.a,
                fixture.instance.curve.p,
                ops,
            );
            for second in first..fixture.base.len() {
                let after_second = after_first.add_mixed(
                    fixture.base[fixture.neg_index[second]],
                    fixture.instance.curve.a,
                    fixture.instance.curve.p,
                    ops,
                );
                for third in second..fixture.base.len() {
                    let complement = after_second.add_mixed(
                        fixture.base[fixture.neg_index[third]],
                        fixture.instance.curve.a,
                        fixture.instance.curve.p,
                        ops,
                    );
                    push_candidate(
                        &mut block,
                        (
                            complement,
                            pack_witness(&[first as u8, second as u8, third as u8]),
                        ),
                        block_size,
                        3,
                        table,
                        fixture,
                        ops,
                        lookup,
                        &mut result,
                        &mut candidates,
                    );
                }
            }
        }
        if !block.is_empty() {
            probe_block(&block, 3, table, fixture, ops, lookup, &mut result);
        }
        assert_eq!(ops.additions, 57_154);
        assert_eq!(candidates, 54_740);
        (result, candidates)
    }

    fn query_retain3_centered(
        fixture: &Fixture,
        table: &RetainedTable,
        block_size: usize,
        ops: &mut OpCounts,
        lookup: &mut LookupCounts,
    ) -> (Option<Vec<usize>>, u64) {
        let mut block = Vec::with_capacity(block_size);
        let mut result = None;
        let mut candidates = 0;
        for first in 0..fixture.base.len() {
            let after_first = Jacobian::from_affine(fixture.target).add_mixed(
                fixture.base[fixture.neg_index[first]],
                fixture.instance.curve.a,
                fixture.instance.curve.p,
                ops,
            );
            for second in first..fixture.base.len() {
                let after_second = after_first.add_mixed(
                    fixture.base[fixture.neg_index[second]],
                    fixture.instance.curve.a,
                    fixture.instance.curve.p,
                    ops,
                );
                for third in second..fixture.base.len() {
                    let after_third = after_second.add_mixed(
                        fixture.base[fixture.neg_index[third]],
                        fixture.instance.curve.a,
                        fixture.instance.curve.p,
                        ops,
                    );
                    for fourth in third..fixture.base.len() {
                        let complement = after_third.add_mixed(
                            fixture.base[fixture.neg_index[fourth]],
                            fixture.instance.curve.a,
                            fixture.instance.curve.p,
                            ops,
                        );
                        push_candidate(
                            &mut block,
                            (
                                complement,
                                pack_witness(&[
                                    first as u8,
                                    second as u8,
                                    third as u8,
                                    fourth as u8,
                                ]),
                            ),
                            block_size,
                            4,
                            table,
                            fixture,
                            ops,
                            lookup,
                            &mut result,
                            &mut candidates,
                        );
                    }
                }
            }
        }
        if !block.is_empty() {
            probe_block(&block, 4, table, fixture, ops, lookup, &mut result);
        }
        assert_eq!(ops.additions, 1_028_789);
        assert_eq!(candidates, 971_635);
        (result, candidates)
    }

    #[derive(Clone, Copy)]
    struct AffineSlopeWork {
        output_index: usize,
        x1: u64,
        y1: u64,
        x2: u64,
        numerator: u64,
    }

    fn batch_affine_add(
        block: &[(PrimePoint, PrimePoint, u32)],
        fixture: &Fixture,
        ops: &mut OpCounts,
        branches: &mut AffineBranchCounts,
    ) -> Vec<(PrimePoint, u32)> {
        let modulus = fixture.instance.curve.p;
        let curve_a = fixture.instance.curve.a;
        let mut output: Vec<_> = block
            .iter()
            .map(|&(_, _, witness)| (PrimePoint::INFINITY, witness))
            .collect();
        let mut denominators = Vec::with_capacity(block.len());
        let mut work = Vec::with_capacity(block.len());

        for (output_index, &(left, right, _)) in block.iter().enumerate() {
            ops.additions += 1;
            if left.infinity {
                branches.identity += 1;
                output[output_index].0 = right;
                continue;
            }
            if right.infinity {
                branches.identity += 1;
                output[output_index].0 = left;
                continue;
            }

            let (numerator, denominator) = if left.x == right.x {
                if left.y != right.y || left.y == 0 {
                    branches.inverse += 1;
                    output[output_index].0 = PrimePoint::INFINITY;
                    continue;
                }
                branches.doubling += 1;
                ops.doublings += 1;
                let xx = ops.square(left.x, modulus);
                let three_xx = add_mod(add_mod(xx, xx, modulus), xx, modulus);
                (
                    add_mod(three_xx, curve_a, modulus),
                    add_mod(left.y, left.y, modulus),
                )
            } else {
                branches.generic += 1;
                (
                    sub_mod(right.y, left.y, modulus),
                    sub_mod(right.x, left.x, modulus),
                )
            };
            assert_ne!(denominator, 0);
            denominators.push(denominator);
            work.push(AffineSlopeWork {
                output_index,
                x1: left.x,
                y1: left.y,
                x2: right.x,
                numerator,
            });
        }

        batch_invert(&mut denominators, modulus, ops);
        for (item, inverse) in work.into_iter().zip(denominators) {
            let slope = ops.mul(item.numerator, inverse, modulus);
            let x3 = sub_mod(
                sub_mod(ops.square(slope, modulus), item.x1, modulus),
                item.x2,
                modulus,
            );
            let y3 = sub_mod(
                ops.mul(slope, sub_mod(item.x1, x3, modulus), modulus),
                item.y1,
                modulus,
            );
            output[item.output_index].0 = PrimePoint::affine(x3, y3);
        }
        output
    }

    fn probe_affine_block(
        block: &[(PrimePoint, PrimePoint, u32)],
        table: &RetainedTable,
        fixture: &Fixture,
        ops: &mut OpCounts,
        lookup: &mut LookupCounts,
        branches: &mut AffineBranchCounts,
        result: &mut Option<Vec<usize>>,
    ) {
        for (point, packed_stream) in batch_affine_add(block, fixture, ops, branches) {
            if let Some(retained) = table.lookup(point, fixture, lookup) {
                if result.is_none() {
                    let mut witness = unpack_witness(packed_stream, 4);
                    witness.extend(retained);
                    *result = Some(witness);
                }
            }
        }
    }

    fn centered_three_prefixes(fixture: &Fixture, ops: &mut OpCounts) -> Vec<(Jacobian, u32)> {
        let mut prefixes = Vec::with_capacity(54_740);
        for first in 0..fixture.base.len() {
            let after_first = Jacobian::from_affine(fixture.target).add_mixed(
                fixture.base[fixture.neg_index[first]],
                fixture.instance.curve.a,
                fixture.instance.curve.p,
                ops,
            );
            for second in first..fixture.base.len() {
                let after_second = after_first.add_mixed(
                    fixture.base[fixture.neg_index[second]],
                    fixture.instance.curve.a,
                    fixture.instance.curve.p,
                    ops,
                );
                for third in second..fixture.base.len() {
                    prefixes.push((
                        after_second.add_mixed(
                            fixture.base[fixture.neg_index[third]],
                            fixture.instance.curve.a,
                            fixture.instance.curve.p,
                            ops,
                        ),
                        pack_witness(&[first as u8, second as u8, third as u8]),
                    ));
                }
            }
        }
        assert_eq!(prefixes.len(), 54_740);
        assert_eq!(ops.additions, 57_154);
        prefixes
    }

    fn query_retain3_affine_leaf(
        fixture: &Fixture,
        table: &RetainedTable,
        block_size: usize,
        ops: &mut OpCounts,
        lookup: &mut LookupCounts,
    ) -> (Option<Vec<usize>>, u64, AffineBranchCounts) {
        let prefix_states = centered_three_prefixes(fixture, ops);
        let prefixes = normalize_batch(&prefix_states, fixture.instance.curve.p, ops);
        let mut block = Vec::with_capacity(block_size);
        let mut result = None;
        let mut candidates = 0u64;
        let mut branches = AffineBranchCounts::default();

        for (prefix, packed_prefix) in prefixes {
            let third = ((packed_prefix >> 14) & 0x7f) as usize;
            for fourth in third..fixture.base.len() {
                block.push((
                    prefix,
                    fixture.base[fixture.neg_index[fourth]],
                    packed_prefix | ((fourth as u32) << 21),
                ));
                candidates += 1;
                if block.len() == block_size {
                    probe_affine_block(
                        &block,
                        table,
                        fixture,
                        ops,
                        lookup,
                        &mut branches,
                        &mut result,
                    );
                    block.clear();
                }
            }
        }
        if !block.is_empty() {
            probe_affine_block(
                &block,
                table,
                fixture,
                ops,
                lookup,
                &mut branches,
                &mut result,
            );
        }

        assert_eq!(candidates, 971_635);
        assert_eq!(branches.total(), candidates);
        assert_eq!(ops.additions, 1_028_789);
        assert_eq!(ops.inversions, 476);
        (result, candidates, branches)
    }

    pub(super) fn time_arm(fixture: &Fixture, arm: Arm, block_size: usize) -> TimedArm {
        let started = Instant::now();
        let (table, retained_states, setup_ops) = build_table(fixture, arm);
        let setup = started.elapsed();

        let started = Instant::now();
        let mut query_ops = OpCounts::default();
        let mut lookup = LookupCounts::default();
        let (witness, candidates, affine_branches) = match arm {
            Arm::Retain4HashSum => {
                let (witness, candidates) =
                    query_retain4_sum(fixture, &table, block_size, &mut query_ops, &mut lookup);
                (witness, candidates, AffineBranchCounts::default())
            }
            Arm::Retain4HashCentered | Arm::Retain4PrefixCentered => {
                let (witness, candidates) = query_retain4_centered(
                    fixture,
                    &table,
                    block_size,
                    &mut query_ops,
                    &mut lookup,
                );
                (witness, candidates, AffineBranchCounts::default())
            }
            Arm::Retain3PrefixCentered => {
                let (witness, candidates) = query_retain3_centered(
                    fixture,
                    &table,
                    block_size,
                    &mut query_ops,
                    &mut lookup,
                );
                (witness, candidates, AffineBranchCounts::default())
            }
            Arm::Retain3PrefixAffineLeaf => {
                query_retain3_affine_leaf(fixture, &table, block_size, &mut query_ops, &mut lookup)
            }
        };
        let query = started.elapsed();
        assert_eq!(lookup.probes, candidates);
        assert!(super::common::verify_witness(fixture, witness.as_deref()));

        TimedArm {
            certificate: ArmCertificate {
                arm: arm.name().into(),
                retained_arity: arm.retained_arity(),
                retained_states,
                distinct_keys: table.distinct_keys(),
                candidates,
                table_probes: lookup.probes,
                key_comparisons: lookup.comparisons,
                setup_ops: setup_ops.into(),
                query_ops: query_ops.into(),
                payload_bytes: table.payload_bytes(),
                index_bytes: table.index_bytes(),
                container_capacity: table.container_capacity(),
                affine_generic: affine_branches.generic,
                affine_inverse: affine_branches.inverse,
                affine_doubling: affine_branches.doubling,
                affine_identity: affine_branches.identity,
                witness,
            },
            setup,
            query,
        }
    }

    pub(super) fn print_run(round: usize, run: &TimedArm) {
        let total = run.setup + run.query;
        let certificate = &run.certificate;
        println!(
            "{round},{},{:.3},{:.3},{:.3},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            certificate.arm,
            run.setup.as_secs_f64() * 1_000.0,
            run.query.as_secs_f64() * 1_000.0,
            total.as_secs_f64() * 1_000.0,
            certificate.retained_states,
            certificate.distinct_keys,
            certificate.candidates,
            certificate.table_probes,
            certificate.key_comparisons,
            certificate.setup_ops.inversions,
            certificate.query_ops.inversions,
            certificate.setup_ops.multiplications,
            certificate.query_ops.multiplications,
            certificate.setup_ops.squarings,
            certificate.query_ops.squarings,
            certificate.setup_ops.additions,
            certificate.query_ops.additions,
            certificate.setup_ops.doublings,
            certificate.query_ops.doublings,
            certificate.payload_bytes,
            certificate.index_bytes,
            certificate.container_capacity,
            certificate.affine_generic,
            certificate.affine_inverse,
            certificate.affine_doubling,
            certificate.affine_identity,
            usize::from(certificate.witness.is_some()),
        );
    }

    pub(super) fn write_certificate(path: &str, bytes: &[u8]) {
        match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
        {
            Ok(mut output) => output.write_all(bytes).expect("write certificate"),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                assert_eq!(fs::read(path).expect("read retained certificate"), bytes);
            }
            Err(error) => panic!("create certificate {path}: {error}"),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crypto_lib::cryptanalysis::ic_boundary::{
            fp_p31_subexp_mersenne_instance, CountedGroup, GroupOps,
        };
        use crypto_lib::cryptanalysis::ic_framework::pkm_semaev::dickson_factor_base;

        #[test]
        fn packed_witness_and_negation_round_trip() {
            let neg_index: Vec<_> = (0..68).map(|index| index ^ 1).collect();
            let witness = [1u8, 14, 39, 66];
            assert_eq!(
                unpack_witness(pack_witness(&witness), 4),
                witness.map(usize::from)
            );
            assert_eq!(
                negate_indices(negate_indices(witness, &neg_index), &neg_index),
                witness
            );
        }

        #[test]
        fn four_sum_negation_orbit_count_includes_fixed_multisets() {
            let neg_index: Vec<_> = (0..68).map(|index| index ^ 1).collect();
            let mut canonical = 0usize;
            let mut fixed = 0usize;
            for first in 0..68 {
                for second in first..68 {
                    for third in second..68 {
                        for fourth in third..68 {
                            let witness = [first as u8, second as u8, third as u8, fourth as u8];
                            let negated = negate_indices(witness, &neg_index);
                            canonical += usize::from(witness <= negated);
                            fixed += usize::from(witness == negated);
                        }
                    }
                }
            }
            assert_eq!(fixed, 595);
            assert_eq!(canonical, RETAINED_FOUR_STATES);
        }

        #[test]
        fn compact_entry_is_twelve_bytes() {
            assert_eq!(size_of::<CompactEntry>(), 12);
            assert_eq!(size_of::<CompactValue>(), 8);
        }

        #[test]
        fn affine_batch_covers_generic_inverse_double_and_identity() {
            let instance = fp_p31_subexp_mersenne_instance();
            let factor_base = dickson_factor_base(&instance, 6, 0, 0).unwrap();
            let first = factor_base.points[0];
            let generic = factor_base
                .points
                .iter()
                .copied()
                .find(|point| point.x != first.x)
                .unwrap();
            let inverse = factor_base.points[factor_base.neg_index[0]];
            let fixture = Fixture {
                target: instance.curve.identity(),
                instance,
                base: factor_base.points,
                neg_index: factor_base.neg_index,
                abscissae: factor_base.abscissae,
                planted_indices: Vec::new(),
            };
            let block = [
                (first, generic, 0),
                (first, inverse, 1),
                (first, first, 2),
                (PrimePoint::INFINITY, generic, 3),
            ];
            let mut ops = OpCounts::default();
            let mut branches = AffineBranchCounts::default();
            let output = batch_affine_add(&block, &fixture, &mut ops, &mut branches);
            for ((left, right, witness), (actual, actual_witness)) in block.into_iter().zip(output)
            {
                assert_eq!(actual_witness, witness);
                assert_eq!(
                    actual,
                    fixture
                        .instance
                        .curve
                        .add(&mut GroupOps::default(), left, right)
                );
            }
            assert_eq!(
                branches,
                AffineBranchCounts {
                    generic: 1,
                    inverse: 1,
                    doubling: 1,
                    identity: 1,
                }
            );
        }
    }
}

use common::{certificate_bundle, load_fixture, read_manifest};
use engine::{print_run, time_arm, write_certificate, Arm};

fn main() {
    let arguments: Vec<_> = std::env::args().collect();
    assert_eq!(
        arguments.len(),
        5,
        "usage: norm_one_torus_m7 --manifest PATH --certificates PATH"
    );
    assert_eq!(arguments[1], "--manifest");
    assert_eq!(arguments[3], "--certificates");

    let (manifest, manifest_bytes) = read_manifest(&arguments[2]);
    let fixture = load_fixture(&manifest);
    let configuration = &manifest.configuration;

    println!(
        "round,implementation,setup_ms,query_ms,total_ms,retained_states,distinct_keys,candidates,table_probes,key_comparisons,setup_inversions,query_inversions,setup_multiplications,query_multiplications,setup_squarings,query_squarings,setup_additions,query_additions,setup_doublings,query_doublings,payload_bytes,index_bytes,container_capacity,affine_generic,affine_inverse,affine_doubling,affine_identity,success"
    );

    let mut certificates: Vec<Option<common::ArmCertificate>> = vec![None; Arm::ALL.len()];
    for round in 0..configuration.rounds {
        for offset in 0..Arm::ALL.len() {
            let arm_index = (round + offset) % Arm::ALL.len();
            let run = time_arm(
                &fixture,
                Arm::ALL[arm_index],
                configuration.query_block_size,
            );
            if let Some(reference) = &certificates[arm_index] {
                assert_eq!(&run.certificate, reference);
            } else {
                certificates[arm_index] = Some(run.certificate.clone());
            }
            print_run(round, &run);
        }
    }

    let certificates = certificates
        .into_iter()
        .map(|certificate| certificate.expect("each arm ran"))
        .collect();
    let bundle = certificate_bundle(&fixture, &manifest_bytes, certificates);
    let bytes = serde_json::to_vec_pretty(&bundle).expect("serialize certificates");
    write_certificate(&arguments[4], &bytes);
}
