# Independent integration review of provenance-only admission

Reviewer: root. Implementer: root/ci_audit. This is a source and integration
review of the exporter correction, not a mathematical reimplementation or a
pre-data approval. The frozen v5 protocol and measured instrument are unchanged.

Reviewed the exact-parent admission, reconstruction of the allowed metadata diff,
pre-data manifest and reconciled receipt bindings, source/binary provenance checks,
source inclusion in the adapter identity, and output annotation. Production code
requires the reviewed v5 full-file and canonical-body hashes, the exact two
pre-data manifests and the independently reviewed reconciliation receipt. The test
fixture registration path is compiled only under cfg(test). Other parent protocols
and caller-rehashed replacements are not admitted by production code.

Two issues found during implementation were corrected before the actual export:
(1) consistency of caller-supplied parent hashes alone was insufficient; exact
reviewed evidence hashes are now required; (2) draft validation accidentally
called frozen-only admission; valid drafts now retain their previous validation
path. Targeted negative tests cover unsupported post-data changes and rehashed
replacement parents. Existing measured analysis and history retain v5 identity;
the metadata export binds v6 and explicitly records filed_before_affected_data:false.

The actual gate package and subsequent check-export both returned exit0 and no
findings. Both explicitly leave qualified:false. The metadata repair therefore
permits honest packaging without accepting the rejected paired clocks. It does
not change the recorded operations, pricing arithmetic, methods, seeds, plan,
outcomes, caps or resource budget. Native tests and their exact counts are retained
separately; a software pass is not a universal proof or an empirical speed result.
