# Intact consumer raw archives: documented migration handoff

These are complete original standardtar.xz archives of consumer full01
(failed,NOT accepted) and full02 (complete local400f38). Each original
custody and every-member hash table is copied byte-for-byte from consumer
807207e1cb7787807c7d32d9402b2cb3bc0ee189. handoff.json pins exact copied
archive bytes/SHA256. No tree was pruned or split. [pcc:unverified]

Both exceed65536 bytes. Their temporary tracked handoff uses the documented
same-repository PR artifact-autofix route in crypto/docs/artifacts.md.
The trusted base tool uploads,downloads and checks full bytes,then commits
real .artifacts.json/.gitignore/removals with a normal push. Artifact
check is intentionally PENDING/failing for these two selected files until
that actual migration;policy and bypass labels are unchanged.
An earlier no-op automation job is not upload proof (3eeb2e).
No remote raw availability is claimed here. [pcc:unverified]

After actual bot migration,retain an additive publication receipt with
immutable manifest commit and full workflow logs. Consumer can reference
the canonical root manifests/objects without changing its old three jobs;
these archives are report evidence,not runtime replay inputs. Preserve
all original reports,claims,custody and failed-attempt standing.
[pcc:unverified]
