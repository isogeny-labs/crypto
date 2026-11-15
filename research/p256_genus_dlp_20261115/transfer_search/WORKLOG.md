# Transfer search work log

## 04:58Z — Literature and first implementation

### What was done (commands, files, features)
- Confirmed framework PR #4209 merged; started local branch codex/p256-transfer-search.
- Recorded the original speedup objective, controls, resource bounds and source review
  in PROTOCOL.md before running the new native computation.
- Read Smith section 8 in the primary PDF: Proposition 5 assumes absolute simplicity,
  which our cover Jacobians do not have. Reviewed existing P-256 degree-11 walk,
  campaign, task and Vélu implementation rather than treating them as unexplored.
- Implemented all-kernel elliptic-plane containment, sextic Richelot enumeration,
  and an actual-field degree-three division-polynomial screen.

### Decisions made (and why)
- Test the six non-pair kernels through polarization, rather than assuming a
  rational Lagrangian must yield a smooth genus-3 Jacobian.
- Continue with a concrete elliptic escape if its rational kernel exists.
- ecdb and EKS access are unavailable (no token/AWS credentials/CLI); KB wrapper
  exits 127 and Conductor is absent. Primary/repository fallback is recorded.

### Open threads (unfinished, pick up next time)
- Inspect first outputs; implement surviving maps; retain speedup requirement as
  open until a destination solver and matched complete-cost evidence exist.
- Report, graph/PDF, proof/source evidence, claim registration and regular PR.

### Errors (what went wrong, how it was handled)
- First build needed explicit conversion from factor-degree u32 to usize for
  permutation indexing. Corrected before reading any mathematical output.
- A guessed isogeny-document path returned 404; the existing native Vélu module
  supplies the precise formulas and limits. No unavailable source was cited.

## 05:02Z — Kernel result and rejected initial elliptic candidate

### What was done (commands, files, features)
- The finite combinatorics agree with the proposed elliptic-plane coverage;
  the initial scratch output is retained for the error audit, not promoted.
- Small-field exhaustive Vélu maps passed, but the actual-field constructor
  rejected the first reported root before any subgroup transfer was accepted.

### Decisions made (and why)
- The reused Poly::xgcd returns (Bezout-left, Bezout-right, gcd), not gcd first.
  Correct the caller to take tuple element two and add explicit factor-divisibility
  checks. The constructor's polynomial check caught the invalid candidate.
- Keep all earlier pinned sources/evidence unchanged. The error is in this new
  caller. Initial elliptic-root output is invalid and is never a transfer result.
- Read the EKS skill: direct AWS access is absent, but GitHub dispatch is an
  alternate route. Do not claim measurement infrastructure is blocked solely
  from absent local AWS credentials.

### Open threads (unfinished, pick up next time)
- Complete the actual-field map and dual checks, identity receipt, final bounded
  report and PR. The destination solver/speedup obligation remains unresolved.

### Errors (what went wrong, how it was handled)
- A compile comparison required Some(1), since polynomial degree is optional.
- Actual-field tests failed on a non-kernel candidate extracted from a Bezout
  coefficient. Correct the tuple selection; preserve the failed initial output.

## 05:20Z — Candidate and bounded exclusions checked

### What was done (commands, files, features)
- Four release tests, formatting, strict new-package Clippy and two saved-output
  comparisons passed. Raw checks and both outputs are retained. [pcc:unverified]
- Implemented the mixed E-factor image in J(C3), checked its full P-256 order,
  eigenvalue relation, six signed images, addition and public scalar cases.
  The destination folded-rho solver and complete-cost run are not implemented.
  This is a candidate, not a demonstrated speedup. [pcc:unverified]
- All nine rational C3 Lagrangians contain one elliptic two-torsion plane;
  the written polarization argument makes their induced ppav quotients split.
  The C2 Richelot kernel is uniquely rational and split. [pcc:unverified]
- F2 has a native order-three witness excluding a rational order-n subgroup by
  Hasse. Standard degree-three Vélu forward/dual, actual-field image order and
  scalar recovery passed. No full F2 order was computed. [pcc:unverified]
- PCC-20261115-e111c2 pins the final audit and two identity receipts. It remains
  unsigned. The initial invalid elliptic scan is explicitly excluded.

### Decisions made (and why)
- Keep the user's speedup objective open; separate ideal collision-step savings
  from arithmetic, folding, cycle handling and complete solve costs.
- The E x E eigenline is a control: its two-addition step model does not gain
  from the ideal square-root-of-three orbit factor. No novelty claim.
- Read Galbraith's primary ECC2012 slides on orbit walks and cycle problems;
  the IACR PKC2010 paper copy returned 403, so do not cite it as read.

### Open threads (unfinished, pick up next time)
- Finish and inspect report PDF; freeze source/proof evidence and publish the
  regular follow-on PR with the cost/solver dependency visible.

### Errors (what went wrong, how it was handled)
- The first PDF build interpreted the row's [-1] as a LaTeX spacing option.
  Use explicit bracket delimiters and rebuild before inspection/pinning.

## 05:26Z — Cost model source and cycle counterexample

### What was done (commands, files, features)
- Read Galbraith's primary book section 14.4, Theorem 14.4.3 and section
  14.4.2. It explicitly charges both group arithmetic and representative cost.
  Downloaded PDF SHA-256: ca75ee62b96daa0d46faaa2d4dc28205683455ad3a2a48586b9b39299c25022d.
- Added actual-image controls D+SD=-S^2D and D+S^2D=-SD: an endomorphism-only
  walk stays inside the same signed orbit. The four-test suite, Clippy and
  saved-output comparisons still pass; pinned output bytes are unchanged.
  [pcc:unverified]
- Added the specific open problem for the folded solver and net cost, with
  explicit acceptance criteria, controls and no active background run.

### Decisions made (and why)
- Keep the candidate and the cycle counterexample together. The next solver
  must use independent increments and short-cycle handling.
- Enlarge the route diagram into two readable columns; the first layout was
  too small in the PDF. Finish inspection before pinning the report.

### Open threads (unfinished, pick up next time)
- Final report inspection, source/execution records, commit and follow-on PR.
- Speedup remains unestablished; no benchmark was launched.

### Errors (what went wrong, how it was handled)
- Corrected a duplicated citation phrase introduced while adding the primary
  book reference. No mathematical output changed.
