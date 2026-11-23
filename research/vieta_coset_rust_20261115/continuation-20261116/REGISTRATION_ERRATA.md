# Retrospective registration correction

The original closed censored record `20261116T1553Z-vieta-polynomial-backends`
contains empty raw/metrics arrays. It was closed without those entries; closing
did not clear them. Its bytes remain unchanged. The earlier working handoff
incorrectly described the registration as complete.

Sibling record `20261116T1646Z-vieta-polynomial-registration` adds fourteen
pointers to the unchanged evidence and two already-computed native summary
medians. Its command audits source equality only; its creation HEAD and timestamp
are registration metadata, not a new measured interval. The measurement remains
source `9814b4bb5ca5949685be0f99bc992f6e1d0cc8bd` and 16:01–16:16 UTC. No calls
were repeated. Raw evidence, protocols, pinned claims and original report remain
unchanged. S3 addresses are prospective, with uploaded=false. All observations
remain pending independent signed validation [pcc:unverified].
