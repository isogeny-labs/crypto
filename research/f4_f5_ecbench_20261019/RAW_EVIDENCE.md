# Native evidence and replay

The final source contract is9bed104b0a. The earlier0acfc55b7f and71c4c99d85 jobs are
separate intermediate experiments. Both compare the frozen a78 algorithm
with its previously qualified GF2 policies against new explicit policies;
no cross-host ratios are pooled or multiplied.

All six dedicated jobs completed with exit0 and automatically terminated.
The archive retains all three source versions, not just the final winners. Executables stay in their immutable
S3 job trees, with SHA256 manifests in the archive; environment-export files
and credentials are never copied. Source snapshots and patches identify the
actual measured code even when the runner checkout revision is a78.

| Archive label | Job ID | Contract |
|---|---|---|
| iteration1-native | 20261019-165506-boolean-recursive-native-188e81 |0acfc55b7f |
| iteration1-ecbench | 20261019-165520-boolean-recursive-ecbenc-9d61aa |0acfc55b7f |
| iteration2-native | 20261019-174306-boolean-final-native-c19491 |71c4c99d85 |
| iteration2-ecbench | 20261019-174315-boolean-final-ecbench-25cc20 |71c4c99d85 |

Each immutable remote job tree is at
`s3://ephemeral-runs-104186811093-us-east-1/jobs/<job-id>/`.
`collect-raw.sh FETCHED_TREE LABEL` copies the administrative and raw evidence
without changing sealed session records. `runs/local-smoke` is functional
L0 validation only. `runs/profiles` and `runs/lookup-profiles` are simulated
instruction counts, not native time. Every failure remains in its tree.

The archive contains20123 raw files in3008848 bytes,
SHA256`d9bd9ab89f837c4aa7b2c962f36f0c22e11443b2cd6eaeea03487610f567e666`. Executables and environment-export files
were checked absent. Extracting and repacking uses the following contract;
the final round-trip receipt is `records/archive-roundtrip.txt`.

The deterministic packing contract, from this directory, is:

```sh
tar --sort=name --mtime='2026-10-19 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs \
  | xz -9e -T1 > runs.tar.xz
sha256sum runs.tar.xz > runs.tar.xz.sha256
```

Check the adjacent SHA256 and extract the archive:

```sh
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
cd ../..
CARGO_TARGET_DIR=/tmp/boolean-replay-target bash research/f4_f5_ecbench_20261019/replay-analysis.sh
```

Analysis and bootstrap statistics are Rust. Python only formats recorded
values, plots graphs, renders PDFs and inspects their pages. Native receipt
admission retains the first eligible of at most three attempts; it never
chooses the fastest retry. Full ecbench replays and native harness comparison
receipts are separate from analysis replay, which runs no solver.

The original study's raw evidence and reports remain unchanged. External
Magma/msolve/FGb/Singular/PolyBoRi runtime comparisons are still missing:
those tools were unavailable in the original native panel. This continuation
makes no fastest-external implementation claim. Arm64/GPU, n19 admission,
complete-cost pricing, independent rho qualification and the original hard
all-roots/8x research gate remain separate open obligations.

Final corrected jobs:

| Archive label | Job ID | Contract |
|---|---|---|
| native | 20261019-190236-boolean-adaptive-native-d28a77 |9bed104b0a |
| ecbench | 20261019-190253-boolean-adaptive-ecbench-cbeb18 |9bed104b0a |

`runs/adaptive-screen` retains the failed global-lock preflight with no session.
`runs/adaptive-screen-retry1` retains the pre-version16-record L0 work screen;
`runs/versioned-screen` retains the same16-record screen under the corrected
method identities. Each has8 warmups and8 measured replays. The frozen-guard
source is `frozen-adaptive-screen/koblitz_groebner.rs`, hash
`b493900ce118affe8f379f0b2d44982edba25d8b2eeb09b56cb6491d792acdc8`.
`replay-frozen-screen.sh SCRATCH_DIR SESSION...` builds that module in a
separate target, restores the current source, then audits the pre-version
sessions. Run it with no concurrent builds or edits in this checkout. Its
successful audit/build/binary manifests are in `runs/frozen-screen-replay`;
its executable is outside the archive. Versioned and legacy all-width
sessions replay on the current binary because their method identities keep
their original search schedules. Weekly CI routes the pre-version pair to
the frozen-source recipe explicitly; no sealed record is edited or skipped.
