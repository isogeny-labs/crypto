# Cold timing resume after Spot reclamation

Frozen before using this resume path. Keep the frozen v1 native source,
target, all ten arms, stop criteria and resource caps unchanged.

Accept a previous ephemeral prefix only as an immutable checkpoint. Copy
completed non-timing warmup, first and A/A records individually; their
deterministic content is verified again against the sealed collector.
These are retained workload warmups, not a claim that the replacement
host's caches were warmed by a previous host. Replacement hosts build
and run the native tests before measurements.

A timed unit is a complete round for one instance: all ten arms and the
additional original A/A execution. Reuse it only if all eleven exits and
complete records exist and every isolation receipt admits the resources.
Every record of that round must come from the same original job. Restart
an incomplete timed round in full on the replacement host. Preserve the
earlier partial round and all refusals. Never assemble one round from
arms timed on different hosts, select a faster repeat, or replace raw
timing values.

Write the originating job/prefix with restored rounds. The final
comparison uses paired per-round baseline/candidate ratios; raw medians
may describe several hosts of the same class and are labelled accordingly.
Report the paired original A/A spread. Do not infer a cross-host speed
ratio from unrelated elapsed values. A/A, independent set/rank checks,
all phases and every failure remain required.
