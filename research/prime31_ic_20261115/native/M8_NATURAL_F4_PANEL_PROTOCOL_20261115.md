# EXP-R31-D — natural public-mask S4/S3 F4 discrimination (2026-11-15)

**Standing:** development protocol and locally generated arithmetic labels, **[pcc:unverified]**. No compiled exact-head F4 panel result, new algebraically discovered witness, full prime31 scalar recovery, rho win, or cost exponent is asserted. The GitHub Actions step is a bounded development run, not the authorized EKS/PCC benchmark.

## Target, frozen input, and what changed

Reuse the repository's **existing Rust** `f4_fp_tower.rs` and `sig_fp_tower.rs` through the focused `native` crate and the balanced S4/S3 expression already checked into PR #4188:

`S4(x0,x1,x2,u)=S4(x3,x4,x5,v)=S3(x6,x7,w)=S4(u,v,w,x(R))=0`.

The field is p=2^31-1=2147483647. The original ordinary nonsingular curve is `y²=x³+847514846x+2087444724`, prime group order `n=2147446831`, cofactor one, `G=(625037715,2100627251)` and fixed public `Q=(1547953785,1667575595)`. This is not a weak/smooth-order substitute.

The committed `M8_NATURAL_F4_PANEL_20261115.json` freezes the first **16** deterministic, **unplanted** public masks `R=[a]G+[b]Q` from the pre-existing SplitMix64 seed `0x4d384e4154555241`. The factor base is the inherited Dickson depth-six fibre, 31 canonical points, terminal 933677148. A separate **generic** validation enumerator examined the original restricted 15+16 signed 4+4 partition. It found **5 masks with witnesses, 11 with no witness in that restricted partition, and 6 total signed templates**. All classifications are local producer arithmetic, not independently signed observations.

This control is essential: **11 split misses are not algebraically certified negative systems.** The balanced S4/S3 ideal permits *other* index partitions and sign classes. A dense/F4 solver correctly returning an unexpected new witness on a split miss is a discovery, not an error. Never condition the F4 algorithm on the frozen membership labels.

## Exact new combinatorial decision (no probabilistic independence assumption)

For any split into `s` and `31-s` points, taking four distinct indices and all 16 sign assignments per side, the template count is

`T(s)=256 * binom(s,4) * binom(31-s,4)`.

For a uniform public group target the **expected raw template count** is exactly `T(s)/n` by linearity of expectation, regardless of collisions. The binomial coefficients are symmetric and log-concave, so the maximum occurs at `s=15,16`. The registered balanced split gives `T(15)=635980800`; the next-nearest `14+17` split gives `T(14)=609889280`. Thus moving to an unbalanced split **cannot** improve expected raw template yield for this fixed B=31 and fixed 4+4 family (balanced is 4.28% higher than 14+17 in raw counts). This is **not** a bound on independent rank, extraction difficulty, distinct target support, or alternative algebraic constructions. Alternate *balanced* partitions remain a valid research question, because the input geometry and support can differ.

## Run protocol and controls

The existing Rust example `examples/prime31_m8_balanced_f4_input.rs --panel-f4` now (1) independently rederives each full public target from its frozen `a,b,G,Q`; (2) constructs exactly the four balanced m8 quotient-ring input polynomials; (3) calls the **actual reused Rust** `f4_tower` with degree 12, 5 seconds cooperative work, and 15000-entry sparse/dense caps per mask; (4) records completed refutations separately from undetermined/censored results; (5) for the five labeled positives only, checks a pre-existing signed full-point witness against the public `R`, all four input polynomials and every emitted F4 basis polynomial. No generic meet-in-middle relation lookup is performed by the oracle. Labels do not affect its inputs, degree, caps or branching.

A known valid witness must never be refuted. A completed refutation on an **unlabeled split miss** may still be a valid unrestricted-m8 refutation if all algebraic saturation/exception cases are handled; the current pilot does **not** certify that.

**Prespecified diagnostic:** run all 16 calls, including misses and censoring; compare the positive-vs-nonpositive F4 work distributions, report solver internal `ms/muladds/NNZ/degree` and cooperative cap status for each call, and do not fit a speedup or solver complexity exponent. Root extraction, usable-row rank, all cold setup, allocation, and final scalar checking are not yet integrated into this pilot.

A correct Rust run demonstrates a reproducible oracle-work distribution and potential false-refutation regression; a positive result without recovered x roots remains a *basis computation*, not index-calculus relation collection. The next implementation gate is **F4 root extraction on this natural panel, checking each full signed y lift, followed by rank and complete cold Q recovery under the existing ecbench and PCC gates**.

## Prior searches and non-duplication

- **PKM (Petit–Kosters–Messeng, PKC 2016)**, primary paper `https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf`: rational-map prime-field algebraic index calculus and Semaev relation systems are prior art. Neither the base method nor the balanced Semaev splitting is claimed new.
- **Diem 2011**, `https://doi.org/10.1112/S0010437X10005075`: subexponential analysis for extension-field sequences and certain selected prime-field sequences; this does not imply a practical generic-prime-field attack.
- **crypto #4077**: frozen m8 useful-output-versus-full-call cost gate. **#4275**: dense-vs-tower oracle on small *planted-positive* experiments, not this frozen natural prime31 panel. **#4228**: certified paired-S3 quotient candidate. **#3877**: chained transfer work. **cryptanalysis #1536**: auxiliary-map Rust IC; no 31-bit cold superiority claimed. **crypto-autoresearcher #3305**: lead atlas and previously searched/negative branches. Retain #4188's m3 projective rank-dedup control; raw collisions cannot be counted as independent equations.
- Research-KB MCP, repository KB wrapper, Conductor check, authenticated EKS/checkpoint gate, Rust compiler in this connector/container runtime, and independently signed PCC execution verifier were not available for this change. This bounded review must **not** be reported as an exhaustive novelty search or a passed benchmark.

## Advancement condition

A complete eligible solver must use non-generic algebraic relations, reveal neither target scalar nor planted answers, charge every failed F4 call, basis extraction, relation verification, independent rank, linear algebra, factor-base preparation and target recovery, and verify `[d]G=Q`. Compare matched optimized negation-map rho and BSGS on the **same exact curve and cold protocol**, independently replay results and grow the tested sizes before any exponent claim. Keep PR #4188 open pending these gates.
