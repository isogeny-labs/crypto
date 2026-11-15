# Actual torsion Frobenius modules and the source's F19 exception

Lean constructs Frobenius on the actual subgroup of curve points killed by
every natural `r`, and identifies its fixed-power module with actual torsion
over a compatible finite extension. It also verifies the original BK1998 F19
exception, including its minimal extension degree3. **The arbitrary-prime
geometric rank-two and determinant-q theorems remain open.** This packet does
not claim the full general BK theorem or the full all-k census for p<500.
[pcc:unverified]

The subgroup and its canonical `ZMod r` module are derived from actual point
addition and annihilation. Coordinate q-Frobenius restricts to a linear
endomorphism. It is injective over every ambient field extension, and an actual
linear automorphism when that extension is algebraic over the finite base.
The inverse comes from the official Frobenius field automorphism, without a
basis, rank, determinant or global torsion-finiteness premise. [pcc:unverified]

For a supplied compatible finite field `K` of size `q^k`, actual point inclusion
gives a `ZMod r`-linear equivalence

\[
 E(K)[r]\;\simeq\;\ker(\operatorname{Frob}_q^k-1).
\]

The finite fixed population and its cardinality follow from the actual finite
point encoding and that equivalence. Prime `r` dividing the actual base point
count supplies a nonzero fixed torsion point through the credited Cauchy
construction and injective point inclusion. The field/point descent theorem
and generic matrix argument were already proved and are credited separately;
the new work connects the actual modules. [pcc:unverified]

The module statement includes composite and characteristic-r cases without
claiming rank2. Its guards are explicit: `r=1` gives a trivial actual subgroup,
while `r=0` gives all curve points. The latter is not silently treated as a
finite rank-two torsion space. [pcc:unverified]

The paper's known Remarks1 example is checked on actual objects:

- `E/F19: y²=x³+x+6` is nonsingular and has18 rational points.
- `(0,5)` has exact order18 and generates the cyclic whole group.
- The kernel killed by3 has three rational points and nine geometric points;
  its Frobenius fixed module has cardinality3, so Frobenius is not identity.
- The third-division polynomial factors as
  `3(X−12)(X³+12X²+13X+9)`. The cubic is irreducible over F19.
- Its actual `AdjoinRoot` field has dimension3 and cardinality6859. An algebraic
  embedding and scalar tower into the closure are constructed using the proved
  embedding theorem.
- The actual Frobenius action has exact order3. Every compatible finite
  extension of size `19^k` contains full geometric three-torsion exactly when
  `3 ∣ k`. The constructed cubic field contains all nine points, and every
  positive extension degree with that property is at least3.
- The actual source matrix has determinant `19 mod3`, derived for this
  particular curve rather than supplied as a hypothesis. [pcc:unverified]

The cubic root power law gives `x^(19³)=x` for every relevant x-coordinate.
Actual point geometry then gives Frobenius cubed equal to either a point or
its negative. Kernel-checked finite F3 matrix rigidity and an actual nonzero
fixed vector force the cubed action to be identity. Its nonidentity and prime
order give the full extension criterion. The existing actual geometric
three-torsion basis is credited. Counts refer to points **killed by3**, including
infinity, as in `E[3]`; they are not counts of points of exact order3.
[pcc:unverified]

This confirms the source's stated exception to deleting `r∤q−1`; it does not
refute the qualified source theorem or claim novelty. The earlier F7 all-k
counterexample and binary F2 determinant controls remain separately credited.
Fresh primary fulltext and individual-page visual review covers BK1998 PDF2–3,
printed142–143, of5 pages, DOI10.1007/s001459900040. No full-paper reading or
proof of its probability lemmas is claimed. Copyright primary files remain
local. Five correct KB calls and Conductor were unavailable with exit127 before
processing/code; bounded repository, pinned-library and primary fallback was
used without an absence claim. [pcc:unverified]

See the [closed certificate](lean/GeneralCertificate.lean),
[proof map](proof-map.json), [source plan](sources.json),
[protocol](PROTOCOL.md), [source review](evidence/source-review.json) and
[one-page report](reports/report.pdf). The main header uses transparent aliases
for actual maps and kernels with explicit field arguments. This avoids named
assignments that the frozen receipt regex would truncate; the mathematical
objects and frozen verifier are unchanged. [pcc:unverified]

Ten new modules (nine concrete and one import adapter) reuse24 frozen custom
modules without copying or changes. All55 predecessor source/artifact pins
are checked before and after each retained build and gate. Failed proofs,
the accepted intermediate header with a naming advisory, exact source/config
states and logs are retained. Current proofs use default limits and standard
foundations, without holes, custom axioms, native proof evaluation, new global
flags or waivers. Inherited local proof settings and38 advisories remain frozen.
[pcc:unverified]

Restore the exact provenance with:

```sh
python3 research/bk_general_frobenius_audit_20261115/restore_attempts.py /tmp/bk-general-attempts
```

Python handles provenance and documents only. No new native cryptographic
experiment, paid compute or agent experiment upload was used. The native
written-header check is procedural, without compiler, parser or independent
physical-host refinement. Signed standing is unverified. General rank and
determinant/Weil-pairing necessity, the full all-k p<500 population and the
remaining obligations across all14 items stay open. [pcc:unverified]

Exact local **PCC-20261115-bec94e** replay PASS at 2026-11-15T17:19:12Z; signed standing unverified. See [verification receipt](verification.json). [pcc:unverified]

Publication: existing ready [PR3743](https://github.com/isogeny-labs/crypto/pull/3743), remote proof `0687605f5d88c67747bd8b3adc8015ba52986cfd` and local receipt `3e3cf7e8f748ffbe066ad1111febf22a056fbf42` share tree `ef21e74e47aa45e43907342597fb842a6ba9afd4`. Anonymous fetch completed; fresh PR head/body/open/ready matched. Publication metadata is outside the frozen pinset. [pcc:unverified]
