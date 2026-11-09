# Skeptical Lean refutations and exact Semaev census

Lean4.35.0 now checks actual torsion-boundary witnesses, a direct r=2
source correction in FST2010, the rational-point strengthening of Semaev's
converse, and nonregularity of a genuine F25 symmetrized descent. The proofs
use actual elliptic point groups and intermediate fields. The qualified BK
theorem, Semaev's algebraic-closure theorem, and generic FGHR regularity
are not refuted; their exact domains and remaining bridges stay explicit.
[pcc:unverified]

The torsion-only proof snapshot audited74declarations with0failing and
0trusted-native exceptions. The current characteristic-torsion follow-up
audits134declarations; the original41-declaration and intermediate96/116-declaration receipts are
retained as earlier snapshots. Counts include generated equation lemmas,
not134new mathematical results. Both proof attempts and final receipts are
retained. The torsion snapshot's unsigned PCC records are `PCC-20261109-44730b` and
`PCC-20261109-5bd3a4`. Earlier packets omitted required declaration hashes;
they remain as superseded records, and their failed metadata check is
preserved. The Lean kernel audit itself passed. [pcc:unverified]

CI subsequently found that superseded packets still participate in
`verify-all`. Their missing hashes were repaired in the active unsigned
packets with append-only correction events; exact original bytes are retained
under evidence/metadata-originals/. The mathematical statements were not
changed. See evidence/metadata-binding-amendment.json. [pcc:unverified]

The follow-up adds actual BK excluded-boundary torsion and a direct FST2010
r=2 source correction. For Algorithm3.3(1)'s E/F7:y²=x³+x, Lean establishes
#E=8, the source supersingular trace condition, a rational point of order2,
and nonrational geometric2-torsion; nevertheless the **actual field**
F7(μ2)/F7 has degree1. This contradicts the literal degree2 sentence on
p239 under Definition2.1. The primary page was visually checked.
[pcc:unverified]

See [PRIMARY_AUDIT.md](PRIMARY_AUDIT.md) for the exact statements and proof
bridges, [the torsion report](reports/proof-20261109-torsion/report.pdf), and
[pcc-torsion-records.json](pcc-torsion-records.json) for unsigned claims.
The entire44-unread-label backlog and all239source records, including170
primary-publication records, are tracked in [UNREAD_BACKLOG.md](UNREAD_BACKLOG.md)
and [paper-audit-backlog.json](paper-audit-backlog.json). Those are metadata
counts; untouched papers are not labelled audited. [pcc:unverified]

## Exact counterexamples

On the nonsingular F2model y²+xy=x³+1, Lean proves #E(F2)=4,
`#E[2](F2bar)=2`, and all geometric2-torsion rational, yet2does not divide1.
The original BK source requires r²torsion points, so a set-inclusion
translation also needs a characteristic/rank qualification, which the actual
corpus node already retains as gcd(r,q)=1. This is a negative control of
that exclusion. Every actual
μ2-valued map on this nontrivial geometric torsion group is pointwise
degenerate, because the actual μ2value type has1element. This refutes
unrestricted pointwise nondegeneracy, not group-scheme Weil duality or
qualified pairing constructions. Packet92d64dremains unsigned. See
[CHARACTERISTIC_TORSION.md](CHARACTERISTIC_TORSION.md),
[actual Lean objects](lean/CharacteristicTorsionBoundary.lean) and
[proof report](reports/proof-20261109-characteristic/report.pdf).
[pcc:unverified]

The binary follow-up proves the full ordinary characteristic-two S3
equivalence over an algebraically closed field, using actual Mathlib elliptic
points and all repeated-coordinate/two-torsion cases. It also checks a domain
negative control overF2: a true zero-sum triple has binaryS3=0 while the
short-Weierstrass expression is1. This refutes substituting a formula on the
wrong model. Claim PCC-20261109-517cb7 remains unsigned. See
[BINARY_S3.md](BINARY_S3.md), [Lean proof](lean/BinaryS3.lean),
[actual counterexample](lean/BinaryMisapplication.lean), and
[proof report](reports/proof-20261109-binary/report.pdf). [pcc:unverified]

The solver-control follow-up proves the actual algebra equivalence
K[x,y]/(xy,x+y−c) ≃ K[t]/(t²−ct), length2, and a closed F5support1/length2
witness. All16preregistered msolve0.6.5 calls pass their exact known-answer
checks, including the prior degree3control. The expanded project gate now
audits96declarations with0failing/0trusted-native exceptions. Claim
PCC-20261109-ee57d0 is unsigned. See
[solver multiplicity study](../solver_multiplicity_audit_20261109/README.md)
for the frozen protocol, raw outputs and report. [pcc:unverified]

The retained msolve preflight also announces k^344 but prints an integer
with a different F5residue for k=2. Lean proves the unequal residues and
out-of-int32-range arithmetic; pinned source casts floating pow before
field reduction. This source/log audit is separate from a formal C/libm
execution proof. Claim PCC-20261109-c7c353 is unsigned. See
[source observations](../solver_multiplicity_audit_20261109/SOURCE_OBSERVATIONS.md).
[pcc:unverified]

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

This adds formal evidence to items1,2,3and9 and primary-source review to
items6and10. It does **not** complete the fourteen-item
request. The original hypotheses and all gaps are retained in the
[acceptance map](../theorem_audit_20261108/README.md#exact-acceptance-map).
The table here records the current delta; earlier components remain valid
only at their already-stated scope. [pcc:unverified]

| Item | Current status and concrete remaining obligation |
|---|---|
| 1 Semaev | Partial: general short and ordinary binary S3 equivalences now proved with all repeated-coordinate cases in actual Mathlib point groups. Rational-only converse and wrong-model substitution refuted. Bounded S3/S4/S5 native census completed. General resultant recursion and degree proofs remain open. |
| 2 BK | Partial: original r²point-count formulation read; actual excluded-case geometric3-torsion witness even with9dividing#E. New F2/r2actual geometric torsion has2points all rational, refuting unrestricted set inclusion despite2notdividingq−1. The actual node already excludes this by gcd(r,q)=1; original r²condition is not refuted. Full qualified Frobenius proof and all-p<500 census remain open. |
| 3 Supersingular | Literal all-prime k=2 assertion refuted at r=2 using the source trace definition and actual groups. FSTp239 directly checked using actual cyclotomic degree1 versus nonrational2-torsion. Corrected odd-prime proof is conditional on Hasse; general Hasse/classification and binary/ternary census remain open. |
| 4 Pairings | Partial: cyclic restriction obstruction, actual cyclotomic/full-torsion distinction, and actual geometric F2torsion/μ2subtypes proving every point-valued map degenerate at characteristic order. Group-scheme duality is not refuted. Actual qualified Weil/Tate constructions, nondegeneracy and MOV/FR calculations remain open. |
| 5 Diem size | Partial: stable branch locus fails actual Condition2.7. Scheme/ramification bridge and size bound remain open; no counterexample to the qualified bound. |
| 6 Gaudry | Original2000 §§2–3 read; fixed-genus whole-Jacobian limit confirmed as the claim's scope. Actual higher-genus census and lower-order error remain open; small-q error alone is not a refutation. |
| 7 Subgroup smoothness | Partial: abstract confinement proof only. A faithful higher-genus Jacobian family and subgroup proportions remain open. |
| 8 Bézout | Partial: actual bivariate quotient equivalence and support1/length2 Lean witness; all16msolve multiplicity/control calls pass. General n!dⁿ bound and application-specific decomposition systems remain open. |
| 9 Gröbner heuristics | Universal square regularity reading has this Lean counterexample; generic variants and JV remain open. Native msolve controls passed, but those heuristic systems are not yet measured. |
| 10 Walks | Actual PMF fixed-support nonmixing family, actual finite hash-graph variance correction, and exhaustive3930-support mixing/82034-map collision census now pass scoped checks. Random-support averaging, nonuniform p_i and limiting collision constants remain open; qualified Hildebrand is not refuted. See ../walk_audit_20261109. |
| 11 Standards | Screening hypotheses reviewed. Large-order anomalous Smart fixture/proof and matched rho benchmark remain open. Screened NIST recommendation is not refuted. |
| 12 msolve/F4 | Partial static audit, exact native GB/staircase/count fixtures, and a Lean-backed preflight coefficient discrepancy/C conversion boundary. No faithful C/libm-to-F4Run refinement, full parameter-map or row-trace certificate yet. |
| 13 Independence | Separate Rust S3 arithmetic now matches the complete six-prime C++ population:942models/4188162triples/244236zeros,0directional discrepancies. New actual quadratic-field Lean foundation gate11pass. The single separate-host manual dispatch returnedHTTP404, with response retained; no remote execution is claimed. See ../independent_s3_validator_20261109. |
| 14 Negative control | Prior Lean lower/upper degree certificates, production Rust control, and actual msolve degree3known-answer run pass. No retroactive claim that older censuses used this control. |

The actual higher-genus Jacobian, pairing, general geometric supersingularity and C-refinement
bridges are substantial missing formalizations, not assumed lemmas. A proof
about an abstract group or a chosen polynomial list does not discharge them.
No unsupported claim node is promoted. PCC records remain producer assertions
until a separate signed verdict. [pcc:unverified]

Sources: [sources.json](sources.json), including exact primary BK/FST/Gaudry/Hildebrand/FGHR hashes/read scopes;
the KB service remains unavailable. Reports and plots:
[reports/run-20261109-native/](reports/run-20261109-native/).
Validation receipts and inherited repository-wide check limitations are
listed in [CHECKS.md](CHECKS.md).

Integration remains blocked by divergent immutable claim ledgers on the
parent branch and main. Neither history was rewritten or selected; exact
prefix counts/revision are in CHECKS.md. The proof/census artifacts remain
reviewable on the separate stacked branch. [pcc:unverified]
