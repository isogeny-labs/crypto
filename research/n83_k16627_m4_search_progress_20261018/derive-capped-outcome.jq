($state[0]) as $s |
($host[0]) as $h |
($phases | map(select(.phase == "root_propagation_returned")) | .[0]) as $root |
($phases[-1]) as $last_phase |
($progress | length) as $n |
($progress[-1]) as $last_progress |
if ($s.ExitCode == 124 and $s.OOMKilled == false
    and $h.Memory == 4294967296 and $h.MemorySwap == $h.Memory
    and ($phases | length) == 7
    and $root.details.result == "Fixpoint"
    and $last_phase.phase == "sat_entered"
    and $n >= 100
    and $progress[0].sequence == 0
    and $progress[0].sample_kind == "entered"
    and $last_progress.sample_kind == "periodic"
    and $last_progress.elapsed_wall_ms >= 1700000
    and ($progress | all(.[];
      .schema == "n83.k16627-m4-search-progress/v1"
      and .source_commit == $last_phase.source_commit
      and .object_blake3 == $last_phase.object_blake3
      and .point_set_blake3 == $last_phase.point_set_blake3
      and (.sample_kind == "periodic" or .sample_kind == "entered")
      and ((.phase as $phase | ["Idle", "ClausePropagation", "XorPropagation", "ConflictAnalysis", "Restart", "BranchSelection"] | index($phase)) != null)))
    and (reduce range(1; $n) as $i (true;
      . and $progress[$i].sequence == $i
      and $progress[$i].elapsed_wall_ms >= $progress[$i-1].elapsed_wall_ms
      and $progress[$i].completed_decisions >= $progress[$i-1].completed_decisions
      and $progress[$i].completed_conflicts >= $progress[$i-1].completed_conflicts
      and $progress[$i].completed_restarts >= $progress[$i-1].completed_restarts)))
then {
  schema: "n83.k16627-m4-search-progress-outer-outcome/v1",
  status: "UNKNOWN_wall_cap",
  arm: "search_progress_positive",
  source_commit: $last_phase.source_commit,
  object_blake3: $last_phase.object_blake3,
  point_set_blake3: $last_phase.point_set_blake3,
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
  progress_records: $n,
  last_sample_elapsed_wall_ms: $last_progress.elapsed_wall_ms,
  last_sample_phase: $last_progress.phase,
  completed_decisions_at_last_sample: $last_progress.completed_decisions,
  completed_conflicts_at_last_sample: $last_progress.completed_conflicts,
  completed_restarts_at_last_sample: $last_progress.completed_restarts,
  work_after_last_sample: null,
  sat_result: null,
  solver_models: null,
  group_lift: null,
  relation_rank: null,
  unpinned_arm_status: "NOT_ATTEMPTED_POSITIVE_PREREQUISITE"
} else error("outer state or progress sequence differs from the frozen bounded outcome") end
