# Unified finite-place norm projection

[Report](reports/report.pdf), [protocol](PROTOCOL.md), [proof map](proof-map.json),
[source plan](sources.json), [source review](evidence/source-review.json),
[verification](verification.json).

Main: `EllipticFinitePlaceProjectionAudit.actual_elliptic_finite_place_projection_certificate`.

The [unified finite-place projection proof](README.md)
proves the norm/tame-symbol projection formula at every actual finite
nonsingular point of a general Weierstrass model over any field. The base
place is the official normalized (X-a)-adic valuation on K(X). The fiber is
the actual full set {P,-P}, with duplicates removed. Its cardinality and
the valuation pullback exponent are derived: two points with exponent one,
or one point with exponent two. The normalized order of the norm equals
the sum of point orders over that distinct fiber. [pcc:unverified]

Proper base residues fix K, are surjective, have the maximal ideal as kernel,
and agree across models and choices of point above a. The proof derives the
projection formula for every nonzero function and rational base unit in both
the classical and corrected inverse conventions. It supplies no assumed
projection, residue compatibility, or arbitrary square cancellation.
Actual F5 controls give full split-fiber value 1 versus one-point value 2,
and distinct fixed-fiber value 4 versus duplicated-pair value 1. Actual
binary split and ramified models give identical base maps for all arguments,
despite different fiber sizes; a ternary fixed-point case also passes.
These controls refute omitted hypotheses and incorrect implementations;
they do not refute a qualified source theorem. [pcc:unverified]

Local **PCC-20261116-3a0b23 PASS at 2026-11-16T04:06:31Z**, signed status unverified.
The whole-project gate checked 44 declarations in eight modules, with zero
failures or trusted native evaluations. All 299 formal and 17 artifact pins
match; 218 custom modules are actually imported, including eight new modules.
Seven source states are retained, including the failed fourth state. The
full 6055-character main header has SHA-256
`a5055bbf5e43e7fb1aee7caf9718cf6059ea60c4e97098d0525015538c8ab110`.
Its separate procedural binding check reports one complete statement, zero
prefixes and zero unresolved statements after the completed proof gate.
The lossless archive restores all 175 files; the one-page report was read
and visually inspected. The 64631-byte source plan preserves the full inline
header. The earlier unsupported reference-only metadata candidate and its
compatibility repair remain outside the frozen proof pins. [pcc:unverified]

Sources checked: six KB calls and both Conductor attempts were unavailable
(exit 127); bounded local and primary-source review supplied the fallback.
Gille–Szamuely PDF pages 218–220 (printed 204–206) were read and individually
viewed: complete bounded proof bodies of 7.4.1–4 and Remark 7.4.5. Earlier
norm, base-change, P1-complex and external dependencies were not all read;
7.4.6 was only partially displayed. Known methods and inherited proofs are
credited. No novelty claim is made. [pcc:unverified]

General closed points, residue extensions, inert fibers, scheme lengths,
P1 and global reciprocity, arbitrary divisor representatives, the isogeny
comparison, general descent, canonical Tate pairing, general MOV/Frey–Rück,
and completion of all fourteen items remain open. Verification used the
same host and prover; independent compiler, parser and physical-host
assurance remain open. [pcc:unverified]

Publication: ready [PR #4389](https://github.com/isogeny-labs/crypto/pull/4389). Remote proof `871c5f4f5c9545a78851dd1475255379c74621ee` and local receipt `3b68eaf20b6faa2c01edf6892089950c955bd49e` share tree `a2297e6d9a1fcb8ec6e451d07b2140015e86dde9`. Anonymous fetch completed and a fresh read matched the exact head, title, body and ready state. All 41 pinsets / 4377 entries match. Current-head CI, review and signing are pending. [pcc:unverified]
