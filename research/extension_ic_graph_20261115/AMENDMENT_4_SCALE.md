# Amendment 4: fresh scale holdouts for incremental pair IC

Freeze before observing any of these instances. After matched p67 treatments,
walk pair-only has graph wall/rho 1.294 and 2.901 on seeds3/11, while walk
hybrid is17.56 and28.84. All local controls pass. This selects the pair-only
route for a growth check. This selection is adaptive to the prior pilot;
it is not an independent confirmation of an unselected algorithm.
[pcc:unverified]

- Public random prime-order curves over cubic fields, p=127,523,1039, all
  congruent to1 modulo3; fixture/base/sampler seeds17,23,37 for each.
- SymmetricPairOnly, RAdding32, protected pivot_a/graph/pivot_b/rho_a/rho_b.
- One retained warmup and five measured rounds, odd-round reverse ordering.
- Cap2000000 residuals per IC execution; whole-run900seconds, pinnedCPU0,
  virtual-memory8GiB, zero retries. Failure/cap/timeout/incomplete cohorts
  stay in raw evidence and invalidate their stratum as already specified.
- Same published core/binary, charged cold base/table/start/restarts/replay,
  local A/A tolerance10%, round-bootstrap10000. Different seeds are separate
  strata; repetitions do not create independent curve samples.
- Smallest reference-parity flag still requires n>=2^30 and interval upper<=1,
  all local controls and replay/input checks. Even that flag does not satisfy
  strongest-external comparison, standard lab admission or general parity.
- Prediction to test: reduced sampling constants persist, but pair support
  density and required relation count can grow faster than sqrt(n). Inspect
  residual counts, setup/sampling/oracle work and wall/rho at every size.
  Reject this candidate's local crossover at a tested size if its interval
  stays above1; do not infer that other field degrees or graph constructions
  cannot cross. A rejected control is inconclusive, never a timing win.

Write raw/custody files to create-new local-pair-scale-20261115-05 and retain
all outcomes. Source/spec publication and a local checkpoint precede execution.
