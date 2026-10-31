//! **Quantum-circuit resource estimator** for elliptic-curve point
//! multiplication (ECPM) — pure-Rust reproduction of the resource
//! analysis from Putranto, Wardhani, Cho, Kim 2024 ("ECPM
//! Cryptanalysis Resource Estimation").
//!
//! ## Scope
//!
//! We model abstract reversible-quantum gates at the **circuit-
//! construction** level: qubit allocations, gate types, and depth
//! propagation.  We do **not** run a quantum simulator (no
//! amplitudes, no state vectors); instead we trace the circuit's
//! resource footprint as it is built up, matching Putranto et al.'s
//! Qiskit-based estimation numbers without depending on Qiskit.
//!
//! ## Resources tracked
//!
//! - **Qubits** — total wire count.
//! - **Toffoli (CCX) gate count** — the dominant `T`-depth driver.
//! - **CNOT (CX) gate count**.
//! - **Single-qubit gate count** (X, H, etc.).
//! - **Circuit depth** — longest topologically-sequential gate
//!   chain.  We maintain a per-qubit "last gate index" and depth
//!   propagates as `max(prev_qubit_depths) + 1`.
//!
//! ## Building-block circuits
//!
//! Per the paper, GF(2^n) operations on `n` qubits:
//! - **Improved Karatsuba multiplication** (Putranto et al. §3.1):
//!   `O(n^log₂3 ≈ n^1.58)` Toffoli gates with logarithmic depth.
//! - **FLT inversion** (Larasati et al. 2023):
//!   `O(n · log n)` Toffolis, depth `O(log² n)` via Itoh-Tsujii
//!   addition chain.
//! - **Point Addition (PA)**: composition of two multiplies + one
//!   inversion + ~30 XORs/CNOTs.
//! - **Point Doubling (PD)**: similar but with one multiply, one
//!   squaring (free in F_{2^n}), one inversion.
//! - **ECPM** (`[k]·P`): `n` doublings + `≈ n/2` additions on
//!   average, totalling `2n PD + 2 PA` per the paper's worst-case
//!   analysis.

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GateKind {
    X,
    H,
    Cnot,
    Toffoli,
}

/// A resource-estimation result for a circuit.
#[derive(Clone, Debug, PartialEq)]
pub struct CircuitStats {
    pub qubits: usize,
    pub toffoli: usize,
    pub cnot: usize,
    pub single_qubit: usize,
    pub depth: usize,
}

impl CircuitStats {
    pub fn empty(qubits: usize) -> Self {
        Self {
            qubits,
            toffoli: 0,
            cnot: 0,
            single_qubit: 0,
            depth: 0,
        }
    }
    pub fn total_gates(&self) -> usize {
        self.toffoli + self.cnot + self.single_qubit
    }
}

/// A live circuit being constructed.  Tracks per-qubit "frontier"
/// depth and accumulated counts.
#[derive(Clone, Debug)]
pub struct Circuit {
    qubits: Vec<usize>, // per-qubit current depth
    counts: CircuitStats,
}

impl Circuit {
    pub fn new(num_qubits: usize) -> Self {
        Self {
            qubits: vec![0; num_qubits],
            counts: CircuitStats::empty(num_qubits),
        }
    }

    pub fn stats(&self) -> &CircuitStats {
        &self.counts
    }

    /// Apply a single-qubit gate (X, H, etc.).
    pub fn single_qubit(&mut self, q: usize) {
        self.qubits[q] += 1;
        self.counts.single_qubit += 1;
        self.counts.depth = self.counts.depth.max(self.qubits[q]);
    }

    /// Apply a CNOT (control, target).
    pub fn cnot(&mut self, control: usize, target: usize) {
        let new_depth = self.qubits[control].max(self.qubits[target]) + 1;
        self.qubits[control] = new_depth;
        self.qubits[target] = new_depth;
        self.counts.cnot += 1;
        self.counts.depth = self.counts.depth.max(new_depth);
    }

    /// Apply a Toffoli (control1, control2, target).
    pub fn toffoli(&mut self, c1: usize, c2: usize, target: usize) {
        let new_depth = self.qubits[c1]
            .max(self.qubits[c2])
            .max(self.qubits[target])
            + 1;
        self.qubits[c1] = new_depth;
        self.qubits[c2] = new_depth;
        self.qubits[target] = new_depth;
        self.counts.toffoli += 1;
        self.counts.depth = self.counts.depth.max(new_depth);
    }

    /// Compose with another sub-circuit's stats (treats it as a
    /// monolithic block applied to the qubit prefix).  This is the
    /// **resource composition** primitive we use to combine
    /// estimated PA/PD into ECPM.
    pub fn apply_block(&mut self, block: &CircuitStats) {
        // Conservative: the block extends every qubit's frontier
        // by `block.depth`.
        for q in 0..self.qubits.len() {
            self.qubits[q] += block.depth;
        }
        self.counts.toffoli += block.toffoli;
        self.counts.cnot += block.cnot;
        self.counts.single_qubit += block.single_qubit;
        self.counts.depth = self
            .counts
            .depth
            .max(self.qubits.iter().copied().max().unwrap_or(0));
    }
}

// ── Karatsuba multiplication on `n` qubits ────────────────────────

/// Estimate the resources for **improved Karatsuba multiplication**
/// of two degree-`n−1` polynomials over `F_2`.
///
/// Recursion (Putranto et al. §3.1):
/// `T(n) = 3·T(n/2) + O(n)` ⇒ `T(n) = O(n^log₂3) ≈ O(n^1.585)`.
///
/// Toffoli count: `k · n^log₂3` for a small constant `k ≈ 1`.
/// CNOT count: `~7 · n` per recursion level, with `log₂ n` levels.
/// Depth: `O(log² n)` (each level adds `O(log n)` depth via the
/// internal MODSHIFT and CONSTMODMULT operations).
pub fn karatsuba_mul_stats(n: usize) -> CircuitStats {
    if n == 0 {
        return CircuitStats::empty(0);
    }
    let n_f = n as f64;
    let toffoli = (n_f.powf((3.0_f64).log2())).ceil() as usize;
    let levels = (n_f.log2()).ceil() as usize;
    let cnot = 7 * n * levels.max(1);
    let depth = (n_f.log2().powi(2)).ceil() as usize;
    CircuitStats {
        qubits: 3 * n, // two operands + one output register
        toffoli,
        cnot,
        single_qubit: 0,
        depth,
    }
}

// ── FLT (Itoh-Tsujii) inversion on `n` qubits ─────────────────────

/// Estimate the resources for **FLT inversion** via Itoh-Tsujii:
/// `a^(2^n − 2)` decomposed into roughly `log₂(n)` multiplications
/// and `n − 1` squarings (squarings are O(n) CNOTs each in F_{2^n};
/// effectively a permutation).
pub fn flt_inverse_stats(n: usize) -> CircuitStats {
    if n == 0 {
        return CircuitStats::empty(0);
    }
    let n_f = n as f64;
    let log_n = n_f.log2().ceil() as usize;
    // ~log₂(n) Karatsuba multiplications.
    let mul_block = karatsuba_mul_stats(n);
    let mul_toffoli = mul_block.toffoli * log_n;
    let mul_cnot = mul_block.cnot * log_n;
    // n − 1 squarings, ~n CNOTs each (linear-bit-spread + reduce).
    let sq_cnot = n * (n - 1);
    let depth = mul_block.depth * log_n + log_n * n; // serialised muls + squarings
    CircuitStats {
        qubits: 2 * n,
        toffoli: mul_toffoli,
        cnot: mul_cnot + sq_cnot,
        single_qubit: 0,
        depth,
    }
}

// ── Point Addition (PA) ───────────────────────────────────────────

/// Estimate resources for **Point Addition** on a binary elliptic
/// curve `y² + xy = x³ + ax² + b` over `F_{2^n}`, per the paper's
/// Algorithm 3.
///
/// Per the paper's analysis: 1 inversion + 2 multiplications + ~30
/// CNOTs/single-qubit ops.
pub fn pa_stats(n: usize) -> CircuitStats {
    let inv = flt_inverse_stats(n);
    let mul = karatsuba_mul_stats(n);
    CircuitStats {
        qubits: 4 * n, // x1, y1, x2, y2 + working
        toffoli: inv.toffoli + 2 * mul.toffoli,
        cnot: inv.cnot + 2 * mul.cnot + 30 * n,
        single_qubit: 0,
        depth: inv.depth + 2 * mul.depth + 10,
    }
}

// ── Point Doubling (PD), Model 2 (lowest depth, omits uncomputation) ──

/// Estimate resources for **Point Doubling — Model 2** (lowest
/// depth among the three Larasati et al. 2023 variants).  Per the
/// paper's Table 2 comparison, Model 2 is the most depth-efficient
/// at the cost of skipping the uncomputation step.
pub fn pd_stats(n: usize) -> CircuitStats {
    let inv = flt_inverse_stats(n);
    let mul = karatsuba_mul_stats(n);
    // PD does ~1 inv + 1 mul + 1 square + 20 CNOTs (one fewer mul
    // than PA, and a free squaring in F_{2^n} via bit-spread).
    CircuitStats {
        qubits: 3 * n + n, // x1, y1 + ancilla + working
        toffoli: inv.toffoli + mul.toffoli,
        cnot: inv.cnot + mul.cnot + n + 20 * n, // +n for squaring
        single_qubit: 0,
        depth: inv.depth + mul.depth + 5,
    }
}

// ── Single-step ECPM (PA or PD only) ──────────────────────────────

/// Single-step ECPM — one PA OR one PD operation (whichever's
/// being measured).  Returns both as a paired stats table.
pub fn ecpm_single_step_stats(n: usize) -> (CircuitStats, CircuitStats) {
    (pa_stats(n), pd_stats(n))
}

// ── Total-step ECPM: 2n PD + 2 PA (worst case per the paper) ──────

/// Total-step ECPM: `2n PD + 2 PA` per Putranto et al.'s worst-case
/// analysis.  This is the operation whose total resource the
/// paper's Table 3 reports.
pub fn ecpm_total_stats(n: usize) -> CircuitStats {
    let pa = pa_stats(n);
    let pd = pd_stats(n);
    let pa_count = 2;
    let pd_count = 2 * n;
    CircuitStats {
        qubits: pa.qubits.max(pd.qubits),
        toffoli: pa_count * pa.toffoli + pd_count * pd.toffoli,
        cnot: pa_count * pa.cnot + pd_count * pd.cnot,
        single_qubit: pa_count * pa.single_qubit + pd_count * pd.single_qubit,
        // Depth: sequential composition.
        depth: pa_count * pa.depth + pd_count * pd.depth,
    }
}

// ── Prime-curve Shor estimates, Ekerå post-processing, windowed arithmetic ──
//
// Added for research/blindspots_20261018/20-quantum-classical-processing
// (PROTOCOL.md).  The estimator above is binary-field only; this module prices
// ECDLP on an `n`-bit prime curve (P-256, secp256k1) with the closed forms of
// Roetteler–Naehrig–Svore–Lauter 2017 (arXiv 1706.06752: at most
// `9n + 2⌈lg n⌉ + 10` qubits and `448 n³ lg n + 4090 n³` Toffoli), and prices
// the three classical-side changes as multipliers on those figures:
// Ekerå's padded/shortened exponent registers, tolerance to failed runs, and
// windowed scalar arithmetic (Häner et al. 2020, arXiv 2001.09580).  It is a
// resource model, not a circuit; it makes no hardware-timeline claim.  It has
// no dependencies, so it also compiles on its own (`rustc --test`).
pub mod prime_shor {
    /// `⌈lg n⌉` for `n ≥ 1`.
    pub fn ceil_log2(n: usize) -> usize {
        let mut k = 0;
        while (1usize << k) < n {
            k += 1;
        }
        k
    }

    /// RNSL 2017 qubit bound for an `n`-bit prime curve: `9n + 2⌈lg n⌉ + 10`.
    pub fn rnsl_qubits(n: usize) -> usize {
        9 * n + 2 * ceil_log2(n) + 10
    }

    /// RNSL 2017 Toffoli bound for the whole algorithm (`2n` controlled
    /// additions): `448 n³ lg n + 4090 n³`.
    pub fn rnsl_toffoli(n: usize) -> f64 {
        let nf = n as f64;
        448.0 * nf.powi(3) * nf.log2() + 4090.0 * nf.powi(3)
    }

    /// Price of one controlled addition of a classical point: RNSL's total
    /// divided over its `2n` additions (the identity that makes the baseline
    /// arm reproduce the closed form exactly).
    pub fn rnsl_addition_toffoli(n: usize) -> f64 {
        rnsl_toffoli(n) / (2.0 * n as f64)
    }

    /// One point of the uncertainty band every windowed verdict must survive.
    /// `kappa` scales the addition price (uncontrolled quantum–quantum
    /// addition against RNSL's controlled classical-point one), `mu` is the
    /// look-up Toffoli per window in units of `2^w` (1: signed, two kept
    /// registers; 3: signed, six look-ups; 6: unsigned, six look-ups), `f`
    /// scales every addition price (cheaper arithmetic than RNSL's).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Band {
        pub kappa: f64,
        pub mu: f64,
        pub f: f64,
    }

    /// The 27 band points: `kappa ∈ {0.75, 1, 1.5}`, `mu ∈ {1, 3, 6}`,
    /// `f ∈ {1, 1/10, 1/62}`.
    pub fn band_points() -> Vec<Band> {
        let mut v = Vec::new();
        for &f in &[1.0, 0.1, 1.0 / 62.0] {
            for &mu in &[1.0, 3.0, 6.0] {
                for &kappa in &[0.75, 1.0, 1.5] {
                    v.push(Band { kappa, mu, f });
                }
            }
        }
        v
    }

    /// Largest window searched (the table of `2^w` points is also built
    /// classically, so it is capped).
    pub const MAX_WINDOW: usize = 24;

    /// Resource figures of one run of the circuit.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct RunCost {
        pub qubits: f64,
        pub toffoli: f64,
        /// 0 for a plain (bit-controlled) circuit.
        pub window: usize,
        /// Classical point additions to build the window tables.
        pub table_additions: f64,
    }

    /// `ops_a + ops_b` bit-controlled additions of a classical point, each at
    /// `f · p(n)`; qubits stay at the RNSL figure.
    pub fn plain_cost(n: usize, ops_a: usize, ops_b: usize, f: f64) -> RunCost {
        RunCost {
            qubits: rnsl_qubits(n) as f64,
            toffoli: (ops_a + ops_b) as f64 * f * rnsl_addition_toffoli(n),
            window: 0,
            table_additions: 0.0,
        }
    }

    /// Windowed circuit: `⌈ops_a/w⌉ + ⌈ops_b/w⌉` windows, each one look-up
    /// (`mu · 2^w` Toffoli) and one quantum–quantum addition
    /// (`kappa · f · p(n)`); `2w` extra qubits, and `2n` more when `mu = 1`
    /// (the looked-up point is kept in two registers).
    pub fn windowed_cost(n: usize, ops_a: usize, ops_b: usize, w: usize, b: Band) -> RunCost {
        let windows = ops_a.div_ceil(w) + ops_b.div_ceil(w);
        let per_window = b.kappa * b.f * rnsl_addition_toffoli(n) + b.mu * (1u64 << w) as f64;
        let extra = 2.0 * w as f64 + if b.mu <= 1.0 { 2.0 * n as f64 } else { 0.0 };
        RunCost {
            qubits: rnsl_qubits(n) as f64 + extra,
            toffoli: windows as f64 * per_window,
            window: w,
            table_additions: windows as f64 * (1u64 << w) as f64,
        }
    }

    /// The window `1..=MAX_WINDOW` that minimises the Toffoli count at a band
    /// point (ties to the smaller window).
    pub fn best_window(n: usize, ops_a: usize, ops_b: usize, b: Band) -> RunCost {
        let mut best = windowed_cost(n, ops_a, ops_b, 1, b);
        for w in 2..=MAX_WINDOW {
            let c = windowed_cost(n, ops_a, ops_b, w, b);
            if c.toffoli < best.toffoli {
                best = c;
            }
        }
        best
    }

    /// Ekerå's two exponent registers for an `m`-bit group order: `(m + σ,
    /// ℓ)` with `ℓ = ⌈m/s⌉ + ν` (`ν = 0` for the lattice arms; `ν < 0`
    /// trades quantum work for the classical limited search when `s = 1`).
    pub fn ekera_registers(m: usize, sigma: usize, s: usize, nu: i64) -> (usize, usize) {
        let ell = (m.div_ceil(s) as i64 + nu).max(1) as usize;
        (m + sigma, ell)
    }

    /// Half-width of the `t` search box, `⌊(B_Δ + ½) r / 2^ℓ⌋`, for a group
    /// order `r ≈ 2^m` (as `f64`, so that it also serves `m = 256`).
    pub fn search_half_width(log2_r: f64, ell: usize, b_delta: f64) -> f64 {
        ((b_delta + 0.5) * (log2_r - ell as f64).exp2()).floor()
    }

    /// Counted classical work of the single-run limited search: baby-step
    /// giant-step over `[-Bt, Bt]` for each of `2 B_η + 1` values of `η`, in
    /// group additions `g` (two scalar multiplications of `1.5 m` additions
    /// per `η`, the giant steps, the shared baby table, one verification) and
    /// hash probes/inserts `n_native`.
    pub fn single_run_search_work(m: usize, bt: f64, b_eta: usize) -> (f64, f64) {
        let w = 2.0 * bt + 1.0;
        let q = w.sqrt().ceil();
        let giants = (w / q).ceil();
        let etas = (2 * b_eta + 1) as f64;
        let g = (q - 1.0) + 1.0 + etas * (3.0 * m as f64 + 1.0 + giants) + 1.5 * m as f64;
        let native = q + etas * giants;
        (g, native)
    }

    /// The most negative `ν ≤ 0` (`ℓ = m + ν`) whose single-run search work
    /// `g + native` (native work at 1 group addition per operation) stays at
    /// or below `2^tier_log2`.
    pub fn tier_nu(m: usize, b_delta: f64, b_eta: usize, tier_log2: f64) -> i64 {
        let mut best = 0i64;
        let mut nu = 0i64;
        while nu > -(m as i64) + 2 {
            let ell = (m as i64 + nu) as usize;
            let bt = search_half_width(m as f64, ell, b_delta);
            let (g, nat) = single_run_search_work(m, bt, b_eta);
            if (g + nat).log2() <= tier_log2 {
                best = nu;
            } else {
                break;
            }
            nu -= 1;
        }
        best
    }

    /// `log2` of Pollard rho on a group of `m`-bit order: `½ lg(π r / 4)`.
    pub fn matched_rho_log2(m: usize) -> f64 {
        0.5 * (std::f64::consts::PI / 4.0 * (m as f64).exp2()).log2()
    }

    /// Expected number of runs when runs are added one at a time from `s` to
    /// `s + k` and post-processing is tried after each; `cum[i]` is the
    /// probability that d is recovered from the first `s + i` runs (made
    /// monotone here).  If the last attempt fails the whole cycle restarts.
    /// Returns `(expected runs, cycle success probability)`.
    pub fn expected_runs_sequential(s: usize, cum: &[f64]) -> (f64, f64) {
        let mut c = Vec::with_capacity(cum.len());
        let mut run_max = 0.0f64;
        for &x in cum {
            run_max = run_max.max(x);
            c.push(run_max);
        }
        let last = *c.last().unwrap_or(&0.0);
        if last <= 0.0 {
            return (f64::INFINITY, 0.0);
        }
        let mut per_cycle = s as f64;
        for x in c.iter().take(c.len().saturating_sub(1)) {
            per_cycle += 1.0 - x;
        }
        (per_cycle / last, last)
    }

    #[cfg(test)]
    mod prime_shor_tests {
        use super::*;

        #[test]
        fn rnsl_qubits_at_256_is_2330() {
            assert_eq!(rnsl_qubits(256), 2330);
        }

        #[test]
        fn baseline_arm_reproduces_the_closed_form() {
            let c = plain_cost(256, 512, 0, 1.0);
            let rel = (c.toffoli - rnsl_toffoli(256)).abs() / rnsl_toffoli(256);
            assert!(rel < 1e-12);
            // The committed table value 1.26e11 is 2.2% below the formula.
            let ratio = rnsl_toffoli(256) / 1.26e11;
            assert!(ratio > 1.02 && ratio < 1.03, "{ratio}");
        }

        #[test]
        fn window_at_haner_calibration_is_near_19() {
            let b = Band {
                kappa: 1.0,
                mu: 3.0,
                f: 1.0 / 62.0,
            };
            let c = best_window(256, 263, 256, b);
            assert!((15..=22).contains(&c.window), "{}", c.window);
        }

        #[test]
        fn windowing_never_loses_qubit_accounting() {
            let b = Band {
                kappa: 1.0,
                mu: 1.0,
                f: 1.0,
            };
            let c = windowed_cost(256, 512, 0, 16, b);
            assert!(c.qubits > rnsl_qubits(256) as f64 + 512.0);
        }

        #[test]
        fn tiers_are_monotone() {
            let a = tier_nu(256, 100.0, 0, 30.0);
            let b = tier_nu(256, 100.0, 0, 40.0);
            let c = tier_nu(256, 100.0, 0, 58.0);
            assert!(a >= b && b >= c && c < 0);
        }

        #[test]
        fn paper_example_is_inside_the_tier_list() {
            // Ekerå's example: ν = -57 costs ~2^33 group operations.
            let ell = 256 - 57;
            let bt = search_half_width(256.0, ell, 100.0);
            let (g, n) = single_run_search_work(256, bt, 0);
            let lg = (g + n).log2();
            assert!((30.0..40.0).contains(&lg), "{lg}");
        }

        #[test]
        fn sequential_runs_formula() {
            let (e, c) = expected_runs_sequential(2, &[0.5, 0.9, 1.0]);
            // cycle: 2 + (1-0.5) + (1-0.9) = 2.6, success 1.0
            assert!((e - 2.6).abs() < 1e-12 && (c - 1.0).abs() < 1e-12);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **Resource scaling table** matching Putranto et al.'s
    /// Table 2 structure: PA and PD-Model-2 for n ∈ {8, 16, 127,
    /// 163, 233, 283, 409, 571}.
    #[test]
    fn print_resource_table() {
        println!();
        println!("=== Quantum-circuit resource estimates ===");
        println!();
        println!(
            "{:>4} | {:>9} {:>11} {:>11} {:>11} | {:>9} {:>11} {:>11} {:>11}",
            "n",
            "PA qbits",
            "PA Toffoli",
            "PA CNOT",
            "PA depth",
            "PD qbits",
            "PD Toffoli",
            "PD CNOT",
            "PD depth",
        );
        for &n in &[8, 16, 127, 163, 233, 283, 409, 571] {
            let (pa, pd) = ecpm_single_step_stats(n);
            println!(
                "{:>4} | {:>9} {:>11} {:>11} {:>11} | {:>9} {:>11} {:>11} {:>11}",
                n,
                pa.qubits,
                pa.toffoli,
                pa.cnot,
                pa.depth,
                pd.qubits,
                pd.toffoli,
                pd.cnot,
                pd.depth,
            );
        }
        println!();
        println!("=== Total-step ECPM (2n PD + 2 PA) ===");
        println!();
        println!(
            "{:>4} | {:>9} {:>13} {:>13}",
            "n", "qubits", "Toffoli", "depth"
        );
        for &n in &[8, 16, 127, 163, 233, 283, 409, 571] {
            let total = ecpm_total_stats(n);
            println!(
                "{:>4} | {:>9} {:>13} {:>13}",
                n, total.qubits, total.toffoli, total.depth,
            );
        }
    }

    /// Karatsuba Toffoli count scales as `n^log₂3 ≈ n^1.585`.
    #[test]
    fn karatsuba_scaling_is_subquadratic() {
        let s8 = karatsuba_mul_stats(8);
        let s16 = karatsuba_mul_stats(16);
        let s32 = karatsuba_mul_stats(32);
        // Doubling n should multiply Toffoli count by ~3 (since
        // log₂(2^1.58) = 1.58 ⇒ 2^1.58 ≈ 3).
        let ratio_16_8 = s16.toffoli as f64 / s8.toffoli as f64;
        let ratio_32_16 = s32.toffoli as f64 / s16.toffoli as f64;
        assert!(
            (2.5..=3.5).contains(&ratio_16_8),
            "Karatsuba scaling 8→16 expected ~3, got {}",
            ratio_16_8
        );
        assert!(
            (2.5..=3.5).contains(&ratio_32_16),
            "Karatsuba scaling 16→32 expected ~3, got {}",
            ratio_32_16
        );
    }

    /// PA uses more Toffolis than PD (since PA does 2 muls; PD does 1).
    #[test]
    fn pa_costs_more_than_pd() {
        let pa = pa_stats(163);
        let pd = pd_stats(163);
        assert!(pa.toffoli > pd.toffoli);
    }

    /// Total ECPM is dominated by `2n PD` for large n.
    #[test]
    fn ecpm_total_dominated_by_pd() {
        let n = 163;
        let total = ecpm_total_stats(n);
        let pd = pd_stats(n);
        let pa = pa_stats(n);
        let pd_contribution = (2 * n) * pd.toffoli;
        let pa_contribution = 2 * pa.toffoli;
        assert!(
            pd_contribution > 10 * pa_contribution,
            "2n PD should dominate 2 PA by ≥10× at n=163"
        );
        assert_eq!(total.toffoli, pa_contribution + pd_contribution);
    }

    /// Circuit-builder depth increments correctly with sequential
    /// gates on the same qubit.
    #[test]
    fn circuit_depth_increments() {
        let mut c = Circuit::new(2);
        assert_eq!(c.stats().depth, 0);
        c.single_qubit(0);
        c.single_qubit(0);
        c.single_qubit(0);
        assert_eq!(c.stats().depth, 3, "three single-qubit gates on q0");
        c.cnot(0, 1);
        // q0 has depth 3, q1 has depth 0; CNOT joins them at max+1 = 4.
        assert_eq!(c.stats().depth, 4);
    }

    /// Toffoli gate raises three qubits' depths simultaneously.
    #[test]
    fn toffoli_synchronises_qubits() {
        let mut c = Circuit::new(3);
        c.single_qubit(0);
        c.single_qubit(0);
        // q0 depth = 2, q1 = 0, q2 = 0.
        c.toffoli(0, 1, 2);
        // All three now at depth 3.
        assert_eq!(c.stats().depth, 3);
    }
}
