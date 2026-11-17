# Quadratic remainder component: development contract

Continue the existing graph/algebraic relation-finder objective in PR4288.
The immediate deliverable is a native, opt-in degree-nine extractor, not a
timing cohort or a change to a production collector. Preserve all prior
sources, measurements, defaults and protocols.

Sources checked: the existing LEAD_EXTENSION_QUADRATIC_REMAINDERS.md,
gaudry_cubic_{symmetric,prepared}.rs, jv_sieve.rs line_poly and
relation_from_roots, jv_sieve_two_large_prime.rs, jv_cover.rs polynomial and
Cantor arithmetic, and large_prime_filter.rs. The February2005 GTT primary
draft https://cacr.uwaterloo.ca/techreports/2004/corr2004-29.pdf sections2/3
was read: signed class coefficients and replay must survive cancellation.
Its hyperelliptic asymptotic theorem is not transferred to this workload.
The existing JV source review covers sections3.2/3.3; new retrieval of
https://eprint.iacr.org/2011/020.pdf returned403, so this continuation relies
on the source's explicit norm identity rather than a new literature claim.
KB unavailable: no MCP tools; scripts/research-kb status exits127 (no
executable). Conductor is also unavailable (command not found).

Before implementation, require the following:

- Degree-nine F=A²-C²H, A=A0+t*s*A1, C=sqrt(s)*B, with the actual source
  convention for sqrt(s), deg(A)<=4, deg(C)=1 and monic degree-seven H.
- Extract exactly seven distinct small-base abscissae. Refuse repeated
  support, wrong source data, exceptional denominators and an Fp-split
  quadratic. Leave full-smooth and existing two-LP behavior unchanged.
- Normalize the residual quadratic U. Compute V=-A/C mod U by polynomial
  extended GCD, and require V²=H mod U. Independently add the seven signed
  small divisor classes and D=(U,V) in the existing Cantor implementation;
  accept only the identity.
- Canonicalize the reduced class D versus -D, retaining orientation as a
  coefficient. Do not identify conjugate classes or call D a prime divisor.
- A bounded graph adapter must bind every source back to this sieve/cover,
  reconstruct class IDs, verify original Jacobian identities again and
  replay every emitted combination through the existing filter.
- Development tests cover a constructed seven-point/quadratic principal
  divisor, sign/canonicalization, malformed inputs, opposite orientations,
  bounded resources and graph cancellation. Attempt actual-cover controls
  separately; a synthetic test is not evidence of natural collision yield.

No timing, sample selection by speed, cloud spend, throughput estimate,
pipeline/rho comparison or complete native cost claim is authorized by this
component protocol. Existing arithmetic hooks remain; bookkeeping coverage
and matched yield/rank/cold accounting remain future measurement obligations.
Local builds/unit tests are development work under isolab-eks/SKILL.md.

Stop at a checked component and a reviewable PR update. A later collection
experiment needs a published frozen cohort and its checkpoint/runner gates.
All producer observations carry unverified PCC standing.
