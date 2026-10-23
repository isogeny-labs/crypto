# One NIST extension-twist composite cofactor remains unsplit

- **Status:** `open`
- **Kind:** empirical bottleneck; bounded native factorization search
- **Affected work:** [factorization continuation](../research/nist_skipped_curve_dissection_20261021/factorization/README.md), [original jobs](../backlog/XP-20261021-nist-skipped-curves/README.md)
- **Next action owner:** native factorization continuation
- **Last updated:** 2026-10-23

## Problem

The complete prime factorization of the P-256 block-876 degree-2
extension-twist order remains unavailable because its 444-bit residual
has not been split by the recorded native searches.

For the exact imported models, let `t=p+1-n`; the target is
`N=(p-1)^2+t^2`. [R02 results](../research/nist_skipped_curve_dissection_20261021/factorization/rounds/R02-20261022/results.json)
contain eight complete targets and this remaining composite. Only one of
fourteen timeout-valued fields remains unresolved. The former 327-bit
P-224 block-5736 and 364-bit published P-224 residuals are now completely
prime-certified. The unresolved literal is

```
33492266050798185653682380062483799213318454801653963493134064721059735936944520907295931496614455858076048437184499090478420204888353
```

## Evidence and uncertainty

- **Observed in R02:** eight of nine distinct targets are completely factored. All
  nine final partitions pass separate native certificate replay and exact
  product verification. One residual is explicitly composite. ECM found
  the P-224 block-5736 split; checkpoint-resumed NFS found the published
  P-224 split. The initial published-P-224 native exit 139 remains an
  unknown-cause failure; successful prime/product replay validates the
  eventual factors independently of that failure.
- **Observed R02 limits:** P−1/P+1 plus 100 allocated ECM curves at
  B1=11000000 per lineage; the P-256 lineage attempted 92 and completed
  stage 2 on 91 before exit 124. Its later PARI proposal serialization
  failed, and NFS terminated with exit 137 during polynomial search.
  These failures are inconclusive, not negative mathematical results.
  See the [full R02 report](../research/nist_skipped_curve_dissection_20261021/factorization/rounds/R02-20261022/reports/run-20261022T0724Z/REPORT.md).
- **Historical R01 limits:** 600-second PARI searches, then 100 ECM curves
  at B1=100000, 200 at B1=1000000 and 100 at B1=3000000 for each residual
  lineage. A split consumed part of its stage's allocation; the remainder
  continued on the smaller cofactor. The final 300 ECM curves found no
  additional factor. [Attempt inventory](../research/nist_skipped_curve_dissection_20261021/factorization/ecm-attempts.json)
  and extracted `runs/` preserve commands, parameters, residues and failures.
- **Derived or proved:** exact nine partition products are proved over
  `Nat` in [R02 ProductIdentities.lean](../research/nist_skipped_curve_dissection_20261021/factorization/rounds/R02-20261022/ProductIdentities.lean).
  Proven factors have replayed native PARI primality certificates. Lean
  does not prove primality or that a composite residual is hard to split.
- **Suspected explanation:** unknown. No smallest-prime-factor lower bound
  follows from this finite ECM search.
- **Missing capability:** a proper split of the remaining composite and
  a complete prime-certified refinement; no unavailable secret is identified.
- **What this does not establish:** historical skip reasons, security, or
  impossibility of a larger/native alternative factorization search.

## What would unblock it

- [ ] Split the remaining P-256 residual completely into primes, recording every
  exact divisor and positive exponent without probable-prime placeholders.
- [ ] Replay each retained prime certificate and exact product against the
  original hexadecimal curve parameters, with all nine target identities
  still matching; update the remaining affected field supplement and report.

These conditions suffice for this factorization gap. They would not fix
the upstream torsion/trace bugs, recover missing historical generators, or
establish why a candidate was skipped.

## Next discriminating step

- **Proof obligation:** provide `C=q*r` for the literal remaining residual,
  with `1<q<C`, exact division and recursively certified factors.
  Run `certify_parts.gp` followed by the separate `verify_factors.gp` replay;
  retain a rejected hint as evidence instead of treating it as progress.
- **Inputs and controls:** fixed literal and checksum-bound checkpoint;
  keep the genuine-GNFS known-answer/resume controls and wrong-target rejection.
- **Execution limits and stop condition:** R02 is finished.
  [R03](../research/nist_skipped_curve_dissection_20261021/factorization/rounds/R03-20261022/PROTOCOL.md)
  records a duplicate-search restore failure and validates the recovered
  master file. [R04](../research/nist_skipped_curve_dissection_20261021/factorization/rounds/R04-20261022/PROTOCOL.md)
  separately permits one corrected 1800-second native continuation. A later
  [draft backlog card](../backlog/XP-20261022-nist-p256-nfs-resume/README.md)
  preserves the next bounded allocation; it is not queued. No remote
  machine is launched by these records.
- **Outcomes:** a verified proper split reduces one residual; prime-certified
  completion closes that target; a rejected hint or bounded negative search
  leaves it open. Infrastructure failure is separate from an arithmetic result.
- **Recheck trigger:** a new exact factor hint or a new bounded native search.

## Resolution and history

2026-10-21: PARI completed six targets. ECM found additional prime factors
of all three remaining target lineages. All 100 final B1=3000000 curves
per residual completed without a new factor. Unresolved.

2026-10-22: R02 closes the two P-224 residuals with native prime certificates
and separate replay. Eight targets and thirteen timeout fields are now
complete. The P-256 residual remains unresolved.

R03: a per-thread-only checkpoint silently restarted polynomial search;
the duplicate attempt was stopped with exit 143. Native exact checks
validated 242 saved candidates and a separate YAFU diagnostic confirmed
resume coefficient 660. The wrapper now refuses an unrecognized master;
R04 separately tests the corrected state with smaller checkpoint batches.

R04: the production allocation ends at its outer bound with exit 137 and
no new factor. Exact validation retains 329 records through coefficient
900. A separately bounded diagnostic selects the best saved polynomial by
native score and generates a sieving job; both job/parameter formats pass
exact input and common-root checks. The native configured relation target
is 46,000,000, with no runtime or sufficiency guarantee. The next bounded
job can start sieving from the [R04 checkpoint](../research/nist_skipped_curve_dissection_20261021/factorization/rounds/R04-20261022/README.md).

R05: the local 3600-second native allocation ends at 2026-10-22 11:28:15 UTC
with exit 139 and a segmentation-fault diagnostic. The exact failing native
component and cause are unknown; no outer-timeout signal was logged. The
main file has 3,164,757 raw-relation lines; a separate worker output has
32,684 lines. Both are retained without claiming deduplication or usable
rank. Post-failure refinement and independent prime/product replay pass,
with no new factor. No factorization worker is running at finalization.
The [completion report](../research/nist_skipped_curve_dissection_20261021/factorization/rounds/R05-20261022/reports/completion-20261023T0100Z/REPORT.md)
and archive manifest identify the final evidence; the original interim
observation remains unchanged. A subsequent
[separate native timeout diagnostic](../research/nist_skipped_curve_dissection_20261021/factorization/native-timeout-diagnostic-20261023/README.md)
reproduces original exit 139 versus patched exit 0 on the same input and
small Q range, validating a two-line printf correction. Its original
executable matches R05; no original production backtrace was retained.
Use the binary-bound checked wrapper for the next bounded resume after
actual checkpoint publication, preserving the separate worker file until
safe merge semantics are verified.

2026-10-23: [FactorDB](../research/nist_skipped_curve_dissection_20261021/factorization/factordb-20261023/README.md)
returns no additional factors for the nine original integers or the exact
444-bit residual, which remains `C`. Native checks verify all ten returned
products and that their pieces only group whole local pieces. Seven
original targets are more completely factored locally and two partitions
agree. This does not establish novelty or explain the historical skip.
