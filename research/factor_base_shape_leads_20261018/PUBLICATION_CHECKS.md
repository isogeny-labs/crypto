# Publication checks

- Research archive: 4,748,392 bytes.
- SHA256: eda98dced594f2bd2daaab63528c8a30ac46610cecba24df115eacd24bb060d5.
- All 8,546 regular-file member hashes pass after extraction.
- Repacking the extracted runs tree with the same GNU tar/xz normalization
  produces byte-identical bytes (cmp exit0).
- The exact newly added CI evidence-replay shell step passes locally: all
  five native run directories verify (60 research seals plus4 software
  regression seals), and native aggregate/independent-coverage outputs match
  the published JSON byte for byte. The new control regression's independent
  coverage output also matches its retained receipt.
- Updated workflow YAML and both new shell snippets parse. All9 CI-scope
  tests pass, including selection for audit source/manifests/evidence changes.
- Three final PDFs were rebuilt; all7 pages were visually inspected.
  Benchmark browsing copies are byte-identical to the study PDFs/figures.
- New draft backlog card validates natively with two intentional TODO warnings.
- Old authoring raw bundle remains SHA256
  2563a83feee992afcdff5ea408542700d834c8f1c94bfa8431efba9d9bf02931.

Logs, exits and the exact replay scripts are in
publication-checks/checks.tar.xz, with its SHA256 sidecar. The research runs
archive was not reopened or altered to add these later publication checks.
All checks are local observations. Fresh current-head GitHub CI/review remains
a separate merge gate; these checks do not bypass the documented base failures.

The independent census uses the native records' subgroup/cardinality/cofactor
facts, checked generator order and its separate field/group arithmetic. It is
not a new independent curve point-count certificate or a regularity proof.
