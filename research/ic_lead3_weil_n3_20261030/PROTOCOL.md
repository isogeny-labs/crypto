# Lead 3: Weil descent at field degree n = 3 (protocol, frozen before data)

Question: on E(F_{p^3}), does Gaudry/Diem index calculus (factor base {P : x(P) in F_p},
Semaev S_{m+1} Weil-restricted) beat rho in the exponent, what does it cost per relation, and
where would it cross rho?

Instrument: the repo's `cryptanalysis::gaudry_cubic` module and `examples/gaudry_cubic_bench.rs`
(unmodified; built in a scratch crate that `#[path]`-includes them, with no-op stubs of the
`jv_bitops` tally hooks and a copy of the `FxHasher` part of `fx_hash.rs`).
Counted unit: affine additions in E(F_{p^3}) + F_p multiplications / 63 (measured Fp-muls per add),
S = total_ops / sqrt(n); rho on the same group via `run_rho3` (same unit, S_rho ~ 1.2-1.8).

Frozen design (amended before any data; the first draft said seeds 3-5, but `--protocol` loops seeds 1..=N):
- `--seeds 5` (seeds 1..5). Seeds 1,2 at p<=2083 are the REPLAY CHECK of the scratch build against the
  committed experiments/21_gaudry_cubic_la.json (counted fields residuals, oracle_fp_muls, group_ops, la_ops, total_ops
  must match exactly); seeds 3,4,5 are new.
- sizes p in {271, 523, 1039, 2083, 4099, 8209} (p = 1 mod 3), n ~ p^3 prime order; sizes above 4099 only if the 4099
  smoke run projects < 40 min wall for the 2LP arm.
- arm P (paired): `--protocol-la --groebner`: dense LA, Wiedemann, and Wiedemann + double large primes
  (|F'| = |F|^{2/3}) on the same instance (m=3, Gaudry S_4 solve).
- arm M  m=3 MitM: `--protocol` (S_3 pair tests per residual), p <= 2083.
- arm J  m=2 (Joux-Vitse): `--protocol --pair-only`, p <= 4099.
Reported per row: residuals, relations independent, decomposition rate, ops per residual and per relation,
total ops, S, rho steps and S_rho. Fit log-log exponents in n by least squares over seeds; report relation phase and
LA phase separately; crossover with rho under theory exponent and fitted exponent (with CI); AGENTS.md 5a price band
0/0.1/1 GAE for unpriced native work. Plot S vs log2 n per arm with rho; PDF.
Stop: any run > 40 min wall is abandoned and reported as such.
Success/kill: 2LP arm fitted total exponent below 1/2 (rho) with plain arm above (theory 4/9 vs 2/3).
Prior art: RESEARCH_RESIDUAL_WALKS.md section 11 (esp. 11.15, 11.19) ran this at p<=2083; this is a replication
plus range extension and is compared against 11.19's k=4 constant.
