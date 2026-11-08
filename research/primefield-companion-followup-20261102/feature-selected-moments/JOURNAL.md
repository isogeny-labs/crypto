# T-1934 component journal

## Plan
### What was done (commands, files, features)
Read latest shared log, exact projector and circuit. Sources checked: Bostan-Lecerf-Schost ISSAC2003, sections3-4 and plain transposed multiplication (https://specfun.inria.fr/bostan/publications/BoLeSc03.pdf); nonlinear precomputation is not removed by linear transposition. KB unavailable. Own component reserved; shared publication scopes held by T1931.
### Decisions made (and why)
Compute two selected coefficients of the terminal quotient product by sparse indexed pairing. Derive full-rank bilinear boundary for arbitrary dense operands. Earlier node/predicate/prefix computation stays charged and may already be dense or equal the final support. Do not claim the full indicator is absent from all intermediates. Local build90s/check60s, up to4 revisions; no cloud/timing ranking.
### Open threads (unfinished, pick up next time)
Kernel, transcript and public equality checks; record large refusal before terminal pairing.
### Errors (what went wrong, how it was handled)
Shared scope conflict: use this component journal until available, no shared writes.

## Qualification and publication
### What was done (commands, files, features)
Validation01 passes1408 kernel/22 rank/144 transcript checks and29 matched public outcomes. Public selected685238 monomial pairs versus689538 reference; counts omit other costs. Large transcript still refuses at14621 pairs before terminal probes. Twelve copied files/17 dependencies bound; report inspected. PCC-20261108-f58ab7 unsigned;510-entry chain verifies.
### Decisions made (and why)
Qualify terminal contraction only. Earlier prefix can equal final indicator; no whole-circuit compression claim. Added probe cap and full-rank bilinear boundary. Acquire shared scopes after T1931 releases, then publish.
### Open threads (unfinished, pick up next time)
Push selected functionals into structured predicate-power/section computation with upstream costs explicit.
### Errors (what went wrong, how it was handled)
No compile/assertion failure. Report-writing command initially used a relative workdir missing its leading slash; process creation failed before writes, then retried with the correct directory. Large cap refusal retained. Synced artifact-bot c742b4cdb87d and renewed all publication scopes.
