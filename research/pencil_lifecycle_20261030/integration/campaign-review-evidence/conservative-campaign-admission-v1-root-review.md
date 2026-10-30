# Root independent review of campaign admission v1

Patch ee05a0225db3da704fbfccb3d3b4992da665c8292cf010e8856b9348f85ef3df remains unapplied. Source-reviewed policy/ledger arithmetic, idempotent reservation identity, ConfigMap CAS binding and computation/fresh call sites.

Finding requiring correction: unconditional campaign_admit before status processing and at benchmark entry rejects expired reservations before existing terminal/failed owner-artifact collection. Expiry should block worker side effects while allowing terminal observation, evidence retention and cleanup. A late terminal observation must not disappear because its launch allowance expired. Requested a bounded regression fixture and v2 amendment from author.

The ledger is a conservative reservation mechanism only. Kubernetes activeDeadlineSeconds and an assumed shutdown reserve do not prove a physical allocation tail bound. Deployed-role verification, reconciled prior consumption, exact runtime/campaign configuration, external enforcement review and prospective protocol remain outstanding. No scientific execution or measured costs follow from this review.
