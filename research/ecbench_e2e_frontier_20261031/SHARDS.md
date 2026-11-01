# Queue-safe frontier shards

The current-source campaign now has **92 exact planning specs**. The pinned
static binary accepted all 92 with `ecbench plan`, expanding them to **13,888
fresh processes** with zero planner failures. This is a planning result only:
no warmup or measured child has run.

Exact identities and counts are in
[`execution/shard-preflight-plans.json`](execution/shard-preflight-plans.json);
the intended queue order and bounds are in
[`execution/shard-manifest.json`](execution/shard-manifest.json).

## Envelope

Every rankable shard has one curve, four targets, one warmup and six measured
rounds in balanced alternating order. Two prospectively selected target blocks
give eight targets per comparison cell. Each shard contains its candidate
group, the strongest applicable rho reference set, and the corresponding A/A
control set.

For `a` arms, the generator fixes the child timeout to

```text
floor(2400 / (4 targets * 7 executions per arm * a arms)) seconds.
```

The 92 shards use 12–28 seconds per child. Their largest sum of child timeout
ceilings is 2,380 seconds, leaving at least 1,220 seconds of the 3,600-second
portable EKS deadline for workload construction, Karpenter provisioning,
harness setup, native audit and upload. A timeout is a retained censored row;
measurement retries are zero.

| panel | jobs | planned processes | target law |
|:--|--:|--:|:--|
| prime 20/24/28/32-bit IC and generic algorithms | 16 | 2,632 | public |
| Koblitz 13/19/23/31-bit rho, root finding, F4/F6 and claw | 32 | 5,152 | public |
| random-binary 13/19/23/31-bit matrix F4/F5 and inherited F4/F6 | 16 | 1,792 | public |
| all 11 nested algebraic solver backends | 12 | 1,960 | public |
| j=0 and j=1728 symmetry panels | 4 | 672 | public |
| n=29 one-column generator-orbit engineering panel | 2 | 392 | public |
| F_((2^m)^4), m=10,12,15 specialized and m=16 wide | 8 | 1,008 | planted; see below |
| binary n=11 double-large-prime v2 | 2 | 280 | public |

The roster includes all 13 methods classified `current_primary` in
`coverage.json`, all 11 nested solver backends, both current Koblitz
strong-rho implementations, all accounting-v2 BSGS variants, the generic
prime root-finding variants, matrix F4/F5, inherited F4, F6-IC, SAT-CDCL,
FES/Gray code, wide FES, Crossbred, XL, Buchberger and exhaustive search. It
also retains the matched legacy/full-price arms needed to interpret symmetry,
generator-orbit and extension-field history. It does not turn the other 61
diagnostic, legacy, pricing-sensitivity or object-specific top-level IDs into
one global ranking; their dispositions remain closed in `COVERAGE.md`.

## Reference rules fixed before data

- Generic prime and random-binary candidates use `rho.negation_v2` and an
  identical A/A arm.
- j=0 and j=1728 use `rho.cubic_v2` and `rho.quartic_v2`, respectively, each
  with an identical A/A arm.
- Koblitz shards carry both `rho.signed_frobenius_strong_v2` and
  `rho.signed_frobenius_strong_escape`, plus an A/A copy of each. The decisive
  denominator is the paired minimum common-accounting `S` of those two
  references. The noise envelope is the union of their A/A envelopes. This
  rule is fixed here; wall time never selects the reference after the fact.
- The one-column generator-orbit panel stays in its full-price unit and
  carries both full Frobenius references and both A/A controls. It is an
  engineering panel, not a uniform-target scaling result.
- The one-word F_((2^m)^4) panels use counted `rho.negation_strong`; the wide
  m=16 implementation uses its only implemented wide matched rho,
  `rho.negation`.

## Extension-field target limitation

The `f2m4_torsion` fixtures exhausted the harness's fixed 10,000-attempt
public hash-to-subgroup constructor during preflight. Those eight specs
therefore use planted known-answer targets, as the historical protocol did.
The scalar is retained only by the runner for independent verification and is
not passed to a solver. These rows can compare implementations on the same
workload, but they are not public-target frontier evidence and are labelled
separately. The wide m=16 panel is additionally expected to censor under this
short queue envelope; historical rows took hours.

## Admission state

The shard set is not frozen and must not be executed yet. The remaining gates
are:

1. exact-image runtime preflight for each arm class (the deterministic
   analysis driver is already hash-bound and passes `--check-only`);
2. a successful repository resume-checkpoint upload and
   `scripts/checkpoint.sh gate`;
3. protocol freeze, including the existing analysis hash, and merge of these
   exact spec IDs to `main` before the first warmup;
4. reconciliation against existing EKS/Actions receipts, then exactly one
   submission per paid shard.

`analyze_shards.py` is already bound in `protocol.json` as SHA-256
`500e8ffa3e737b2a8a7711ba60b3a1f78e8ad95958c94bd0b3b05d330049df94`;
its `--check-only` gate accepts the exact 92-spec/13,888-process roster and
pinned executable.

The current runtime has no checkpoint transport, AWS/Kubernetes/Iso Lab
client, or valid GitHub Actions credential, so no submission was attempted.
