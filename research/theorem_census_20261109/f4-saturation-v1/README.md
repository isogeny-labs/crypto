# First saturation trial: retained failures

Frozen protocol hash `637a5a3b29fecdd04fd841012757104c846fee85c239ca61ae1282261537b6f2`.
The ordinary F101 call matches the expected basis. Saturation exits 1 because
upstream rejects the low characteristic. Both calls and raw diagnostics are
retained. [pcc:unverified]

The frozen sample count also implies four calls while the stopping rule and
runner specify two. `evidence/export-check-03.log` retains three planning
failures. This record is not protocol-valid; see `DEVIATION.json`. No missing
calls were fabricated and the frozen protocol was not changed. Export drafts
retain metadata corrections. [pcc:unverified]

The separately frozen [second version](../f4-saturation-v2/README.md) does not
erase this result. The [formal audit and source map](../F4_MODEL.md) and
[PDF report](../reports/proof-20261110-f4-model/report.pdf) describe both
versions and the independent all-field Lean model boundary. [pcc:unverified]
