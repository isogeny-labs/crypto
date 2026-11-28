# Affine CLI acceptance protocol package — staged draft

This package binds two fixed n23/m3/seed42 ordinary-binary cells, both twists, to the finite checked-affine unit-row request. Source and clean CLI build identity is commit `802444fa9dc94194b2ae525d9cd3ad14fe1b9b52`. Source/evidence PR #3490 is separate from this protocol publication.

`protocol.draft.json` is canonical-schema-valid but not frozen or executed. `driver.py` includes the original post-verify sealed inventory and terminal digest anchor. `export_actual.py` is the reviewed V3 adapter plus its one-line pins authentication correction. `tooling-pins.json` binds the exact canonical checker, schemas, interpreter and dependencies; canonical checker sources/schemas are included byte-for-byte under `canonical-tooling/`. `TOOLING_CONTENT_MAP.json` maps the included copies to existing pinned runtime paths. Do not substitute a different environment silently.

`COMMANDS_PREPARED.json` contains reviewable acceptance and external-export argv only. Runtime source, gate and readiness paths are local operational bindings, not commands executed during package preparation. `ready.template.json` deliberately leaves protocol publication and private custody authorization unset and execution disabled. The custody template is not a renewed allowance. Private credentials, active custody authority and generated restricted data are not included in this package.

The primary cell metric uses original manifest/payload identity plus pre-custody generate/verify replay evidence; a later S3 failure does not erase a valid cell. The global pipeline metric includes all registered commands, repeated export, exact public projection and actual private S3 upload/readback/restore. Canonical export files belong outside the sealed run. No degree, completed solver, scalar-recovery, full trajectory or timing result is inferred.

Publish only after final protocol freeze and retain exact GitHub readback. The original cloud budget and scientific deadline are not renewed by this package. No actual dataset acceptance or cloud operation has been executed here.

Suggested final custody request, not authorization: USD 1 maximum total, one 1,800-second attempt, no retries and no EC2/EKS submissions. See `CUSTODY_PROPOSAL.json`; current allowance and absolute deadline remain unbound.
