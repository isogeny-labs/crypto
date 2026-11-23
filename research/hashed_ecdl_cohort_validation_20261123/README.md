# Independent admission and larger-cohort continuation

Follow-up tracking: PR5065, after merged PR5046. Classical/native/local only;
S3 remains excluded. The independent C/OpenSSL admission replay is complete;
no larger solver cohort has started here.
[pcc:unverified]

The frozen census returned the expected classifications for199control rows
and certified all12recorded public targets across6prime-order domains.
All6domains passed the recorded exceptional-class screen. These are unsigned
producer observations, recorded before citation; independent reviewer execution
and signed standing remain open. See [report](REPORT.md), [PDF](REPORT.pdf),
[source](native/admission.c) and [result receipt](evidence/ADMISSION-RESULT.json).
[pcc:unverified]

The merged-source check at main da777709 compares315 tracked port/leaf paths:
313 match exact prior bytes, with differences limited to global ACTIVE and
the canonical ledger. Critical native controls, registration and LeafGuard
source match. PR5046 merged via ab6e8913 with head e06609df; its branch was
deleted while a four-file final metadata update was being prepared. The
unattached metadata commit10173d9d is retained locally, not represented as
published work. See [MERGED-SOURCE-CHECK.json](MERGED-SOURCE-CHECK.json).
Data record PCC-20261123-5b182c is unsigned. [pcc:unverified]

The port study's native160-test execution and focused proofs remain intact.
The leaf application derives the precision size comparison from the canonical
leaf guard, retaining every other inherited/dimension premise. Its seven
consumer/three unchanged-owner gates, eight axiom queries, fresh replay and
local PCC bindings pass. Shared owner50b95696/PR35 remains open. These are
scoped producer checks; signed standing has not been established here after
the merge. [pcc:unverified]

Both scientific checkpoints remain local and immutable: port28371590bytes,
SHA256891e9fbb3b2aa16c4a1956cefe3a0a83cbf5a87909b1bf89b6dc777261f20d32
(1010files/130directories); leaf1198547bytes,
SHA256a5cd1e06675d609b3039ae769a7867b7f3496db5d54632efdfa343eb99f13516
(106files/25directories). Fresh hashes/modes/paths pass. Later publication
and continuation metadata are separate from those captures. [pcc:unverified]

## Sources checked before selecting the next check

Read the frozen candidate's
`crates/hashed-ecdl-cpu/examples/scale.rs`,
`src/cryptanalysis/hashed_ecdl.rs`, the original main protocol/export erratum,
future-only draft and point-audit-v2 source. The worker and auditor both
restrict seeds101/202. The unadopted draft describes those same12 fixed
targets; it is not a larger-target protocol. [pcc:unverified]

`Curve::with_order_witness` already checks prime field/order, canonical
coefficients, nonsingularity, a nonzero r-torsion generator and a unique
Hasse multiple. Scalar multiplication at lines574–586 uses ordinary binary
addition/doubling without reducing by the advertised order. This method is
already implemented. Any new admission check is an independent replay,
not a missing-method claim. Source-review record27c39a is unsigned.
[pcc:unverified]

Primary references read:

- [SEC1v2](https://www.secg.org/sec1-v2.pdf), section3.1.1.2.1: field/order,
  point, nonsingularity, cofactor and exceptional-class validation. Reuse
  relevant arithmetic checks for these research fixtures, without claiming
  deployment-standard certification.
- [Sutherland18.783 Fall2025 Lecture7](https://math.mit.edu/classes/18.783/2025/LectureNotes7.pdf),
  Theorem7.3 and section7.4: Hasse's bound and the existing unique-multiple
  method for determining curve order.
- Official Lean4.35 `Init/Data/Nat/Lemmas.lean:1485`,
  `Nat.eq_of_dvd_of_lt_two_mul`; apply it as stated if a formal arithmetic
  instance is needed. No missing generic counting law is proposed.

KB MCP was absent and two admission queries returned127. Bounded primary and
repository source reads are the fallback; they provide no novelty/absence
clearance. Conductor is unavailable127; no reservation is asserted.
[pcc:unverified]

## Next action

The [draft independent-admission check](DRAFT-ADMISSION-PLAN.md) and
[prospective addendum](ADMISSION-ADDENDUM-01.md) are preserved unchanged.
Prospectively update worker, auditor, collector
and canonical export identity rules together, test synthetic metadata controls,
freeze/publish/checkpoint the new protocol and only then execute new targets.
Keep original360 calls,324seed-table export findings,84-case tensor panel and
all failed attempts unchanged. The new branch inherits the merged main ledger;
it does not overwrite/rebase the earlier producer histories. [pcc:unverified]

## Reproduce the admission census

Check `sha256sum -c runs.tar.xz.sha256`, then `tar -xJf runs.tar.xz`.
The policy-required archive contains the full original control rows and build,
pre-call and post-call logs. The72,728-byte raw JSONL remains byte-identical
inside `runs/native01/rows.jsonl`; its SHA256 is pinned in the result receipt.
The loose receipts remain resolvable by PCC paths. [pcc:unverified]

With a64-bit C compiler and OpenSSL development package, compile the source
below. Supply the extracted public inputs and a new output path. The checker
refuses replacement. Keep the original archived results unchanged.

```sh
TASK_REPLAY=$(mktemp -d)
cc -std=c11 -O2 -Wall -Wextra -Wshadow -Wconversion -Werror \
  native/admission.c -o "$TASK_REPLAY/admission" -lcrypto
timeout --signal=TERM --kill-after=5 60 "$TASK_REPLAY/admission" \
  runs/inputs/targets.tsv "$TASK_REPLAY/new-rows.jsonl"
```

Use the frozen whole-cgroup15032385536-byte sampled stop when reproducing
the recorded resource envelope; the archived wrapper records that policy.
Diagnostic operation counts are not bit-operation costs. No benchmark or
canonical performance chart is changed: this study measured correctness and
classification, with no timing/ranking comparison. [pcc:unverified]
