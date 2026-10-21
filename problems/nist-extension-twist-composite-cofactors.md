# Three NIST extension-twist composite cofactors remain unsplit

- **Status:** `open`
- **Kind:** empirical bottleneck; bounded native factorization search
- **Affected work:** [factorization continuation](../research/nist_skipped_curve_dissection_20261021/factorization/README.md), [original jobs](../backlog/XP-20261021-nist-skipped-curves/README.md)
- **Next action owner:** native factorization continuation
- **Last updated:** 2026-10-21

## Problem

Complete prime factorizations of three degree-2 extension-twist orders are
still unavailable because their remaining composite cofactors were not
split by the recorded PARI and GMP-ECM allocations.

For the exact imported models, let `t=p+1-n`; the target is
`N=(p-1)^2+t^2`. [results.json](../research/nist_skipped_curve_dissection_20261021/factorization/results.json)
contains the proven factors and exact residuals. The residual sizes are
327 bits for P-224 stream block 5736, 444 bits for P-256 block 876, and
364 bits for published P-224. These block three of fourteen timeout-valued
fields; the other eleven fields are resolved.

## Evidence and uncertainty

- **Observed:** six of nine distinct targets are completely factored. All
  nine final partitions pass separate native certificate replay and exact
  product verification. Three residuals are explicitly composite.
- **Observed search limits:** 600-second PARI searches, then 100 ECM curves
  at B1=100000, 200 at B1=1000000 and 100 at B1=3000000 for each residual
  lineage. A split consumed part of its stage's allocation; the remainder
  continued on the smaller cofactor. The final 300 ECM curves found no
  additional factor. [Attempt inventory](../research/nist_skipped_curve_dissection_20261021/factorization/ecm-attempts.json)
  and extracted `runs/` preserve commands, parameters, residues and failures.
- **Derived or proved:** exact nine partition products are proved over
  `Nat` in [ProductIdentities.lean](../research/nist_skipped_curve_dissection_20261021/factorization/ProductIdentities.lean).
  Proven factors have replayed native PARI primality certificates. Lean
  does not prove primality or that a composite residual is hard to split.
- **Suspected explanation:** unknown. No smallest-prime-factor lower bound
  follows from this finite ECM search.
- **Missing capability:** a proper split of each remaining composite and
  a complete prime-certified refinement; no unavailable secret is identified.
- **What this does not establish:** historical skip reasons, security, or
  impossibility of a larger/native alternative factorization search.

## What would unblock it

- [ ] Split all three residuals completely into primes, recording every
  exact divisor and positive exponent without probable-prime placeholders.
- [ ] Replay each retained prime certificate and exact product against the
  original hexadecimal curve parameters, with all nine target identities
  still matching; update the three affected field supplements and report.

These conditions suffice for this factorization gap. They would not fix
the upstream torsion/trace bugs, recover missing historical generators, or
establish why a candidate was skipped.

## Next discriminating step

- **Proof obligation:** provide `C=q*r` for one of the three exact frozen
  residuals, with `1<q<C`, exact division and recursively certified factors.
  Run `certify_parts.gp` followed by the separate `verify_factors.gp` replay;
  retain a rejected hint as evidence instead of treating it as progress.
- **Inputs and controls:** exact residual strings in `results.json`; keep
  the fixed positive ECM control and wrong-target rejection control.
- **Execution limits and stop condition:** the current search allocations
  are exhausted. A further search needs a separately frozen method and
  resource allocation; an experiment selected for later execution also
  needs a backlog card under AGENTS.md section 14. No larger campaign or
  remote machine is queued by this record.
- **Outcomes:** a verified proper split reduces one residual; prime-certified
  completion closes that target; a rejected hint or bounded negative search
  leaves it open. Infrastructure failure is separate from an arithmetic result.
- **Recheck trigger:** a new exact factor hint or a new bounded native search.

## Resolution and history

2026-10-21: PARI completed six targets. ECM found additional prime factors
of all three remaining target lineages. All 100 final B1=3000000 curves
per residual completed without a new factor. Unresolved.
