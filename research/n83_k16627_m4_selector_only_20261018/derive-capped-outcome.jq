($arm == "selector-only" or $arm == "phase-only") as $valid_arm |
(if $arm == "selector-only" then "n83.k16627-m4-selector-only/v1" else "n83.k16627-m4-phase-only/v1" end) as $schema |
(if $arm == "selector-only" then "n83.k16627-m4-selector-only-phase/v1" else "n83.k16627-m4-phase-only-phase/v1" end) as $phase_schema |
($arm == "selector-only") as $selector_pinned |
($arm == "phase-only") as $phase_pinned |
($state[0]) as $s |
($host[0]) as $h |
($phases | map(select(.phase == "root_propagation_returned")) | .[0]) as $root |
($phases | map(select(.phase == "orbit_choices_pinned")) | .[0]) as $pins |
($phases[-1]) as $last_phase |
($progress | length) as $n |
($progress[-1]) as $last |
if ($valid_arm
    and $s.ExitCode == 124 and $s.OOMKilled == false
    and $h.NanoCpus == 1000000000
    and $h.Memory == 4294967296 and $h.MemorySwap == $h.Memory
    and $h.NetworkMode == "none" and $h.ReadonlyRootfs == true
    and ($phases | length) == 8
    and ($phases | map(.phase)) == ["base_import_done", "group_preflight_done", "model_domain_done", "seven_blocks_pinned", "orbit_choices_pinned", "root_propagation_entered", "root_propagation_returned", "sat_entered"]
    and ($phases | all(.[]; .schema == $phase_schema and .source_commit == $source
       and .object_blake3 == "1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d"
       and .point_set_blake3 == "564c6bb72b346b73112240bb0bddc4ef68b7525f7ef2556d6046e52c32f976e0"))
    and ($pins.details | length) == 4
    and ($pins.details | all(.[]; .selector_pinned == $selector_pinned and .phase_pinned == $phase_pinned and .phase == 0 and (.phase_wires | length) == 7))
    and $root.details.result == "Fixpoint"
    and $last_phase.phase == "sat_entered"
    and $n >= 100
    and $progress[0].sequence == 0 and $progress[0].sample_kind == "entered"
    and $progress[0].completed_xor_passes == $root.details.stats.xor_passes
    and $progress[0].completed_xor_nanoseconds == $root.details.stats.ns_propagate_xors
    and $progress[0].completed_xor_row_scans == $root.details.stats.xor_row_scans
    and $progress[0].completed_xor_repivots == $root.details.stats.xor_repivots
    and $progress[0].completed_xor_row_ops == $root.details.stats.xor_row_ops
    and $last.sample_kind == "periodic" and $last.elapsed_wall_ms >= 1700000
    and ($progress | all(.[]; .schema == $schema and .source_commit == $source
       and .object_blake3 == "1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d"
       and .point_set_blake3 == "564c6bb72b346b73112240bb0bddc4ef68b7525f7ef2556d6046e52c32f976e0"
       and (.sample_kind == "entered" or .sample_kind == "periodic")
       and ((.phase as $phase | ["Idle", "ClausePropagation", "XorPropagation", "ConflictAnalysis", "Restart", "BranchSelection"] | index($phase)) != null)))
    and (reduce range(1; $n) as $i (true;
      . and $progress[$i].sequence == $i
      and $progress[$i].elapsed_wall_ms >= $progress[$i-1].elapsed_wall_ms
      and $progress[$i].completed_decisions >= $progress[$i-1].completed_decisions
      and $progress[$i].completed_conflicts >= $progress[$i-1].completed_conflicts
      and $progress[$i].completed_restarts >= $progress[$i-1].completed_restarts
      and $progress[$i].completed_xor_passes >= $progress[$i-1].completed_xor_passes
      and $progress[$i].completed_xor_nanoseconds >= $progress[$i-1].completed_xor_nanoseconds
      and $progress[$i].completed_xor_row_scans >= $progress[$i-1].completed_xor_row_scans
      and $progress[$i].completed_xor_repivots >= $progress[$i-1].completed_xor_repivots
      and $progress[$i].completed_xor_row_ops >= $progress[$i-1].completed_xor_row_ops)))
then {
  schema:"n83.k16627-m4-partial-choice-outer-outcome/v1",
  arm:$arm, status:"UNKNOWN_wall_cap", source_commit:$source,
  start_utc:($start | rtrimstr("\n")), end_utc:($end | rtrimstr("\n")),
  outer_wall_cap_seconds:1800, container_exit_code:$s.ExitCode,
  oom_killed:$s.OOMKilled, hard_memory_limit_bytes:$h.Memory,
  swap_limit_bytes:($h.MemorySwap - $h.Memory),
  cgroup_peak_bytes:($peak | rtrimstr("\n") | tonumber),
  phase_records:($phases | length), root_result:$root.details.result,
  root_propagation_ms:$root.details.root_propagation_ms,
  progress_records:$n, last_sample_elapsed_wall_ms:$last.elapsed_wall_ms,
  last_sample_phase:$last.phase,
  completed_decisions_at_last_sample:$last.completed_decisions,
  completed_conflicts_at_last_sample:$last.completed_conflicts,
  completed_restarts_at_last_sample:$last.completed_restarts,
  completed_xor_passes_at_last_sample:$last.completed_xor_passes,
  completed_xor_nanoseconds_at_last_sample:$last.completed_xor_nanoseconds,
  completed_xor_row_scans_at_last_sample:$last.completed_xor_row_scans,
  completed_xor_repivots_at_last_sample:$last.completed_xor_repivots,
  completed_xor_row_ops_at_last_sample:$last.completed_xor_row_ops,
  work_after_last_sample:null, sat_result:null, solver_models:null,
  group_lift:null, relation_rank:null
} else error("partial-choice resource, identity, phase or monotone progress gate failed") end
