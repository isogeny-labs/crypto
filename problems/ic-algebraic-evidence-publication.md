# Algebraic IC final-source delivery

- **Status:** investigating
- **Kind:** remaining publication/validation gate; artifact access resolved
- **Affected work:** [IC campaign](../research/ic_algebraic_crossover_20261023/README.md), D2, private PR2550
- **Next action owner:** current IC campaign
- **Last updated:** 2026-10-26

## Problem

The additional source/evidence commits still need final-source checks and a normal
push to the existing PR. Artifact access and upload are now resolved; no storage
exception decision is outstanding.

## Evidence and uncertainty

- Owner approved private-PR publication and later explicitly authorized the
  configured S3 checkpoint and artifact writes. The first automatic checkpoint
  refusal is resolved by that explicit authorization.
- The standard tool published all41 additional files,7,508,437bytes,as30content
  objects in32real manifests. Every actual S3 GET/size/SHA256 check passes;
  previous bindings and all24 original migrated files remain intact.
  [Actual-byte receipt](../research/ic_algebraic_crossover_20261023/pipeline-receipts/artifact-publication-20261026/remote-byte-replay.json).
  Delivery pointers were committed as312a968c8. No fabricated manifest, Git
  exception, evidence splitting or remote deletion was used.
- The official resume checkpoint uploaded and passes actual archive/meta/resume
  readback. A restricted checkpoint token outside Git also passes the official
  upload gate; root credentials are not written to project files or reports.
- The missing8,451,778-byte backend site input was hydrated and hash-checked.
  The site suite now executes51tests:49pass and two generated-data issues remain.
  The own partial-chart copy/link defect is fixed; main also lacks the cited
  canonical n83 history-table row. Base failures are reported, not suppressed.
- Main e8fcadf62 fixes the prior SAT delimiter; merge d3c9f02c7 incorporates it.
  Four Git-tracked include_str fixtures omitted by sparse checkout were restored.
  The release command reached its2,700-second watchdog:4,869named passes,121ignores,one unfinished unchanged JV test. It is not a full-suite pass.
- Earlier frozen source passes and finite research receipts keep their original
  scope. No full IC improvement, current-head CI pass or merge follows from
  storage delivery alone.

## What would unblock it

- [x] Explicit owner publication and S3 authorization.
- [x] Genuine upload/pointers and every41file's actual remote byte verification.
- [x] Current hydration and checkpoint transport capability.
- [ ] Finish required current-source/native-example checks, reporting inherited
  base failures without skipping tests or changing unrelated science.
- [ ] Inspect actual remote head, push normally to PR2550 and follow actual-head
  CI/review. No rebase, force-push or review/CI bypass.

## Next discriminating step

Complete the current library/native build gates and source-bound replay protocol,
refresh the publication packet, pass the scoped artifact policy check and make
the forward-only source push. New research cases still require the protocol
freeze and a fresh quiet source-bound S3 checkpoint.

## Resolution and history

Access subproblem resolved on2026-10-26 after explicit authorization; delivery
receipt is durable. Source push/CI/review remain pending. The earlier no-identity,
GitHub binary-upload401 and unused CI relay remain historical facts, not current
S3 blockers. No additional VM launched; USD100 total extra-cloud cap includes
replay. Broader research and full wide adapters/costs remain open.

Final-source update: the native auditor6tests and20other touched-example
targets35tests pass. Default Clippy checks the auditor with0own diagnostics,
retaining5unchanged base-library warnings; strict Clippy remains failed and
recorded. Two complete bootstrap archives are also stored and byte-verified.
The prospective source-bound248-process replay still awaits JSON freeze and a
fresh quiet checkpoint; no fresh case has run. Inherited115duplicate Gitblobs
were removed from tracking only after real main-binding and actual S3 verification.
