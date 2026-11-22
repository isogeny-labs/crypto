---
title: "Skeptical theorem audit: concrete Lean witnesses"
date: "2026-11-09"
geometry: margin=18mm
fontsize: 10pt
---

Two concrete counterexamples compile in Lean. On F5, S3(3,1,4)=0 for
y²=x³+1, although x=3 has no rational point. On the nonsingular base-defined
curve y²=x³+x+1 over the genuine 25-element field F5[t]/(t²−2), the actual
symmetrized descent at the lifted nonzero 3-torsion target (2,1) is [f,0].
Lean proves its failure of regularity. [pcc:unverified]

Repository transitive axiom gate: 41 declarations audited, 0 failing,
0 trusted-native exceptions. This includes generated equation lemmas;
it is not a count of41new mathematical results. Signed PCC standing is
pending. [pcc:unverified]

![Formal dependency chain. Every solid step is a Lean declaration; the actual target is lifted to the extension.](proof.pdf){width=100%}

These refute, respectively, a rational-only S3 converse and the universal
square-system reading of FGHR2014 Hypothesis3. They do not refute Semaev's
algebraic-closure equivalence, generic-input behavior, or the conditional
FGHR complexity theorem. FGHR §3 analyzes zero-dimensional square systems,
and Definition5 requires nonzero equations. Discarding the zero equation
changes the square presentation. [pcc:unverified]

The witness uses the short-Weierstrass representation. It does not establish
an Edwards/Jacobi-model counterexample to a representation-restricted reading
of Hypothesis3. [pcc:unverified]

Primary source: FGHR2014, DOI10.1007/s00145-013-9158-5. Its full PDF
SHA256 is recorded in sources.json. Read §§2.1,3,4.1.1, Hypothesis3 and
Theorem4.1. Lean sources are RationalCounterexample.lean and
DescentCounterexample.lean under ../../lean/.
Identity certificate IDC1h819303b7fd181c8c points to the general symmetric
identity proof; its unchanged repository checker also accepts32+11 points.
Lean4.35.0 and Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679 are pinned.
[pcc:unverified]

The native census reports zero false zeros, missing zeros or symbolic-degree
failures in every stated domain. All nonsingular short models over
F5,F7,F11,F13,F17,F19 were checked for m=3; those over F5,F7 for m=4,5.
All ordinary binary models over F2,F4 were checked for m=3,4,5. The program
enumerates every ordered base x tuple and uses actual point sums in quadratic
extensions. It computes full symbolic S4/S5 polynomials using padded
resultants. This is finite observation, not a universal theorem.
[pcc:unverified]

\clearpage

![Recorded S4/S5 tuple populations per base field. Logarithmic horizontal scale; exact counts printed. No sampling uncertainty: the bounded populations are exhaustive. Run: 20261109-native.](tuple-counts.pdf){width=94%}

The raw16aggregate rows are in ../../evidence/semaev-native.jsonl.
Controls include field inverses, a fixed group's associativity, scalar
Sylvester determinants including every F2 coefficient tuple, and fixed F5
S4/S5 term counts426/43267 with zero counts209/1282. One early control run,
one strengthened control run and one broad census execution succeeded. The
export retains all three, with no exclusion, interruption or resume.
[pcc:unverified]

Flow: planned3, attempted3, completed3, not completed0, interrupted0,
resumed0, excluded0. No runtime or speed claim: elapsed clocks were not
recorded. Host: shared selected Linux container, local CPU only, no paid
cloud spend, no independent second host. Protocol deviation: pre-run
PROTOCOL.md existed, but no committed frozen JSON preregistration. The
observational export does not earn frozen-protocol acceptance.
[pcc:unverified]

The original14-item request remains partial. Actual torsion/Frobenius,
supersingularity/Hasse, pairings, higher-genus Jacobians, solver traces,
walk mixing, large-order Smart attack and second-host validation still need
their stated constructions/runs. See ../../README.md for each requirement.
The earlier checked S3 equivalence and production degree control are in
../../../theorem_audit_20261108. No node receives signed promotion here.
[pcc:unverified]

Canonical IC/rho leaderboards, progress timelines and attack graphs were
checked for applicability and left unchanged: these are exact finite
correctness populations and scoped formal counterexamples, not matched
performance comparisons or independently signed corpus updates.
[pcc:unverified]
