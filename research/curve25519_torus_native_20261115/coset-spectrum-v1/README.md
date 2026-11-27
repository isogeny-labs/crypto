# Curve25519 coset-spectrum assessment

**Record correction:** initial8d8628 omits required declaration hashes and
fails PCC verification. The correctly bound companioncd1904 passes local
verification; signed standing remains unverified. The original record stays
unchanged and remains an active gate blocker pending a supported correction
path. Read [the erratum](record-correction-v2/ERRATA.md). [pcc:unverified]

[Two-page report](report.pdf), [full written review](REVIEW.md),
[sources](SOURCES.md), [formal fidelity map](PROOF_MAP.md),
[editable mechanism diagram](diagram.dot),
[same coefficient-selection problem, current update](../../../problems/curve25519-section-coefficient-selection-v2.md).

The candidate replaces explicit coset-product construction by six unknown
bounded labels in a guarded characteristic-polynomial identity. Its written
representation bound is O(log q+log k) at fixed arity; this is not a root-
solving bound. The baseline point target circuit already has logarithmic
index/power chains. The candidate algebra has dimension6; the coefficient
solution algebra's dimension D is unknown. [pcc:unverified]

The finite Lean control checks a nonzero square-zero2x2 matrix over ZMod101
whose characteristic polynomial is T^2. It identifies the reducedness gap
in a general matching-spectrum inference. Full spectral extension/projection,
native implementation, original-curve proof binding and a concrete coefficient
elimination algorithm remain open. This assessment launches no experiment,
solver campaign, paid worker or benchmark. [pcc:unverified]

All preceding source, original criteria, native evidence, report bytes and
claim FILES retain their bytes. The observed PR5052 ledger registration gap is
recorded separately, without replay or a claim of repaired history. The previous
full archive's trusted publication remains a separate dependency.

Records: finite matrix endpoints PCC-20261123-8d8628; written assessment,
inspected report and retained sources PCC-20261123-5a6ca9. Maintained standing
reports unverified for the formal record; the interpretive record has no
independent signed verdict. [pcc:unverified]

Current publication: PR5071; preceding evidence PR5052 is merged. The owner
reference inside the pinned problem-v2 records its earlier writing context.
