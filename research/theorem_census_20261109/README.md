# Skeptical Lean refutations and exact Semaev census

Two concrete witnesses now compile in Lean4.35.0. The first disproves the
rational-point strengthening of Semaev's converse. The second constructs a
genuine degree-two extension of F5, lifts a nonzero order-three target on a
nonsingular curve to it, and proves that its two-coordinate symmetrized
descent is nonregular. These statements do not refute Semaev's actual
algebraic-closure theorem or a generic-instance regularity assertion.
[pcc:unverified]

The final repository proof gate audits41declarations with0failing and
0trusted-native exceptions. That includes generated equation lemmas, not
41new mathematical results. Both proof attempts and final receipts are
retained. The current unsigned PCC records are `PCC-20261109-44730b` and
`PCC-20261109-5bd3a4`. Earlier packets omitted required declaration hashes;
they remain as superseded records, and their failed metadata check is
preserved. The Lean kernel audit itself passed. [pcc:unverified]

## Exact counterexamples

1. On `y²=x³+1` over F5, `S3(3,1,4)=0`, but there is no F5-point above x=3.
   `rational_only_s3_converse_false` uses Mathlib's actual elliptic point
   group. The curve is nonsingular. The field qualification matters.
2. Let `K=F5[t]/(t²−2)` and `E:y²=x³+x+1`. Lean establishes irreducibility,
   `[K:F5]=2`, `#K=25`, nonsingularity, and the actual base-changed target
   `(2,1)≠O`, with `3(2,1)=O`. For `e1=x+y`, `e2=xy`, the symmetrized S3
   equation is

   `f=(e1²−4e2)4−4(e1(e2+1)+2)+(e2−1)²−4e1`.

   Descent by actual monic-remainder coordinates gives `[f,0]`. Lean proves
   nonregularity both of this sequence and of its top homogeneous parts.
   `universal_descent_regularity_counterexample` bundles the concrete input
   validity and failure. [pcc:unverified]

FGHR2014 Hypothesis3 says that systems from symmetric-coordinate descent
are regular, without an explicit generic-input qualification in that sentence.
Our witness refutes the **universal square-system reading**. Definition5
requires nonzero input equations; §3 also restricts the complexity analysis
to zero-dimensional square systems. The witness violates those conditions,
so it does not contradict the conditional complexity theorem. Discarding the
zero equation changes the square presentation, leaving one equation in two
variables. No claim about generic inputs, random sampling, Joux–Vitse
Assumption2, or the generic ideal-degree claim follows. [pcc:unverified]

This witness uses the short-Weierstrass representation. It does not establish
an Edwards/Jacobi-model counterexample to a representation-restricted
interpretation of Hypothesis3. The universal reading and those narrower
domains must be kept distinct. [pcc:unverified]

The formal bridge includes `symmetrization_over_extension`, the actual
coefficient descent (not an invented list), and Mathlib's
`RingTheory.Sequence.IsRegular`. No custom regularity axiom is used.
The identity record is `IDC1h819303b7fd181c8c`; its formal pointer names
`symmetrization_universal`, and the unchanged repository checker accepts its
32 deterministic evaluation points plus11 extra points. These evaluations
are auxiliary: the Lean proof establishes the polynomial identity.
[pcc:unverified]

## Finite census

The native C++ program checks both implications against actual elliptic point
sums in quadratic extensions. It checks full symbolic degrees, including
when degree exceeds field size; it does not estimate degrees by evaluation.
Every tested model and base-field coordinate tuple is included in the stated
domains. All discrepancy and degree-failure counters are zero.
[pcc:unverified]

| Base fields | Models | m | Exact scope |
|---|---|---|---|
| F5,F7,F11,F13,F17,F19 | Every nonsingular short model | 3 | Every ordered x tuple, y lifts in Fp² |
| F5,F7 | Every nonsingular short model | 4,5 | Full sparse S4/S5 resultants; every ordered x tuple |
| F2,F4 | Every ordinary model `y²+xy=x³+a2*x²+a6`, `a6≠0` | 3,4,5 | Every coefficient choice and tuple; y lifts in F4/F16 |

Raw output: [evidence/semaev-native.jsonl](evidence/semaev-native.jsonl).
Padded universal Sylvester formulas are checked against scalar determinants,
including all eight-coefficient choices over F2. Fixed F5 controls compare
S4/S5 term counts426/43267 and zero counts209/1282. These controls and the
census use this one machine; they are not independent-host validation.
Finite agreement does not prove the general recursion, degree theorem or
binary equivalence. [pcc:unverified]

The run had a before-run Markdown bound, but lacked a committed frozen JSON
preregistration. This reporting omission is retained as a protocol deviation.
`export.json` is an observational export: structural/file validation is
distinct from frozen-protocol acceptance. No before-data freeze or signed
scientific acceptance is asserted. No performance claim is made; clocks are
missing, and no paid cloud resources were used. [pcc:unverified]

## Reproduce

From this directory, with the pinned Lean toolchain and dependency cache:

```sh
cd lean
lake build
lake env lean Refutations.lean
```

From the repository root:

```sh
tools/lean/proof-gate.sh research/theorem_census_20261109/lean
g++ -O2 -Wall -Wextra -Werror -std=c++17 research/theorem_census_20261109/native/semaev.cpp -o /tmp/semaev-audit
/tmp/semaev-audit --controls
timeout 600 /tmp/semaev-audit
```

The exact native sources, binary digest, compiler identity, host observation,
failed Lean builds and both axiom-audit receipts are retained under
[evidence](evidence). The dependency on
`../theorem_audit_20261108/lean` pins the earlier checked actual-group S3 work.
The identity issuer is reproduced with `cargo run --release --locked` from
`identity/`; it imports the unchanged repository checker and SHA256 source.

## Original fourteen requirements

This adds evidence to items1 and9. It does **not** complete the fourteen-item
request. The original hypotheses and all gaps are retained in the
[acceptance map](../theorem_audit_20261108/README.md#exact-acceptance-map).
The table here records the current delta; earlier components remain valid
only at their already-stated scope. [pcc:unverified]

| Item | Current status and concrete remaining obligation |
|---|---|
| 1 Semaev | Partial: general short S3 equivalence from prior increment; rational-only converse refuted here; bounded S3/S4/S5 native census completed. Universal binary equivalence, recursion and degree proofs remain open. |
| 2 BK | Partial: linear algebra core only. Actual E[r]/Frobenius bridge and all p<500 torsion census remain open. Actual theorem excludes r dividing q−1; no refutation established. |
| 3 Supersingular | Partial: embedding-order arithmetic under explicit trace-zero/divisibility assumptions. Supersingularity/Hasse bridge and requested binary/ternary census remain open. |
| 4 Pairings | Partial: cyclic restriction obstruction. Actual Weil/Tate constructions, domains, nondegeneracy and MOV/FR calculations remain open. |
| 5 Diem size | Partial: stable branch locus fails actual Condition2.7. Scheme/ramification bridge and size bound remain open; no counterexample to the qualified bound. |
| 6 Gaudry | Source reviewed; actual hyperelliptic Jacobian census and asymptotic error remain open. Small-q error alone would not refute the asymptotic statement. |
| 7 Subgroup smoothness | Partial: abstract confinement proof only. A faithful higher-genus Jacobian family and subgroup proportions remain open. |
| 8 Bézout | Partial: quotient-length versus support control. General bound and msolve multiplicity comparison remain open. |
| 9 Gröbner heuristics | Universal square regularity reading has this Lean counterexample; generic variants and JV remain open. No msolve measurement yet. |
| 10 Walks | Partial: exact iid label collision identity. Fixed-partition collision/mixing census and qualified asymptotic claims remain open. |
| 11 Standards | Screening hypotheses reviewed. Large-order anomalous Smart fixture/proof and matched rho benchmark remain open. Screened NIST recommendation is not refuted. |
| 12 msolve/F4 | Partial static audit. No faithful C-to-F4Run refinement or trace certificate yet. |
| 13 Independence | Other-language fixed controls/census on this one host. Independent second-host execution remains unavailable. |
| 14 Negative control | Prior Lean lower/upper degree certificates and actual production Rust control pass. No retroactive claim that older censuses used this control. |

The actual higher-genus Jacobian, pairing, supersingularity and C-refinement
bridges are substantial missing formalizations, not assumed lemmas. A proof
about an abstract group or a chosen polynomial list does not discharge them.
No unsupported claim node is promoted. PCC records remain producer assertions
until a separate signed verdict. [pcc:unverified]

Sources: [sources.json](sources.json), with primary FGHR PDF hash/read scope;
the KB service remains unavailable. Reports and plots:
[reports/run-20261109-native/](reports/run-20261109-native/).
Validation receipts and inherited repository-wide check limitations are
listed in [CHECKS.md](CHECKS.md).

Integration remains blocked by divergent immutable claim ledgers on the
parent branch and main. Neither history was rewritten or selected; exact
prefix counts/revision are in CHECKS.md. The proof/census artifacts remain
reviewable on the separate stacked branch. [pcc:unverified]
