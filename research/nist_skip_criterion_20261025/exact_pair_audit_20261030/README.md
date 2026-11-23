# Exact label-allocation audit of verified split scores

Preparation,30October2026. The broader69-position census and its82certified
branches were previously verified and merged in PR2666. This audit uses those
existing score rows. It does not recompute curve orders/characters, generate
curves, run an attack or rerun the external sampling study.

The [fixed plan](RUNPLAN.md) compares42additional candidates with eight
publications in four fields, reproducing the external report's comparison
definition and retaining all source positions. It replaces its Monte Carlo
label allocation with exact finite enumeration; it does not replace the
requested10000real prime-order curves per field. All prior data/labels and
external reported probabilities were visible before this audit.

Producer uses subset enumeration and pair scores; the separate verifier uses
doubled midranks and rank-sum dynamic programming. Shared TSV I/O has no
statistic/null arithmetic. Both native builds and toy controls pass. Actual
allocation arithmetic has not started at this preparation commit. The
original checkpoint/protocol deviations remain explicit in the copied sources
and are not repaired by this later audit.

Read [BASE-CHANGE.md](BASE-CHANGE.md) for the algebraic meaning of a possible
public G/b scalar recipe. Its source is RFC5639AppendixA.2, not a claim about
the original NISTgeneration code. No new empirical trait is asserted there.

Local execution is exact analysis of existing results under AGENTS9a's
development exception, explicitly CHECKPOINT_SKIP. No S3receipt or sampling
gate pass is claimed. Source freeze precedes real input processing. Run only
the native bounded launcher after that freeze, with fresh directories:

```sh
sh research/nist_skip_criterion_20261025/exact_pair_audit_20261030/run.sh \
 /workspace/nist-exact-pair-audit-build-001 /workspace/nist-exact-pair-audit-run-001
```
