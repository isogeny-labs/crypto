# Historical 60-bit prime conductor-gap study

This is an archival import requested by the owner, not a new experiment or a
new native execution path. It preserves the source, frozen parameters, failures,
receipts and follow-up route assessment from source commit
`b6666f5058260f602070c06d1f56d03979295dec`.

The historical study checked a source and rational kernel for the prime
`576460755499654231` (60 bits). The explicit quotient equation, complete
non-kernel images and matched endpoint ECDLP experiment remain open. The source
and kernel checks are executable Sage checks plus a CM/volcano argument, not
a complete Lean proof. See [parameters.json](parameters.json), the unchanged
[historical summary](historical-summary.json), and the
[existing problem record](../../problems/conductor-gap-upper-floor-and-cost.md).

The Python/Sage producer, native C helper and historical Python-driven timings
are preserved as evidence inside the archive. They are not installed or
introduced as this repository's research harness. A future extension must use
the repository's native execution and measurement rules, with new receipts.

## Retrieve and inspect

The artifact policy moves the archive's bytes to the content-addressed S3
store; the same-repository artifact-autofix workflow performs that upload and
fresh-byte verification on the PR. Until its commit lands, the archive is
present directly on this source branch. After migration, retrieve it with:

```sh
python3 tools/artifacts/artifacts.py pull research/prime_conductor_gap60_20261031
cd research/prime_conductor_gap60_20261031
sha256sum -c source-and-evidence.tar.xz.sha256
tar -xJf source-and-evidence.tar.xz
```

The extracted `historical/` tree retains each original path and byte. In
particular, `historical/evidence/input_manifest.json` hashes the frozen evidence
files relative to `historical/evidence/`. The source archive was made by:

```sh
git -C /workspace/github-export archive --format=tar --prefix=historical/ b6666f5058260f602070c06d1f56d03979295dec | xz -6 -T1 > source-and-evidence.tar.xz
```

The complete loose mirror is on the `codex/prime-conductor-gap60-archive-20261031`
branch of [aburan28/isogeny-labs](https://github.com/aburan28/isogeny-labs/tree/codex/prime-conductor-gap60-archive-20261031/archives/prime-conductor-gap60).

## Report and scope

[The descriptive report](reports/report.pdf) and
[workflow diagram](reports/status.svg) distinguish completed prerequisites,
the failed standard constructor and incomplete block work. No fresh benchmark,
backend ranking, record claim, curve-registry promotion, or leaderboard change
is made by this import. Canonical graphs remain unchanged for that reason.

Repository prior work checked before choosing the archival placement:

- `research/conductor_gap_cm_orbits_20261015/REPORT.md` and
  `problems/conductor-gap-upper-floor-and-cost.md`: existing construction and
  accounting obligations; the historical instance here remains distinct.
- `docs/artifacts.md`: immutable byte storage and same-repository autofix.
- Sage 10.9's square-root Velu source, Robert's efficient-representation paper,
  Galbraith's tall-volcano paper and Fast and Frobenius: links and input
  applicability assessments are retained in
  `historical/research/route_assessment.json`.

No current experiment was launched while preparing this import. Cargo is not
installed in the publishing runtime; that pre-push library check is recorded
as unavailable, not passed. Archive/hash and documentation checks are separate
from scientific replay.
