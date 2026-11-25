# Prospective build resource details

Before the first bounded qualification build, add a256MiB free-disk launch
condition and a64MiB sampled free-disk stop. These supplement, without
increasing, the600s preparation wall cap and15032385536-byte whole-cgroup
sampled memory stop. One build worker and one shared new Cargo target cache
serve the separately copied source trees; copy each produced worker/test
executable before the next build can replace its path.

The restored toolchain's bytes and modes are already pinned. As environment
maintenance before the build, request eviction of cached pages only for those
owned restored files using GNU dd's `iflag=nocache count=0`; this does not read
or replace their contents. Record any failure. Recheck source/tool identities
before/after calls. No benchmark clock is started by cache maintenance.

The completed source-edit step has produced identical extended scale.rs files
for parent/candidate, a36-target collector/auditor and a C/OpenSSL target/scalar
checker reusing the frozen admission source. A warnings-as-errors C syntax-only
check completed before this bounded qualification; it emitted no executable
and is not the separately copied executable qualification. No new target or
solver has run. [pcc:unverified]
