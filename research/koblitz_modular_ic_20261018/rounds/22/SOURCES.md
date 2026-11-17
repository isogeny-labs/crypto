# Sources and provenance

Scientific inputs are frozen at69f67409f5fe6b525c23b000edea047dd38fcf83.
PR2215 subsequently merged into isogeny-labs/crypto; this follow-up integrates
on merge3c87193e7c754e9abefa401b7cd0974d0708b3b1. These are distinct source
roles. R01–21 evidence is preserved historically, not resealed under R22.

Repository sources reviewed before choosing the calculation:

- Round19 G25 actual weighted CM row maps and their orbit orientation.
- Round20 G28 integral recovery, source-image geometry and controlled-pole nulls.
- Round21 G31 regular-cycle denominator5, Q40 four node187 models,
  CM.md pole-geometry obligations, SATURATION.md and NEXT.md.
- carrier16::circuit(false), the existing Tate/QR incidence and registered units.
- The four Q8 source programs and complete coefficient-descent residuals.
- The pinned binary elliptic arithmetic, independent reverse verifiers,
  identity-certificate AST and Lean audit implementation.

Primary external references consulted:

- Singular official source, normal.lib, integralbasis.lib, brnoeth.lib and
  elim.lib, in the installed4.4.1 native backend; upstream repository:
  https://github.com/Singular/Singular . The library APIs inform the
  normalization, integral-basis and places calls. The recorded saturation
  exponent display error is retained; no exponent certificate is claimed.
- Sutherland's independent modular-curve models:
  https://math.mit.edu/~drew/X1_optcurves.html . They are a possible model
  crosscheck only. No identification with this incidence component has
  been certified, so no independent model is substituted for its geometry.
- Stacks Project, valuation-ring specialization and valuation rings:
  https://stacks.math.columbia.edu/tag/01J8 and
  https://stacks.math.columbia.edu/tag/00I8 . These support the written
  valuative bridge; the source-specific coupled descent argument is written
  in SATURATION.md and its algebraic implication is proved in Lean.
- Pinned mathlib matrix, finite-sum, ring and characteristic2 APIs from
  the project manifest; no unpinned theorem source is imported.

The native implementation retains full Smith transformations and integer
inverses rather than citing software output without a certificate. No novelty,
IC cost, deployed-curve or end-to-end solver claim is made. The exact source
manifest, failed attempts, backend package lock and completed test/audit receipts
are bound by ledger.json and runs.sha256 after validation.

Delivery integrated main8eef1252714097e0b8279c0bf55b82ed9e49943d as the
second parent of4008aeb19e1. The earlier main checkpointbaa1951 is included
as an ancestor;it was incorrectly labelled the merged tip in the first seal. The study algorithms,Lean sources and binary elliptic implementation
were unchanged by that integration. Main's artifact/CI/proof-gate changes are
adopted. Final source_revision is a committed pre-seal source frame,verified
against every pin;later CI checks that historical frame rather than assuming
unrelated future main changes reproduce its bytes. The ready PR remains open
with auto-merge disabled under this task's original instruction.

The written closed-immersion consequence of the equivariant retraction uses
Stacks Project Lemma41.7.2 (proper monomorphism iff closed immersion):
https://stacks.math.columbia.edu/tag/04XV . This applies after the explicitly
stated CM-morphism/descent interpretation;it is not a new Lean schemes theorem.

SS5 follows the existing complete base support rather than a new search.
Its source is the exact unchanged polynomial-ring prefix of cm22-incidence.sing
with residual indexes48,52,238,240,242 and unit indexes0,11,1,1,19,36,64,74
from cm22-input-circuit.json. The native Singular linalg.lib mat_rk routine
operates on the constant F32 Frobenius matrix. The finite-reduced-algebra
residue-degree interpretation is stated in CM.md;it is not a novelty claim
or a fully formalized schemes theorem. REPLAY_SS5.sh keeps this native
certificate separate from the incomplete global normalization calculations.
