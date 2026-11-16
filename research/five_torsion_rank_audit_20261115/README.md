# Actual geometric five-torsion: 25 points and rank two

Lean proves that every nonsingular general Weierstrass curve over an
algebraically closed field with five nonzero has **25 actual points killed by
five**, **24 points of exact order five**, and an actual two-element basis over
`ZMod 5`. Characteristics two and three are included. The count and rank are
derived from actual points; neither is a premise. [pcc:unverified]

The point criterion is stronger in its characteristic scope: for every actual
nonsingular affine point on every general model, in every characteristic,
`5P=0` if and only if the literal Mathlib polynomial `W.preΨ' 5` vanishes at
its x-coordinate. Actual doubling and equal-x group classification prove both
directions. Coprime group annihilation excludes the three-torsion branch;
actual nonsingularity proves the required doubling domains. [pcc:unverified]

The polynomial is the pinned library definition. Write `D=Ψ2Sq`, `T=Ψ3`,
`H=preΨ4` and `C=6X²+b2X+b4`. Lean proves over every commutative ring:

\[
 H=TC-D^2,\qquad U=\Psi_5=D^2H-T^3,\qquad U'=5D(CH-T^2),
\]

and the small identity `T²H=D²(CH−T²)−CU`. At a root on a nonsingular curve,
`D` and `T` are nonzero. If five is nonzero, these identities exclude a common
root of `U` and `U'`, deriving separability. The official degree theorem gives
degree12, hence twelve distinct abscissas. The actual ordinate polynomial has
two distinct roots above each. An explicit equivalence identifies the actual
killed-five subgroup with these fibers plus infinity, giving25. The actual
annihilated subgroup supplies its `ZMod 5` action; cardinality then gives
rank2 and constructs the basis. Exact order24 is counted separately from the
25-point kernel. [pcc:unverified]

Actual controls check the characteristic boundary:

- On the credited curve `y²+y=x³+1` over the algebraic closure of F2, the new
  five-torsion kernel has25 points and rank2.
- On the credited curve `y²=x³−x` over the algebraic closure of F3, it also has
  25 points and rank2.
- Over every field of characteristic5, `y²=x³+1` is nonsingular, with
  discriminant3, while its literal fifth polynomial evaluates to−1 everywhere.
  Its actual five-torsion kernel contains only infinity and has cardinality1.
  This disproves deleting the characteristic guard from the rank/count claim.
  It is not a counterexample to BK's point-count-divisibility condition.
  [pcc:unverified]

The earlier ordinary characteristic-five curve with geometric kernel5 is
credited separately. No finite group-scheme length or supersingularity
definition is identified here. The actual general-model rank gap at prime5 is
closed; determinant at5, rank/determinant for general primes≥7, canonical Weil
pairings and the full all-k p<500 census remain open. Qualified source
theorems survive; established methods and previous proofs are credited, with
no novelty or whole fourteen-item completion claim. [pcc:unverified]

Fresh primary review covers Galbraith ch9 PDF16–17/36, printed180–181,
section9.8 and Definition9.8.5/Lemma9.8.6/Theorem9.8.7, read in fulltext and
individually viewed. That stated polynomial theorem uses short models and
excludes characteristics two and three. This packet proves its n5 extension
directly; it does not attribute an unrestricted theorem to that passage.
Earlier BK1998 printed142–143 review is credited. Selected sections only,
not a whole-book/paper reading; copyright files remain local. Exact hashes,
URLs and credit are in [source review](evidence/source-review.json).
[pcc:unverified]

Ten new concrete modules reuse12 frozen custom modules through one local path
dependency, with zero copies or edits. All31 predecessor pins match before and
after every retained state and gate. Ten states retain six failures and four
successes, including the failed state09 internal exception and recovery parse
errors; no recovery output was accepted. The final explicit singleton
equivalence passes normal kernel checking. [pcc:unverified]

The whole tracked/transitive gate passes50 theorems with zero failures and
zero trusted-native exemptions. The earlier77-theorem gate is credited
separately. Final new-source warnings are zero; seven inherited advisory
lines and the frozen auditor's String.mk deprecation remain. The complete
1,281-character header matches, with one complete result, zero prefixes,
zero unresolved cases and an empty omitted suffix. The lossless archive
preserves197 files in80 objects and23,824 bytes, with exact roundtrip checks
and five gate files. [pcc:unverified]

See the [closed certificate](lean/FiveCertificate.lean),
[proof map](proof-map.json), [source plan](sources.json),
[protocol](PROTOCOL.md) and [report](reports/report.pdf).

```sh
python3 research/five_torsion_rank_audit_20261115/restore_attempts.py /tmp/five-rank-attempts
```

KB and Conductor were unavailable with exit127 before processing/code;
bounded repository, primary and official-library fallback was used without
an absence claim. Default limits and standard foundations are used, without
holes, custom axioms, native proof evaluation, new global flags, waivers or
frozen edits. Python handles documents and provenance only. No new native
cryptographic experiment, paid compute or agent upload was used. The
written-header check is procedural and shares the host; it does not establish
parser/compiler or physical-host independence. Signed standing remains
unverified. [pcc:unverified]

Exact local **PCC-20261115-2cfd10** replay PASS at 2026-11-15T18:18:29Z; signed standing unverified. See [verification receipt](verification.json). [pcc:unverified]

Publication: existing ready [PR3743](https://github.com/isogeny-labs/crypto/pull/3743), remote proof `39ab9da50a5c9a82d55aab3b9c07a0e1a33d7687` and local receipt `8d7b7adbae8bec4c378b696624efdc2b39f664e9` share tree `4bf3bcdcd337726d5b89d346c1f8de5a0ac21565`. Anonymous fetch completed; fresh PR head/body/open/ready matched. Publication metadata is outside the frozen pinset. [pcc:unverified]
