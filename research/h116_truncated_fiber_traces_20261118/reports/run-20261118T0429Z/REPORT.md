---
title: "H116 formal-scalar low-jet run report"
date: "2026-11-18"
geometry: margin=0.72in
fontsize: 10pt
---

# Outcome

One deterministic excluded-fixture run constructed eleven normalized x-map
coefficients from `(p,A,B,t)=(31,1,19,-5)` without accepting the full map as
candidate input. The prefix
`[1,0,20,0,13,7,2,4,8,14,5]` equaled the retained full degree-37 Vélu-map
quotient. Its first nine entries equaled the independent Bostan recurrence,
and an independent verifier found a zero transformed first-order residual
through degree nine. [pcc:unverified]

![Construction, controls, and open semantic bridge. Green records passing
comparisons, purple records the formal cutoff, and red records obligations still
open after this run.](mechanism.png){width=100%}

The editable figure is `mechanism.dot`; `mechanism.svg` is its vector render.
No quantitative graph is included because the operation-count rows are
different workloads rather than repeated or commensurate arms. Plotting them
as a speed comparison would be misleading. [pcc:unverified]

# Run identity

| Field | Value |
| --- | --- |
| run id | `run-20261118T0429Z` |
| repository parent | `4340f2aa4d00f91c8d3cc93fdd573e7f5ee1b384` |
| generator identity | SHA-256 prefix `914c1909ffdd5e6b`; full digest in the experiment note |
| verifier identity | SHA-256 prefix `949c646348d4d55d`; full digest in the experiment note |
| result identity | SHA-256 prefix `b0afc3ee9ddc9e1b`; full digest in the experiment note |
| host | x86-64 Linux 6.18.57, AMD EPYC 9V45, 5 visible CPUs |
| isolation | ordinary workspace; no timing claim |
| requested/completed | 1 / 1 deterministic fixture |

The retained result is `../../FORMAL_SCALAR_JET_CONTROL.json`. The full-map
oracle is `../../MAP_CERTIFICATE_CONTROL.json`. The generator and verifier are
`../../h116_formal_scalar_jet.py` and
`../../h116_formal_scalar_verify.py`.

# Controls

| Control | Result |
| --- | --- |
| candidate constructed before oracle load | pass |
| full-map quotient, coefficients 0--10 | exact equality |
| recurrence, coefficients 0--8 | exact equality |
| transformed first-order identity, degrees 0--9 | zero residual |
| lambda=2 coordinate isomorphism | exact expected scaling |
| wrong scalar | rejected |
| unscaled target | rejected |
| candidate coefficient-10 mutation | verifier rejected |
| synthetic p=23 leading Frobenius term | first difference at `w^11` |
| synthetic p=19 leading Frobenius term | first difference at `w^9` |

# Exact operation counts

The counters report base-field operations in the naive dense-series code.
They are deterministic counts, not timings. Candidate construction excludes
JSON parsing, oracle work, controls, and serialization. [pcc:unverified]

| Stage | additions | multiplications | inversions |
| --- | ---: | ---: | ---: |
| candidate construction | 64,426 | 67,584 | 16 |
| candidate internal verification | 7,961 | 7,926 | 0 |
| supplied-map quotient oracle | 55 | 55 | 0 |
| recurrence control | 27 | 57 | 12 |
| p=23 synthetic control | 39,712 | 45,722 | 4 |
| p=19 synthetic control | 23,442 | 26,602 | 4 |
| unscaled-target control | 15,413 | 17,630 | 2 |
| wrong-scalar control | 72,387 | 75,510 | 16 |
| coordinate control | 72,387 | 75,510 | 16 |

# Formal evidence and boundary

Lean proves that `X^(p-1)` has zero prefix below degree 22 for every prime
`p>19`, hence its even coefficients through degree 20 vanish. The repository
gate reports `PASS 64 thm 0 trusted` with default limits. This formalizes the
precision cutoff only. The actual elliptic identities `V+pi=[t]`,
`pi(z)=z^p`, formal subtraction, and x-coordinate pole-order transfer remain
open. [pcc:unverified]

The result therefore supports one fixed-size excluded-fixture construction.
It does not establish generic map semantics, computational advantage, an exact
digit predicate, a decoder, or subexponential ECDLP runtime. [pcc:unverified]

# References

- Bostan, Morain, Salvy, Schost, *Fast algorithms for computing isogenies
  between elliptic curves*, Math. Comp. 77 (2008), arXiv:cs/0609020.
- H116 frozen plan: `../../FORMAL_SCALAR_JET_PLAN.md`.
- Full experiment note: `../../FORMAL_SCALAR_JET_EXPERIMENT.md`.
