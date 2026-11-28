# Actual geometric seven-torsion: 49 points and rank two

Lean proves that every nonsingular general Weierstrass curve over an
algebraically closed field with seven nonzero has **49 actual points killed
by seven**, **48 points of exact order seven**, and an actual two-element
basis over `ZMod 7`. Characteristics two, three and five are included. Counts,
rank and the multiplication formulas are derived from actual points.
[pcc:unverified]

The literal Mathlib `W.preΨ' 7` satisfies `7P=0` if and only if it vanishes at
the x-coordinate of an actual nonsingular affine point. Both directions hold
in every characteristic, without a global discriminant premise for this point
criterion. Actual tripling and fourfold multiplication are proved through
the official addition-and-subtraction map on symmetric x-coordinates and
actual duplication. All division domains are justified. Equality of the
actual x-representatives gives `4P=3P` or `4P=−3P`; the first forces `P=0`,
and the second gives `7P=0`. [pcc:unverified]

Write `D=Ψ2Sq`, `T=Ψ3`, `H=preΨ4`, `C=6X²+b2X+b4` and `U=preΨ5`. Lean
proves the literal recurrence `V=preΨ7=UT³−D²H³`, its structured derivative
`V'=7D·J`, and small root-derivative identities over every commutative ring.
Actual nonsingularity excludes zeros of `D`, `T` and `H` at a `V`-root.
When seven is nonzero the identities derive separability. The official degree
theorem gives24 distinct abscissas, with two distinct actual ordinate points
above each. An explicit equivalence identifies the actual killed-seven
subgroup with those fibers plus infinity, giving49. Its annihilation supplies
the canonical `ZMod 7` action; actual cardinality gives rank2 and constructs
the basis. The48 exact-order points are counted separately. [pcc:unverified]

Actual controls bind the characteristic assumptions:

- The credited curves over the algebraic closures of F2, F3 and F5 each have
  a new actual seven-torsion kernel of size49 and rank2. Their curve identities
  and discriminants are credited, rather than presented as new populations.
- Over the actual algebraic closure of F7, `y²=x³−x` is nonsingular with
  discriminant1. Its literal seventh polynomial is nonzero because its value
  at0 is−1. The leading coefficient7 vanishes, giving degree at most23.
  The distinct-root bound and actual two-point ordinate fibers therefore give
  a whole point kernel of size **at most47, hence not49**, and module rank
  different from2. This is a strict bound, not an exact kernel-size1 claim.
  It disproves deleting coprimality from the rank/count theorem.
  [pcc:unverified]

The negative control does not refute BK's qualified point-count-divisibility
statement, identify a nonreduced group-scheme length, or establish a
supersingularity definition. The general-model rank gap at7 is closed;
determinants for primes≥5, ranks for general primes≥11, canonical Weil
pairings and the complete all-k p<500 census remain open. Established methods,
the actual three/five-torsion work, root/ordinate/module arguments and official
degree/duplication/addition formulas are credited. No novelty or whole
fourteen-item completion claim is made. [pcc:unverified]

The primary scope is Galbraith ch9 PDF16–17/36, printed180–181,
section9.8/Definition9.8.5/Lemma9.8.6/Theorem9.8.7, freshly read in fulltext and
individually viewed in the immediately preceding five-rank study and credited
exactly here. Its stated polynomial theorem uses short models and excludes
characteristics two and three. This n7 extension is proved directly; it is not
attributed to that passage. No newly broadened or whole-book reading is
claimed. Copyright sources remain local. The pinned official `AddSubMap`
definitions and actual theorem were read; its opening TODO is stale relative
to the later proved theorem. Exact source and credit details are in
[source review](evidence/source-review.json). [pcc:unverified]

Eleven new modules, ten concrete and one modern public-import adapter, reuse22
frozen custom modules through one local dependency with zero copies or edits.
All45 predecessor pins match before and after every build and gate. Eight
exact states preserve the failed state07 and seven successes. Earlier unused
tactic and section-variable advisories were corrected without waivers.
[pcc:unverified]

The whole tracked/transitive gate passes59 theorems with zero failures and
zero trusted-native exemptions. The prior50-theorem gate is credited
separately. Final new-source warnings are zero; seven inherited advisories
and one frozen auditor deprecation remain. The complete1,611-character header
matches with one complete result, zero prefixes, zero unresolved cases and an
empty omitted suffix. The lossless archive preserves172 files in68 objects
and23,872 bytes, with exact roundtrip checks and five gate files.
[pcc:unverified]

See the [closed certificate](lean/SevenCertificate.lean),
[proof map](proof-map.json), [source plan](sources.json),
[protocol](PROTOCOL.md) and [report](reports/report.pdf).

```sh
python3 research/seven_torsion_rank_audit_20261115/restore_attempts.py /tmp/seven-rank-attempts
```

Four correct KB wrapper calls and Conductor were unavailable with exit127
before processing/code; bounded repository/official/primary fallback was used
without an absence claim. Current default limits and standard foundations are
used, without holes, custom axioms, native proof evaluation, new global flags,
waivers or frozen edits. Python handles documents and provenance only. No
new native cryptographic experiment, paid compute or agent upload was used.
The written-header check is procedural and shares the host; it does not
establish parser/compiler or physical-host independence. Signed standing
remains unverified. [pcc:unverified]

Exact local **PCC-20261115-919b86** replay PASS at 2026-11-15T18:58:37Z; signed standing unverified. See [verification receipt](verification.json). [pcc:unverified]

Publication: existing ready [PR3743](https://github.com/isogeny-labs/crypto/pull/3743), remote proof `55719ef52054e3ccd14bfd644f3c6e8c6cdca0db` and local receipt `77442d15a1c7ccaf486a7e308aa8aa33ba80c020` share tree `4c37a7d2a1d9c87618333afbf6e34707d8da9315`. Anonymous fetch completed; fresh PR head/body/open/ready matched. Publication metadata is outside the frozen pinset. [pcc:unverified]
