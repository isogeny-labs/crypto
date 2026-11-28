//! Exact, bounded scalar motifs from Section 3 of the September 23, 2026
//! manuscript *Integer multiplication below n log n*.
//!
//! [`BitMotif`] executes one coordinate invocation of the eight-row XOR
//! schedule. A `u64` carries 64 independent copies of a bit wire. It checks the
//! scalar identity and scratch restoration, not the paper's full tensor
//! network, frame changes, tape schedule, or multiplication algorithm.
//!
//! [`PublishedAudit`] recomputes the published `h = 100` counts using checked
//! integer arithmetic, without materializing any network. The rank formulas
//! are transcribed from the manuscript; recomputing their arithmetic is not
//! an independent proof of the subspace labels or the recursive cost bound.
//!
//! Source: OpenAI, Section 3, `03-motifs.tex`, Git blob
//! `cdb0ba527aa8df5d57c151183fd3893c1a7e21ab`:
//! <https://github.com/openai/math/blob/main/preprints/Integer-multiplication-below-n-log-n-September-23-2026/build/sections/03-motifs.tex>.
//! This implementation follows the mathematical schedule directly; it does
//! not copy the independent `Klakocik/below-nlogn` implementation.

use std::fmt;

/// Largest ground set accepted by the executable scalar reference.
pub const MAX_EXECUTABLE_H: usize = 16;

/// Conservative storage budget, in 64-bit words, for indices, two data banks,
/// and three scratch buffers. The constructor enforces this before allocation.
pub const MAX_MATERIALIZED_WORDS: usize = 1 << 20;

/// Errors are detected before an invocation changes its target or scratch.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MotifError {
    GroundSetOutOfRange {
        requested: usize,
        maximum: usize,
    },
    WordBudgetExceeded {
        required: usize,
        available: usize,
    },
    LengthMismatch {
        buffer: &'static str,
        expected: usize,
        actual: usize,
    },
    CountOverflow(&'static str),
}

impl fmt::Display for MotifError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GroundSetOutOfRange { requested, maximum } => write!(
                f,
                "scalar motif ground set {requested} is outside 3..={maximum}"
            ),
            Self::WordBudgetExceeded {
                required,
                available,
            } => write!(
                f,
                "scalar motif needs {required} materialized words; budget is {available}"
            ),
            Self::LengthMismatch {
                buffer,
                expected,
                actual,
            } => write!(f, "{buffer} has {actual} words; expected {expected}"),
            Self::CountOverflow(quantity) => write!(f, "integer overflow computing {quantity}"),
        }
    }
}

impl std::error::Error for MotifError {}

/// Allocation-free shape of a *single* coordinate invocation. Its banks have
/// `C(h, 3)` words, not the `C(h, 3)^3` roles of the full published network.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BitMotifShape {
    pub h: usize,
    pub bank_words: usize,
    pub neighbors_per_triple: usize,
    pub side_words: usize,
    pub center_words: usize,
    pub materialized_words: usize,
}

impl BitMotifShape {
    pub fn checked(h: usize) -> Result<Self, MotifError> {
        if !(3..=MAX_EXECUTABLE_H).contains(&h) {
            return Err(MotifError::GroundSetOutOfRange {
                requested: h,
                maximum: MAX_EXECUTABLE_H,
            });
        }
        let bank_words = usize::try_from(choose_three(h as u128)?)
            .map_err(|_| MotifError::CountOverflow("bank words"))?;
        let neighbors_per_triple =
            usize::try_from(mul(3, choose_two((h - 3) as u128)?, "neighbors")?)
                .map_err(|_| MotifError::CountOverflow("neighbors"))?;
        let side_words = bank_words
            .checked_mul(neighbors_per_triple)
            .ok_or(MotifError::CountOverflow("side words"))?;
        // Triple indices: 3v; pair indices: 2s; two banks: 2v;
        // three independent scratch buffers: 3(s+h). Count each usize as a
        // full u64 word, also conservatively covering 32-bit targets.
        let materialized_words = bank_words
            .checked_mul(5)
            .and_then(|x| side_words.checked_mul(5).and_then(|s| x.checked_add(s)))
            .and_then(|x| h.checked_mul(3).and_then(|c| x.checked_add(c)))
            .ok_or(MotifError::CountOverflow("materialized words"))?;
        Ok(Self {
            h,
            bank_words,
            neighbors_per_triple,
            side_words,
            center_words: h,
            materialized_words,
        })
    }
}

/// Arbitrary initial side and central wire values are supported and restored.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BitScratch {
    pub side: Vec<u64>,
    pub center: Vec<u64>,
}

/// Executable scalar identity, with no floating-point arithmetic and no hidden
/// conventional multiplication backend.
#[derive(Clone, Debug)]
pub struct BitMotif {
    shape: BitMotifShape,
    triples: Vec<[usize; 3]>,
    /// Ordered `(target, source)` pairs with intersection cardinality one.
    neighbors: Vec<(usize, usize)>,
}

impl BitMotif {
    pub fn new(h: usize) -> Result<Self, MotifError> {
        Self::with_word_limit(h, MAX_MATERIALIZED_WORDS)
    }

    /// Set a smaller storage budget. Supplying a larger value never overrides
    /// [`MAX_MATERIALIZED_WORDS`]. This budget includes three scratch buffers
    /// and two banks even though the constructor allocates only the indices.
    pub fn with_word_limit(h: usize, word_limit: usize) -> Result<Self, MotifError> {
        let shape = BitMotifShape::checked(h)?;
        let available = word_limit.min(MAX_MATERIALIZED_WORDS);
        if shape.materialized_words > available {
            return Err(MotifError::WordBudgetExceeded {
                required: shape.materialized_words,
                available,
            });
        }
        let mut triples = Vec::with_capacity(shape.bank_words);
        for a in 0..h {
            for b in a + 1..h {
                for c in b + 1..h {
                    triples.push([a, b, c]);
                }
            }
        }
        let mut neighbors = Vec::with_capacity(shape.side_words);
        for (s, left) in triples.iter().enumerate() {
            for (t, right) in triples.iter().enumerate() {
                if left.iter().filter(|i| right.contains(i)).count() == 1 {
                    neighbors.push((s, t));
                }
            }
        }
        debug_assert_eq!(neighbors.len(), shape.side_words);
        Ok(Self {
            shape,
            triples,
            neighbors,
        })
    }

    pub fn shape(&self) -> BitMotifShape {
        self.shape
    }

    pub fn bank_len(&self) -> usize {
        self.shape.bank_words
    }

    pub fn side_len(&self) -> usize {
        self.shape.side_words
    }

    pub fn center_len(&self) -> usize {
        self.shape.center_words
    }

    pub fn zero_scratch(&self) -> BitScratch {
        BitScratch {
            side: vec![0; self.side_len()],
            center: vec![0; self.center_len()],
        }
    }

    /// Execute rows 0 through 7. The specified result is `target ^= source`,
    /// with all scratch values restored, for all bit patterns in each word.
    pub fn apply_forward(
        &self,
        source: &[u64],
        target: &mut [u64],
        scratch: &mut BitScratch,
    ) -> Result<(), MotifError> {
        self.validate(source, target, scratch)?;
        self.run(source, target, scratch, false);
        Ok(())
    }

    /// Execute the inverse schedule (rows 7 through 0). In characteristic two
    /// the final scalar map is again `target ^= source`; intermediate updates
    /// use the actual reversed schedule.
    pub fn apply_inverse(
        &self,
        source: &[u64],
        target: &mut [u64],
        scratch: &mut BitScratch,
    ) -> Result<(), MotifError> {
        self.validate(source, target, scratch)?;
        self.run(source, target, scratch, true);
        Ok(())
    }

    /// Three local invocations exchange two `C(h,3)`-word banks and restore
    /// three independent scratch buffers. This is the scalar exchange
    /// identity, not the paper's three tensor-coordinate stages on `v^3`
    /// data roles, and does not claim their rank-saving cost.
    pub fn exchange_banks(
        &self,
        x: &mut [u64],
        y: &mut [u64],
        scratch: &mut [BitScratch; 3],
    ) -> Result<(), MotifError> {
        // Validate every stage before any stage mutates data.
        for state in scratch.iter() {
            self.validate(x, y, state)?;
        }
        self.run(x, y, &mut scratch[0], false);
        self.run(y, x, &mut scratch[1], true);
        self.run(x, y, &mut scratch[2], false);
        Ok(())
    }

    fn validate(
        &self,
        source: &[u64],
        target: &[u64],
        scratch: &BitScratch,
    ) -> Result<(), MotifError> {
        for (buffer, actual, expected) in [
            ("source", source.len(), self.bank_len()),
            ("target", target.len(), self.bank_len()),
            ("side scratch", scratch.side.len(), self.side_len()),
            ("central scratch", scratch.center.len(), self.center_len()),
        ] {
            if actual != expected {
                return Err(MotifError::LengthMismatch {
                    buffer,
                    expected,
                    actual,
                });
            }
        }
        Ok(())
    }

    fn run(&self, source: &[u64], target: &mut [u64], scratch: &mut BitScratch, inverse: bool) {
        for step in 0..8 {
            let row = if inverse { 7 - step } else { step };
            match row {
                // Subtraction and addition coincide over F_2.
                0 | 5 => {
                    for (i, &(s, _)) in self.neighbors.iter().enumerate() {
                        target[s] ^= scratch.side[i];
                    }
                }
                1 | 4 => {
                    for (s, triple) in self.triples.iter().enumerate() {
                        for &i in triple {
                            target[s] ^= scratch.center[i];
                        }
                    }
                }
                2 | 7 => {
                    for (i, &(_, t)) in self.neighbors.iter().enumerate() {
                        scratch.side[i] ^= source[t];
                    }
                }
                3 | 6 => {
                    for (t, triple) in self.triples.iter().enumerate() {
                        for &i in triple {
                            scratch.center[i] ^= source[t];
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

/// An exact, reduced nonnegative ratio. No floating-point approximation is
/// used to check a small rank deficit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExactRatio {
    pub numerator: u128,
    pub denominator: u128,
}

impl ExactRatio {
    fn reduced(numerator: u128, denominator: u128) -> Self {
        let mut a = numerator;
        let mut b = denominator;
        while b != 0 {
            (a, b) = (b, a % b);
        }
        Self {
            numerator: numerator / a,
            denominator: denominator / a,
        }
    }
}

/// Formula-level counts for the entire published `h=100` network. These are
/// distinct from the materialized local [`BitMotif`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NetworkAudit {
    pub h: u128,
    pub triples: u128,
    pub tensor_bank_roles: u128,
    pub ambient_dimension: u128,
    pub invocations: u128,
    pub neighbors_per_triple: u128,
    pub centers_per_invocation: u128,
    pub side_wires_per_invocation: u128,
    pub wires: u128,
    pub decreasing_dimension_loss: u128,
    /// The bit/rational interface adds `N` source ranks; the complex/binary
    /// interface adds zero. Omitting this changes the published bit bound.
    pub source_rank_correction: u128,
    pub edge_rank_sum: u128,
    pub baseline_rank_sum: u128,
    pub rank_deficit: u128,
    pub relative_rank_deficit: ExactRatio,
    pub loss_per_bank_role: ExactRatio,
    /// Arithmetic check of `s < W*m`, conditional on the paper's rank formula.
    pub strict_rank_saving: bool,
    /// Exact sufficient inequality `eta > 20/2^50`, together with `m < 2^20`.
    /// The paper uses these to infer `s/W < m^(1-2^-50)`; this flag does not
    /// verify the other linear work or the later amplification overhead.
    pub published_exponent_inequality: bool,
}

impl NetworkAudit {
    /// Check only storage for one scalar word per role. Even a true return
    /// would not establish feasibility of the array/tensor construction.
    pub fn scalar_roles_fit(&self, word_budget: u128) -> bool {
        self.wires <= word_budget
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PublishedAudit {
    pub bit: NetworkAudit,
    pub complex: NetworkAudit,
    pub recurrence_exponent: ExactRatio,
}

impl PublishedAudit {
    /// Recompute the `h=100` count formulas in constant storage using `u128`.
    pub fn compute() -> Result<Self, MotifError> {
        let h = 100;
        let bit_neighbors = mul(3, choose_two(h - 3)?, "bit neighbors")?;
        let complex_neighbors = add(
            choose_three(h - 3)?,
            mul(3, h - 3, "complex neighbors")?,
            "complex neighbors",
        )?;
        let denominator = 1u128 << 50;
        Ok(Self {
            bit: published_network(h, bit_neighbors, h, true)?,
            complex: published_network(h, complex_neighbors, h + 1, false)?,
            recurrence_exponent: ExactRatio {
                numerator: denominator - 1,
                denominator,
            },
        })
    }
}

fn published_network(
    h: u128,
    neighbors: u128,
    centers: u128,
    bit: bool,
) -> Result<NetworkAudit, MotifError> {
    let triples = choose_three(h)?;
    let triple_square = mul(triples, triples, "v^2")?;
    let n = mul(triple_square, triples, "N")?;
    let m = mul(mul(h, h, "h^2")?, h, "m")?;
    let invocations = mul(3, triple_square, "I")?;
    let side = mul(triples, neighbors, "side wires")?;
    let scratch = mul(
        invocations,
        add(side, centers, "invocation scratch")?,
        "total scratch",
    )?;
    let two_n = mul(2, n, "2N")?;
    let wires = add(two_n, scratch, "W")?;
    let loss = mul(mul(invocations, centers, "Ic")?, h, "L")?;
    let baseline = mul(wires, m, "Wm")?;
    let source_rank_correction = if bit { n } else { 0 };
    let edge_rank_sum = add(
        add(
            baseline
                .checked_sub(two_n)
                .ok_or(MotifError::CountOverflow("Wm-2N"))?,
            mul(2, loss, "2L")?,
            "edge ranks",
        )?,
        source_rank_correction,
        "source-corrected edge ranks",
    )?;
    let rank_deficit = baseline
        .checked_sub(edge_rank_sum)
        .ok_or(MotifError::CountOverflow("rank deficit"))?;
    let exponent_gate = m < (1u128 << 20)
        && mul(rank_deficit, 1u128 << 50, "exponent inequality numerator")?
            > mul(20, baseline, "exponent inequality denominator")?;
    Ok(NetworkAudit {
        h,
        triples,
        tensor_bank_roles: n,
        ambient_dimension: m,
        invocations,
        neighbors_per_triple: neighbors,
        centers_per_invocation: centers,
        side_wires_per_invocation: side,
        wires,
        decreasing_dimension_loss: loss,
        source_rank_correction,
        edge_rank_sum,
        baseline_rank_sum: baseline,
        rank_deficit,
        relative_rank_deficit: ExactRatio::reduced(rank_deficit, baseline),
        loss_per_bank_role: ExactRatio::reduced(loss, n),
        strict_rank_saving: edge_rank_sum < baseline,
        published_exponent_inequality: exponent_gate,
    })
}

fn add(a: u128, b: u128, name: &'static str) -> Result<u128, MotifError> {
    a.checked_add(b).ok_or(MotifError::CountOverflow(name))
}

fn mul(a: u128, b: u128, name: &'static str) -> Result<u128, MotifError> {
    a.checked_mul(b).ok_or(MotifError::CountOverflow(name))
}

fn choose_two(n: u128) -> Result<u128, MotifError> {
    if n < 2 {
        return Ok(0);
    }
    Ok(mul(n, n - 1, "binomial(n,2)")? / 2)
}

fn choose_three(n: u128) -> Result<u128, MotifError> {
    if n < 3 {
        return Ok(0);
    }
    Ok(mul(mul(n, n - 1, "binomial(n,3)")?, n - 2, "binomial(n,3)")? / 6)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn patterned(len: usize, salt: u64) -> Vec<u64> {
        (0..len)
            .map(|i| {
                (i as u64)
                    .wrapping_mul(0x9e3779b97f4a7c15)
                    .rotate_left((i % 64) as u32)
                    ^ salt
            })
            .collect()
    }

    fn scratch(motif: &BitMotif, salt: u64) -> BitScratch {
        BitScratch {
            side: patterned(motif.side_len(), salt),
            center: patterned(motif.center_len(), !salt),
        }
    }

    #[test]
    fn forward_and_inverse_match_direct_xor_and_restore_arbitrary_scratch() {
        for h in 3..=10 {
            let motif = BitMotif::new(h).unwrap();
            let source = patterned(motif.bank_len(), 0x0123456789abcdef);
            let initial = patterned(motif.bank_len(), 0xfedcba9876543210);
            let expected: Vec<_> = initial.iter().zip(&source).map(|(a, b)| a ^ b).collect();
            for inverse in [false, true] {
                let mut target = initial.clone();
                let mut state = scratch(&motif, 0xa5a5a5a5a5a5a5a5);
                let saved = state.clone();
                if inverse {
                    motif
                        .apply_inverse(&source, &mut target, &mut state)
                        .unwrap();
                } else {
                    motif
                        .apply_forward(&source, &mut target, &mut state)
                        .unwrap();
                }
                assert_eq!(target, expected, "h={h}, inverse={inverse}");
                assert_eq!(state, saved, "h={h}, inverse={inverse}");
            }
        }
    }

    #[test]
    fn basis_vectors_check_every_source_target_coefficient() {
        let motif = BitMotif::new(6).unwrap();
        for position in 0..motif.bank_len() {
            let mut source = vec![0; motif.bank_len()];
            source[position] = u64::MAX;
            let mut target = vec![0; motif.bank_len()];
            let mut state = scratch(&motif, position as u64);
            let saved = state.clone();
            motif
                .apply_forward(&source, &mut target, &mut state)
                .unwrap();
            assert_eq!(target, source);
            assert_eq!(state, saved);
        }
    }

    #[test]
    fn three_local_schedules_exchange_banks_and_restore_all_scratch() {
        let motif = BitMotif::new(6).unwrap();
        let mut x = patterned(motif.bank_len(), 11);
        let mut y = patterned(motif.bank_len(), 29);
        let old_x = x.clone();
        let old_y = y.clone();
        let mut states = [scratch(&motif, 1), scratch(&motif, 2), scratch(&motif, 3)];
        let saved = states.clone();
        motif.exchange_banks(&mut x, &mut y, &mut states).unwrap();
        assert_eq!(x, old_y);
        assert_eq!(y, old_x);
        assert_eq!(states, saved);
    }

    #[test]
    fn bad_lengths_are_rejected_before_any_mutation() {
        let motif = BitMotif::new(5).unwrap();
        let mut x = patterned(motif.bank_len(), 11);
        let mut y = patterned(motif.bank_len(), 29);
        let mut states = [scratch(&motif, 1), scratch(&motif, 2), scratch(&motif, 3)];
        states[2].center.pop();
        let original = (x.clone(), y.clone(), states.clone());
        assert!(matches!(
            motif.exchange_banks(&mut x, &mut y, &mut states),
            Err(MotifError::LengthMismatch { .. })
        ));
        assert_eq!((x, y, states), original);

        let source = vec![0; motif.bank_len() - 1];
        let mut target = patterned(motif.bank_len(), 77);
        let mut state = scratch(&motif, 89);
        let original = (target.clone(), state.clone());
        assert!(motif
            .apply_forward(&source, &mut target, &mut state)
            .is_err());
        assert_eq!((target, state), original);
    }

    #[test]
    fn preflight_bounds_ground_set_and_materialized_storage() {
        assert!(BitMotif::new(2).is_err());
        assert!(BitMotif::new(100).is_err());
        assert!(BitMotif::new(usize::MAX).is_err());
        let shape = BitMotifShape::checked(6).unwrap();
        assert!(matches!(
            BitMotif::with_word_limit(6, shape.materialized_words - 1),
            Err(MotifError::WordBudgetExceeded { .. })
        ));
        assert!(BitMotif::with_word_limit(6, shape.materialized_words).is_ok());
        let largest = BitMotifShape::checked(MAX_EXECUTABLE_H).unwrap();
        assert!(largest.materialized_words <= MAX_MATERIALIZED_WORDS);
    }

    #[test]
    fn published_counts_include_source_rank_correction() {
        let audit = PublishedAudit::compute().unwrap();
        let bit = audit.bit;
        let complex = audit.complex;
        assert_eq!(bit.triples, 161_700);
        assert_eq!(bit.tensor_bank_roles, 4_227_952_113_000_000);
        assert_eq!(bit.ambient_dimension, 1_000_000);
        assert_eq!(bit.invocations, 78_440_670_000);
        assert_eq!(bit.neighbors_per_triple, 13_968);
        assert_eq!(complex.neighbors_per_triple, 147_731);
        assert_eq!(bit.wires, 177_176_569_091_445_000_000);
        assert_eq!(complex.wires, 1_873_807_244_643_542_670_000);
        assert_eq!(bit.edge_rank_sum, 177_176_569_088_785_861_287_000_000);
        assert_eq!(complex.edge_rank_sum, 1_873_807_244_636_671_267_308_000_000);
        assert_eq!(bit.source_rank_correction, bit.tensor_bank_roles);
        assert_eq!(complex.source_rank_correction, 0);
        assert_eq!(
            bit.loss_per_bank_role,
            ExactRatio {
                numerator: 100,
                denominator: 539
            }
        );
        assert_eq!(
            complex.loss_per_bank_role,
            ExactRatio {
                numerator: 101,
                denominator: 539
            }
        );
        assert_eq!(
            bit.relative_rank_deficit,
            ExactRatio {
                numerator: 339,
                denominator: 22_587_335_000_000
            }
        );
        assert_eq!(
            complex.relative_rank_deficit,
            ExactRatio {
                numerator: 73,
                denominator: 19_906_842_167_500
            }
        );
        assert!(bit.strict_rank_saving && complex.strict_rank_saving);
        assert!(bit.published_exponent_inequality && complex.published_exponent_inequality);
        assert!(!bit.scalar_roles_fit(MAX_MATERIALIZED_WORDS as u128));
        assert!(!complex.scalar_roles_fit(MAX_MATERIALIZED_WORDS as u128));
    }
}
