# Amendment 21 jobs: all 11 pods lost (recorded 2026-11-15 about 17:30Z)

Measured, as far as it goes. `[pcc:unverified]`. Amendment 15 §7 loss rule.

- **When.** All 11 pods of `results/LAUNCHES_20261115_A21.md` were created between about 16:46 and 16:47Z and started their
  jobs by 16:49Z. At 16:56Z every runner answered `running`: the 10 Kummer jobs were in their E3.7 control (`cap:12:0:6`), and
  `iso-null` was in `A-m2-iso-p1-null`.
- **At about 17:26Z** every runner URL returned HTTP 404 through RunPod's proxy. `GET /v1/pods/<id>` returned empty records, and
  `GET /v1/pods` listed **none of the 11 pods**. Older `EXITED` pods are still listed, so these were deleted, not stopped. This
  session did not delete them: its deletes in this session were only of finished pods, each by ID.
- **No rows were saved.** No fetch happened between the start and the loss. The runs were mostly in their controls, whose
  answers are already known.
- **Billing does not explain it.** `GET /v1/billing/pods`, grouped by pod, shows none of this session's pods on 2026-11-15:
  neither these 11 nor the morning's pods, which ran for hours and returned results. It does show two pod IDs not created by
  this session (`t0bqkj2h4g2dgo`, US$3.43, 98 min; `y8dxu6qiiykumx`, US$2.02, 57 min, 04:00–12:00Z) that are not in the pod
  list either. So the billing view does not reliably cover these pods, and the cause and time of the deletion are **unknown**.
- **Attempt counts** (per cell, across routes; Amendment 15 §7):
  - Kummer `t = 14` cap 6, both draws: attempt 1 lost.
  - `m = 4` and `m4n`, 8 cells: attempt 1 lost.
  - Isogeny null: attempt 1 lost.
  
  Each may be relaunched once, with the owner's authority.

## Cause (owner, 2026-11-15)

The owner, on being told of the loss: "Damnit I think budget ran out". The most likely cause is that the RunPod account balance
ran out, so RunPod terminated the pods. That is consistent with all 11 disappearing together.
- **Lesson for the route.** Amendment 15 §6 relied on a single balance confirmation ("a few tens of dollars", decision 10).
  Launching 11 pods (about US$11.40/h, about US$717 at their limits) needed a balance check sized to the batch.
- **Rule from here (plan; to be frozen with the relaunch).** Before each batch, the owner confirms the balance covers the batch's
  spend at the limits, or the batch is sized to the confirmed balance. Fetches happen every 15 minutes, so a stop loses little.
