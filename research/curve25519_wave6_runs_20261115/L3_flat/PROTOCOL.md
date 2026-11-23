# L3: flat y^D = 1 against the power-map chain, as a component ratio (frozen before data, 2026-11-15)

Question. At equal base V = μ_D on the k = 20 member (p = 1,048,573, A = 926), is the decomposition system
cheaper with one variable of degree D per summand (`subflat`) than with the chain of one variable per prime
factor (`sub`, the exponent studies' presentation)?

Instrument. `curve25519_family f4sub 20 subflat D 0 mont M sym 6 SEED 1800` with SEED = 20261108 + 1000·M + D
(so each flat cell draws the same six targets as the recorded chain cell), `ulimit -v 6000000`, RAYON_NUM_THREADS=1.
Cells: m = 3: D ∈ {9, 12, 14, 18, 21, 27, 28, 36} ascending, stop at the first censored cell; m = 2:
D ∈ {9, 21, 27, 57, 63, 133, 171, 189, 219, 399} ascending (all below the engine's degree cap of 400), stop at the
first censored cell. Binary control: rerun the chain cells k20 m3 D = 9 and D = 21 and k20 m2 D = 57 with the new
binary (`sub`); their per-target field_ops must equal the recorded dc2c30a4 values exactly, otherwise every ratio
below is reported against the rerun, not the record.

Predictions. P1: flat SD = D + 1 at every completed flat cell. P2 (ratio): chain / flat mean field_ops at m = 3 is
in [70, 260] at D = 27 and 28 (W5-RP-1's band), and above 10 at D = 9, 12, 14, 18, 21. P3: flat D = 36 at m = 3
completes under 6 GB and 1,800 s per target (the chain cell censored at 12 GB). P4: at m = 2 the chain / flat ratio
is above 1 at every D with two or more prime factors and the flat cost follows the recorded prime-D curve
(T ≈ 9.7e4 · (D/19)^2.2 within a factor 3). P5: the binary control reproduces the recorded field_ops exactly.

Decision. P2 failing (ratio below 10 at D ≤ 21, or below 70 at D = 27, 28) refutes the "chain is a net loss"
reading; P1 failing refutes the theory lane's W6-TH-5. A measured ratio above 1 at every D is a component
improvement of the decomposition oracle for subgroup bases, not a pipeline result and not a leaderboard row.
