# Bounded modular recoding report

This two-page report describes the completed CM-MODULAR-RECODING-V1 census, independent native replay, P-192 component improvement and live measurement-admission reconciliation. All five curves receive the same finite representative and window search. Counts are exact source-specific operation counts; no cryptographic time was collected.

The report's table and plot are derived from the unchanged copy of ../../recoding-v1/summary.json saved as plot-data.json. Both final PDF pages were opened and inspected. The report explicitly starts shifts from centered scalars. See inspection.json and manifest.sha256. The previous five-curve construction report remains unchanged.

Render counts.typ to counts.svg and report.typ to report.pdf with native Typst 0.13.1, then preview both PDF pages. The retained logs include the unsuccessful preview invocation with a malformed --ppi argument and its successful correction. They do not change the underlying experiment.

Raw data, source publication, frozen protocol, failures, artifact-store URI and replay commands: [recoding-v1](../../recoding-v1/README.md). The exact archive is 93,072 bytes, SHA256 29365ee7c6edb752520467b44de2fa360c5c8542a7efe28c50665eb84cc07406. The artifact bot moved it out of Git; the materialization instructions recover the original bytes without repeating the census.

No scientific scan, job, generator, IAM/RBAC grant, budget or NodePool mutation was repeated during publication. Current admission evidence is in ../../qualification/environment-v2/. A future runtime comparison needs complete cold accounting and an admitted, funded host.
