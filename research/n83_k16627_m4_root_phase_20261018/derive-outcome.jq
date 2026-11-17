($state[0]) as $s |
($host[0]) as $h |
($phases | map(select(.phase == "root_propagation_returned")) | .[0]) as $root |
($phases[-1]) as $last |
if ($s.ExitCode == 124 and $s.OOMKilled == false
    and $h.Memory == 4294967296 and $h.MemorySwap == $h.Memory
    and ($phases | length) == 7
    and $root.details.result == "Fixpoint"
    and $last.phase == "sat_entered")
then {
  schema: "n83.k16627-m4-root-phase-outer-outcome/v1",
  status: "UNKNOWN_wall_cap",
  arm: "root_phase_positive",
  source_commit: $last.source_commit,
  object_blake3: $last.object_blake3,
  point_set_blake3: $last.point_set_blake3,
  start_utc: ($start | rtrimstr("\n")),
  end_utc: ($end | rtrimstr("\n")),
  outer_wall_cap_seconds: 1800,
  container_exit_code: $s.ExitCode,
  oom_killed: $s.OOMKilled,
  hard_memory_limit_bytes: $h.Memory,
  swap_limit_bytes: ($h.MemorySwap - $h.Memory),
  cgroup_peak_bytes: ($peak | rtrimstr("\n") | tonumber),
  worker_json_present: false,
  phase_records: ($phases | length),
  root_result: $root.details.result,
  root_propagation_ms: $root.details.root_propagation_ms,
  root_stats: $root.details.stats,
  last_durable_phase: $last.phase,
  last_phase_elapsed_wall_ms: $last.elapsed_wall_ms,
  sat_result: null,
  solver_decisions: null,
  solver_conflicts: null,
  solver_models: null,
  group_lift: null,
  relation_rank: null,
  unpinned_arm_status: "NOT_ATTEMPTED_POSITIVE_PREREQUISITE"
} else error("outer state or phase sequence differs from the frozen bounded outcome") end
