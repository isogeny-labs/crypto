# Item 20: classical pre- and post-processing for quantum ECDLP

## Verdict

**INCONCLUSIVE** (level: **constant**; no exponent or primitive change is claimed).

The frozen rule (PROTOCOL.md, "Decision rule") makes the item INCONCLUSIVE if any
control fails. Control 3 (negative control, random pairs must hit the chance rate
within 4 standard errors in every cell) failed in 21 of 567 single-run cells, all at
m = 12 (card 02, `raw/s12.jsonl`). Every other gate passed: the 8 selftest gates,
the positive control, monotonicity, search/criterion agreement (0 mismatches in every
cost cell), and the null control at m = 16 and m = 20. The rule gives no exception, so
the item verdict is INCONCLUSIVE. The arm-level mechanical verdicts below are reported
as what the rule yields row by row, not as the item verdict.

Arm-level rule outputs at n = 256 (P-256 and secp256k1 share one row), against A0 at
the same band point, A0 = committed RNSL baseline with success taken as 1:

| arm | product ratio vs A0 (band) | classical work, log2 GAE at c = 0 / 0.1 / 1 | rule output |
|---|---|---|---|
| E1-0 (Ekera, sigma 7, no search) | 1.021 | 10.21 / 10.21 / 10.24 | MATCHES |
| E1-2^20 (nu = -28) | 0.966 | 18.83 / 18.97 / 19.83 | MATCHES |
| E1-2^30 (nu = -48) | 0.927 | 28.83 / 28.96 / 29.83 | ADVANCES |
| E1-2^40 (nu = -68) | 0.887 | 38.83 / 38.96 / 39.83 | ADVANCES (uncapped, edge of the 2^40 cap) |
| E1-2^50 (nu = -88) | 0.848 | 48.83 / 48.96 / 49.83 | TRADES (capped) |
| E1-2^58 (nu = -104) | 0.816 | 56.83 / 56.96 / 57.83 | TRADES (capped) |
| E2-s2 (lattice) | 1.715 | 6.97 / 7.95 / 10.40 | REGRESSES |
| E2-s3 (lattice) | 2.205 | 8.45 / 9.75 / 12.41 | REGRESSES |
| W(A0), window 16-24 | 0.038 to 0.110 | 28.46 (tables) at all c | ADVANCES |
| W(E1-0) | 0.039 to 0.111 | 28.46 / 28.46 / 28.46 | ADVANCES |
| W(E1-2^40) | 0.034 to 0.095 | 38.83 / 38.96 / 39.83 | ADVANCES |

If the m = 12 null failures were set aside (a post hoc step the protocol does not
allow), the best arm output would be ADVANCES at the constant level. Do not read
that as a result of this item.

Against A0p (A0 with its measured single-run success, 0.60) every E1 and W row
advances more strongly (0.49 to 0.62 for E1). That column is never decisive: it
corrects the baseline's assumed success of 1, which is an accounting change.

EH-128/160/192 (Ekera-Hastad short logarithms, 0.38/0.47/0.57 of A0) are a promise
problem, cited not simulated, and carry no verdict.

## What was run

Six frozen cards (`card-01.yaml` to `card-06.yaml`), all completed with every command
exit 0 and `card_sha256` equal to the committed card (checked at assessment). The
native Rust instrument `examples/blindspots_20_quantum_classical.rs` plus
`prime_shor` in `src/cryptanalysis/quantum_estimator.rs` (both bound by sha256; same
files at HEAD). Nothing was re-run for this assessment. Raw outputs:
`raw/*.jsonl`, `raw/receipt-card-0N.json`. Graph: `graph.svg`, `graph.png`,
source `plot_item20.py` (plots recorded values only).

Method: a resource model (RNSL closed forms, 9n + 2 ceil(lg n) + 10 qubits and
448 n^3 lg n + 4090 n^3 Toffoli at n = 256 gives 2,330 qubits and 1.2875e11 Toffoli,
2.2% above the committed table value 1.26e11) extended with Ekera's per-run
operation counts, window cost and a band `kappa x mu x f` (27 points) for the
windowed arms; and an exact sampler of the Shor measurement distribution on toy
cyclic groups (m = 12, 16, 20; prime just below 2^m, the shape of both group orders)
with real post-processing (counted BSGS search, hidden-number LLL lattice). The
single-run success probabilities used by the model are Wilson lower bounds measured
at m = 16 (2000 runs): p1 = 0.9928 (minimum over nu = 0, -2, -4), p0 upper bound
0.6024; lattice cumulative successes from 300 trials.

## Numbers (m = 16 unless stated; Wilson 95%)

Single run, B_eta = 0, curve-shaped prime, nu = 0, 4000 runs per cell:

| sigma | B_Delta = 0 (Bt = 0) | B_Delta = 10 | B_Delta = 100 |
|---|---|---|---|
| 0 | 0.601 [0.586, 0.616] | 0.767 [0.753, 0.779] | 0.775 [0.762, 0.788] |
| 2 | 0.748 [0.735, 0.762] | 0.941 [0.934, 0.948] | 0.951 [0.943, 0.957] |
| 4 | 0.772 [0.759, 0.785] | 0.977 [0.972, 0.981] | 0.986 [0.982, 0.989] |
| 7 | 0.783 [0.769, 0.795] | 0.991 [0.987, 0.993] | 0.998 [0.996, 0.999] |

At m = 12: sigma 7, B_Delta 100 gives 0.9969 [0.9956, 0.9978] (10000 runs); at
m = 20 (500 runs): sigma 4, B_Delta 100 gives 0.988 [0.974, 0.995]. Other-extreme
prime (S16w, r just above 2^15, 1500 runs): sigma 0, Bt = 0 gives 0.827
[0.807, 0.846], at the top edge of, and just above, Ekera's 60-82% band; sigma 7, B_Delta 100 gives
0.9987 [0.9952, 0.9996]. Shorter exponent register (nu = -2, -4, sigma 7, B_Delta
100): 0.9955 [0.9915, 0.9976] and 0.996 [0.9921, 0.998].

Counted post-processing cost (100 verified runs, B_eta 0, B_Delta 100): mean 91 to
94 group additions at nu = 0 against the worst-case formula 102 (ratio 0.90 to
0.92); with B_eta = 1, B_Delta = 30 the ratio is 0.41 to 0.49 (early exit). The
measured/formula ratio is the extrapolation error for every m = 256 figure.

Lattice (400 trials, sigma 4): recovery with s runs is 0.75 to 0.81 (s = 2) and
0.77 to 0.88 (s = 3) as point estimates, 0.97 to 1.00 with one extra run and 1.00
(Wilson lower bound 0.99) with s + 2 runs; mean runs to stop 2.2 (s = 2) and 3.1 to 3.3
(s = 3). Mean lattice work 1.0e3 to 1.8e3 flops (s = 2) and 5e3 to 7e3 (s = 3), 1.3
to 1.7 reductions. Full table: `raw/lattice-m*.jsonl`.

Distance to matched rho (derived: 0.5 log2(pi r / 4) = 127.83 bits against the
log2 post-processing work above): E1-0 117.6 bits, E1-2^40 88.0 bits, E1-2^58 70.0
bits at c = 1; at c = 0 and 0.1 add 0.0 to 1.0 bits (E1-2^40 89.0 and 88.9); E2-s2
117.4 bits at c = 1 (120.9 at c = 0). W rows: 99.4 bits (tables only), 88.0 for
W(E1-2^40). This is the matched-rho reference for the post-processing only. No
cross-method row exists here: the quantum arms are not ranked against rho, so the
section 5a band (0, 0.1, 1 GAE) is applied to the unpriced native work of the
classical step and the orders above hold at all three prices.

## Controls

* Selftest gate: all 8 pass (RNSL 2,330 qubits, Toffoli ratio 1.0218; sampler vs
  brute force to 6e-17; LLL/Babai; BSGS ratio 0.769; window best 16, model T 9.8e8
  against Haner range [2.3e9, 7.4e9]; search vs criterion 0/150).
* Positive control (m = 16, curve-shaped prime): sigma 7, B_Delta 100 lower bound
  0.9961 (>= 0.97); sigma 0, Bt = 0 point 0.601 (in [0.40, 0.90]); monotone in
  sigma and B_Delta: pass.
* Negative control: m = 16 and m = 20 pass in every cell. m = 12 fails in 21 cells
  (list in `raw/s12.jsonl`, `null_ok` false): the observed null rate is below the
  protocol's chance value in every failing cell and above it in none (z from -4.0 to
  -229). The chance formula (2 B_eta + 1)(2 Bt + 1)/r is a union bound that
  overcounts overlapping search boxes once the box covers a large share of r = 4093
  (for example B_eta 2, Bt 401: formula 0.98, observed 0.68 to 0.71, close to the
  independent-box value 1 - (1 - 803/4093)^5 = 0.67). This diagnosis is post hoc
  and is not a licence to pass the gate.
* Search/criterion agreement: 0 mismatches in every cost cell.

## Deviations

PROTOCOL.md amendments 1-7 (all made before any measured run; none loosens a gate):
other-extreme prime labelling, search work formula, cell sizes (S12 10000, S16 split,
S20 500, m = 20 lattice at sigma 4 only), selftest gates as built, A0p uses the upper
Wilson bound, positive control only at curve-shaped m = 16, per-trial lattice costs.
No deviation from the protocol happened in the cards or at assessment.

## Caveats

* Toy scale: success probabilities come from m = 16 groups (lower bounds); the
  extrapolation to m = 256 is assumed, not measured. The lattice arms' m = 256 work
  is extrapolated from m <= 20.
* The windowed rows are a model band, not a measured circuit. Best windows hit the
  cap of 24 at some band points, and the instrument's own gate puts the model at
  f = 1/62, kappa = 1, mu = 3 about 2.4x below the low edge of Haner et al.'s T-count
  range. Treat the 0.03 to 0.11 ratios as an optimistic band, not an estimate.
* The classical step for E1-2^40 reaches the stated 2^40 cap at c = 1 (39.83 bits);
  E1-2^50 and above are capped. The 2^40 line is the protocol's, not a measurement.
* P-256 and secp256k1 share a row (the model has no a = 0 versus a = -3 term).
* No claim about hardware timelines, depth, wall clock or error correction. Host:
  shared 4-vCPU container, isolation level L0 (no isolation claim); only counted
  operations are reported, no wall-clock result.
* This is a logical-resource model comparison, not a performance measurement, so
  nothing is copied into `benchmarks/`.

## Success criterion note

No row is a cross-method win over rho; the quantum arms are a different resource.
Relative to the committed RNSL baseline, the largest uncapped constant-factor
reduction is 0.887 (E1-2^40, Toffoli only; qubits unchanged); with the INCONCLUSIVE
control failure this is recorded as unverified, not as an improvement.
