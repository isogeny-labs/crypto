# DRAFT, not frozen and not dispatched: measurements this lane's results call for

Status: draft for the lane owner. Nothing here has been registered, allocated
or run. No paid compute was used by this lane. [pcc:unverified]

## M1. Traced descent inside the complete deferred pipeline (counted work and process wall)

Question: does C1, the Joux–Vitse F4 trace replay with the acceptance rule in
`traced_descent_check.rs`, lower the complete deferred-pair process when it
replaces only the descent's F4 call? The local result covers recorded residual
streams only.

- Arms: unchanged deferred pipeline (`DescentBackend::Original`), and the same
  pipeline with `nagao_decompose_using(..., Some(traced_solver))` in the
  descent loop. Nothing else changes: the same collection, kernel, stream,
  stop and materializer.
- Fixture: the Run50 public input (p1009, seed389) is exposed and already
  consumed, so it can serve only as a positive control. The primary comparison
  needs a fresh public target generated after freezing, with the seed derived
  from the protocol hash.
- Controls: an A/A pair on the unchanged arm. Exact equality of accepted
  descents, affine rows, scalar and every non-descent counter between the
  arms. The oracle arm (F4 to a certified basis) on the descent residuals of
  both arms. Replay acceptance, rejection and fallback counts. The learning
  RREF bound reported separately.
- Primary metric: counted descent work under the three pricings of
  `descent_breakdown compare`, plus process wall. Wall goes through the
  registered native isolation wrapper on an isolated host; the AGENTS.md route
  is isolab-eks or the user's Runpod authorization, decided by the owner, not
  by this draft.
- Prediction from the local controls: descent F4 multiplications fall about
  2.3x and the conservatively priced descent about 1.6–1.7x. The end-to-end
  effect is bounded by the descent share (58% of the Run50 deferred observed
  subtotal) and by uncounted work. C5 shows that a counted saving can vanish
  from wall time. The prediction is falsified if the wall ratio CI includes 1.
- Budget: none requested here. The owner sets it.

## M2. Exact-log class domain of the deferred graph (finite algebra, local)

Question: for C2, how many signed canonical degree-2 classes have discrete logs
that the certified kernel determines exactly? What fraction of the class
universe is that?

- Inputs: a recorded deferred-graph collection: vertices, anchors,
  activated/live edges and the kernel. Run50's large witnesses are LOCAL at
  `/workspace/ic-runpod-runs/...`, which is absent from this host. The owner
  has to supply them, or a fresh local deferred collection has to be run. The
  latter is a local Rust run without the descent, small at p1009, but it is
  still a run and is not done here.
- Computation: connected components from anchors through recorded rows; exact
  witness replay per admitted class; an exact count of the class universe for
  `p = 1009` by enumerating monic irreducible quadratics `U` over `F_p` and the
  `V` with `V^2 ≡ H mod U`.
- Output: `|D_exact|`, `|D_universe|` and the C2 yield model with measured
  inputs. Only if it predicts at least a 1.5x residual reduction, draft M3: the
  anchored materializer on the identical residual stream, with independent
  witness replay.
