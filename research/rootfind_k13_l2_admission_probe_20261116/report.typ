#set page(paper: "us-letter", margin: (x: 0.72in, y: 0.67in))
#set text(font: "New Computer Modern", size: 9pt)
#set heading(numbering: none)

= K13 solve-duration admission readback

*Scope.* Read-only audit of two archived Apple M4 Pro L0 K13 sessions. No new
curve solve, x86 observation, or solver performance verdict. Measurement record
`PCC-20261116-912e14` is unverified; the recomputation wrapper is
producer-authored.

The hash-checked JSONL inputs are
`0be204a394b2ef7ef223cb30960f12747ea30f736a135f46c2c0d76ecf0dbbde`
(target seed 202611160401) and
`dc16c97ae21a250cc1d47237231d9642960c100dc3fdb68f4dd210af171bff44`
(target seed 202611160511). `duration_readback.py` checked the registered
Koblitz E0 n13 identity, outcomes and L0 grade. All 32 and 4 measured
batch-m3 solve clocks respectively were below 1 ms; their ranges were
0.239–0.572 and 0.263–0.505 ms. The exact record IDs are in
`DURATION_READBACK.json`.

#figure(image("flow.svg", width: 55%), caption: [Local duration evidence identifies a possible L2 admission failure. The x86 probe and frozen panel are separate tests.])

`src/cryptanalysis/ecbench/measurement.rs` sets the grade-v3 floor at
1,000,000 ns. The frozen ten-arm panel requires every admitted pair at L2 or
higher. Its target seed is 202611151301, different from both archived inputs.
These local clocks cannot predict x86 duration. The new protocol holds the
curve, factor base, public targets and method fixed; it checks 32 measured
batch solves and an A/A arm before the 560-execution comparison.

*Current execution state.* The correct EKS account exceeds its monthly and
daily spend limits, and Conductor T-6814 holds the checkpoint path. The probe
is unrun. No canonical leaderboard graph changes from this readback. The
separate complete-cost problem record retains native-pricing, isolation and
independent-replay requirements.
