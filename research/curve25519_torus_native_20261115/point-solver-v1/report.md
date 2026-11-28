# Direct original-point replay and the next elimination block

The native Rust adapter exports the unchanged original six-member point
System directly, one SMT variable per original variable and one equality per
sparse row. The public solve API receives Spec/R/Q/limits only. Complete
models are accepted only after exact binding/canonical-value validation,
original System.check and original indexed sign/group replay. Arbitrary
caller-built Systems are outside the public API. [pcc:PCC-20261123-2033f1]

Twelve supplied complete models pass. Two backend cases assert ALL known
original assignments (toy and original-q/k20633) and pass SAT/replay. Neither
is discovery. Native small SAT/UNSAT and output-cap controls also pass their
declared checks. Final three release tests, fmt and strict all-target Clippy
pass locally. These do not establish hosted gates. [pcc:PCC-20261123-2033f1]

Exactly one public p101/q3/k8 query used R=(89,27),Q=(70,41),327 variables,
357 quadratic equations and1139 terms. No unknown values were supplied.
The backend emitted its timeout diagnostic under the declared5000ms cap,
returned no status/model and received no get-value request. Native signal6
and the separate BrokenPipe transport error are recorded, with no retry.
This supplies no absence, hardness, rate or exponent inference.
[pcc:PCC-20261123-2033f1]

The95-source context is unchanged76+unchanged9+new10 inputs in THREE archives,
not a monolithic archive. Eight new compiled hashes and retained executable
pre/post hashes match. All captured streams roundtrip through the54020-byte
native archive; the output-cap case is explicitly truncated. Producer standing
is unsigned/not_covered. Source/evidence were published before this report.
[pcc:PCC-20261123-2033f1]

The point circuit fixes the sum of eightfold images to Q; the section circuit
fixes a raw-R sector. Equal Q does not make their search spaces identical.
Thus the earlier3511-variable section cap and this327-variable point cap are
not a paired performance comparison. See FIDELITY.md. No attack-cost advantage
is established. [pcc:PCC-20261123-2033f1]

The next proposal reuses the existing six-dimensional section interpolation
kernel, rather than another generic adapter. Compile its evaluation/jet rows
for UNKNOWN point coordinates; eliminate coefficients only on guarded pivot
charts, retaining exact fallback equations on unresolved strata. Preserve
target-pole/removable charts, aliases, multiplicities, normalization and the
raw-sector distinction. Extension/projection, solving degree, full cold cost
and useful-yield scaling remain open. See NEXT.md. This is a proposed compiler
operation, not a claimed new theorem or implemented solver. [pcc:unverified]

Classical sources: Cheng, cs/0507026 section2/Theorem1; Snowden Math679 lecture2;
original torus-section-operator. Prior backend context: Ozdemir et al., CAV2024,
Split Groebner Bases for Satisfiability Modulo Finite Fields; pinned cvc5 docs.
The section/subset-sum connection and linear elimination are classical. KB
unavailable; bounded primary-source review does not establish novelty.

The two-page report.pdf and editable diagram accompany this text. No new Lean
endpoint, formal compiler/group/cost theorem, paid job or solver campaign.
Existing trusted-artifact access, immutable-record remedy and journal conflict
remain separately open. All original criteria and published histories stay
frozen. [pcc:unverified]
