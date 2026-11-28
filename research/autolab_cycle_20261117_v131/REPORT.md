# C131: liveness collector passes controls;actual cones reach work cap

Executed the existing frozen,previously unattempted collector exactly once. No source,input or old attempt was changed. The initial checks passed32circuits,1,512exhaustive assignments,15,120value comparisons,320exact polynomial comparisons,177forced periodic collections and one deterministic failed-allocation/collection/retry control. Collection occurs after recursive arithmetic returns,remaps every live image,rebuilds uniqueness and clears caches containing old IDs. Source use counts retain every image needed by a future gate or requested output;work never resets on retry.

All actual1/2/4/8correction profiles moved past repeated allocation failures by collecting dead intermediates,but reached2,000,001attempted recursive calls at the2Mwork limit before any output polynomial completed. They reclaimed316,303/322,225/330,985/303,070nodes,with19/20/22/25collections and3gate retries per arm. Retained nodes at the cap were52,165/47,936/39,112/78,769. This distinguishes retention overhead from the earlier100kallocation stop,but does not prove that the final diagram is small or that it can finish within the same work budget. No actual polynomial,degree,pruning certificate or exact root count was obtained.

Supervisor measurements(seconds,KiB RSS):

{
  "status": "ok",
  "wall_seconds": 12.382531606010161,
  "peak_process_rss_kib": 739992
}

Budgets remained90souter/4GiB/CPU0,100kmanager nodes,500kcache entries,2Mrecursive calls,500kfresh unique-node insertions and5Mcharged collection visits. Compaction copies are separately charged by collection visits rather than fresh insertion counts. Worker reporting retains78/84sguards. Capped timings are development observations,not qualified speed or memory ratios. Existing17-field coefficient-basis tests remain distinct from this binary-selector representation experiment.

New hypothesis: use the precision filtration to certify unit derivatives along parameter directions,without expanding an entire correction polynomial. The exact derivation and bounded rank protocol are in UNIT_DIRECTION_NEXT.md. A rank increase after adding the current Sylvester-projected coefficient row yields an explicit direction preserving all earlier coefficients and toggling the necessary correction. Its selector can be eliminated by a cofactor circuit,with exact source reconstruction. Rank equality would refute only this sufficient filtration route. This is unexecuted theory,not a measured dimension reduction or speed gain.

One scientific worker ran this cycle. Checkpoint preparation started immediately after review,using a compact reusable helper;the prior coordinator overrun is retained in PREVIOUS_BUDGET_REVIEW.json. All frozen plans,source/input hashes,raw stdout/stderr and attempts are preserved. Portable replay verifies those hashes and reproduces all four capped collector counters. The previousC130validation passed558records;its delayed record is included in this checkpoint. The original source/U/ghost/outer all-roots recovery,full curve/domain coverage,genuine matchedF4/F5/F6interfaces and20hard-instance release gate remain active and unpassed. Neither2xnor50xis claimed.
