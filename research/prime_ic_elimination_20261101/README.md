# Prime-field index-calculus lead elimination campaign

Date: 2026-11-01
Status: locally executable registered routes complete; broader mechanisms
remain explicitly blocked/open
Source baseline: `isogeny-labs/crypto@3d76699b00`

## User requirement and acceptance criteria

The requested outcome is to pursue every ranked prime-field index-calculus lead
until it is either disproved or exhausted.  This campaign therefore keeps four
outcomes separate:

- **disproved:** a checked counterexample invalidates the stated mathematical
  claim under its own hypotheses;
- **rejected:** a frozen experiment crosses its prospective do-not-advance
  threshold with all controls valid;
- **exhausted within scope:** every registered experiment in a declared finite
  budget completed without an advancing result;
- **blocked/open:** required code, proof, execution identity or infrastructure
  is missing.  A blocker is never reported as negative scientific evidence.

Completion requires, for every lead, a source-linked claim, an explicit
falsifier, mutation and null controls where applicable, retained failures and
censoring, exact source/output identities, and a complete-cost interpretation.
Finite experiments may reject a representation or implementation; only a
counterexample or proof can decide a family theorem.

## Primary sources checked

| Source | Pinned identity | Campaign use |
| --- | --- | --- |
| Akshima et al., *Subexponential ECDL*, v0.2 | `bochum-crypto/subexponential-ecdl@1ee44e562b1b9a50ea339f3b8f81112b5a24b96f`, PDF SHA-256 `82e273629b437660607117d5e99f400141ed7aa78b25c6749b03651f42deaa11` | Conditional hashed-factor-base reduction, edge/degree and complete relation-collection costs |
| OpenAI, *The Exponential Time Hypothesis is False*, Appendix D | `openai/math@7ef5c0cc5ca4b0e072b7a050fca09ab24bc6fa3a`, PDF SHA-256 `5b1612dc2eebb2cad5b4cbe62d1fed2982f01ab1908c26f70010a651487e8123` | Theorem D.1 hypotheses and D.4 packet/deformation/precision recurrence |
| Petit--Kosters--Messeng, PKC 2016 | PDF SHA-256 `a53030a6cde720c354aebfb65b1eb01e07766ce2d26e543d77f2516c0a49ef5b` | Prime-field algebraic factor bases and generalized root-finding obligation |

The two first PDFs were retrieved again from the exact commits and their
registered hashes matched.  The decisive Appendix D theorem and full D.4
recurrence were read against the native implementation.  Repository evidence
reviewed includes `prime_ic_lead_review_20261031`,
`tensor_boolean_breakthrough_20261031`, `hashed_ecdl_counts_20261031`,
`prime_shared_coefficient_sat_20261031`, `ic_m4_mitm_scaling_20261031`, and
`prime_ic_zero_filter_20261030`.

## Frozen lead register

| Rank | Lead | Current evidence | Decisive next experiment | Current state |
| ---: | --- | --- | --- | --- |
| 1 | Hashed factor-base Boolean tensor contraction | The native finite fallback is rejected; symbolic guards first pass at `lambda=2^480`.  Semantic cuts expose 154--354 raw fanout edges but only 7--15 exact signal classes.  Natural ranks beat support-matched sparse nulls, while all four matched cap-1 typed topology nulls have exactly the same rank. | A full arbitrary-rewire projected model counter, or a composable quotient message with complete costs | legacy census infeasible; minimal typed global-wiring mechanism rejected; theorem/full null open |
| 2 | Fixed-`m=3` chained-`S3` CDCL(T)/`F_p` | The frozen F17 SAT correctness panel passed 36/36 target/backend model-set comparisons and 42/42 independently checked UNSAT proofs.  No source-bound `F_p` trail theory or measured natural-query exponent exists. | Explanation-checked trail theory, then the complete `s=6..14` panel with strict `e<0.5`, `a<1.5` gates | representation gate passed; performance implementation-blocked |
| 3 | PKM `m=4` generalized root finding | F4 degree is 6,7,7 at `N=8,12,16`, already above the recorded `m=4` complete-cost threshold 4.  Direct 2+2 MITM has planted exponent about 0.4 but uniform-relation rarity trends to `p^0.6`. | Registered `N=20` closure as a degree-growth diagnostic; identical-system hybrid comparison | current MITM and F4 routes rejected; broader solver family resource/backend-blocked |
| 4 | Target-parametric state reuse | The current exact `Prepared`/`Counter` path is history-independent down to charged steps and assignment counters; it retains no target-dependent online state.  Local membership diagrams exist, but no compact full-summation message exists. | Implement a provenance-audited retained full-summation state before the `1/8/64/512` factorial | current implementation exhausted; broader mechanism open |
| 5 | Special-`j`/Solinas/chart constants | The frozen 24-cell exact panel reproduced in every cell.  All Boolean-variable, CNF and CNF-XOR slope differences have magnitude at most `0.000604`, against the `0.05` advance gate. | Common-backend arithmetic/propagation and matched-subgroup factorial | representation-size route rejected; broader solver effects blocked |

The machine-readable state and falsifiers are in `campaign.json`.  Status is
updated only from retained evidence; an absent cloud credential, timeout, OOM,
unchecked UNSAT or missing rung remains `blocked` or `inconclusive`.

This closes the current execution pass, not every mathematical lead. No
family-wide claim was disproved. The registered executable routes produced
scoped rejections, one passed representation gate and two exhausted current
implementations; arbitrary projected counting, a source-bound `F_p` trail
theory, a retained full-summation state and specialized common-backend tests
cannot be called exhausted until those missing mechanisms or resources exist.

## Execution constraints observed before data

- The managed environment exposes no AWS identity, runtime secret or outbound
  identity.  EKS Spot submission and private receipt retrieval are currently
  blocked.
- The repository checkpoint gate also lacks both its AWS profile and upload
  token.  Frozen study runners that require that gate will not be bypassed.
- `conductor` is not installed in this environment (`exit 127`).  Work is
  isolated to this campaign, the tensor crate and the append-only session log.

Local builds, exact arithmetic checks and source audits are development work;
they do not substitute for the required isolated measurements.

## Completed campaign components

- [`RANK1_GUARDS.md`](RANK1_GUARDS.md): arbitrary-precision Appendix D.4 guard
  transcript, predecessor control, and native representation boundary.
- [`RANK1_CODE_CONTROLS.md`](RANK1_CODE_CONTROLS.md): bounded direct-versus-
  recursive identities and load-bearing packet/mode mutation controls.
- [`RANK1_QUOTIENT_RESULT.md`](RANK1_QUOTIENT_RESULT.md): production semantic
  cuts, exact equality quotient, held-out scaling and two controlled nulls.
- [`RANK3_M4_DISPOSITION.md`](RANK3_M4_DISPOSITION.md): complete-cost
  disposition of the two implemented `m=4` routes and the exact remaining
  generalized-root-finding obligations.
- [`RANK2_SAT_E1_RESULT.md`](RANK2_SAT_E1_RESULT.md): proof-checked F17
  CNF/CNF-XOR representation-equivalence result and evidence boundary.
- [`RANK4_REUSE_RESULT.md`](RANK4_REUSE_RESULT.md): exact fresh-versus-reused
  target-history control and the current implementation boundary.
- [`RANK5_REPRESENTATION_RESULT.md`](RANK5_REPRESENTATION_RESULT.md): frozen
  24-cell special-curve/modulus representation screen and exact counts.
