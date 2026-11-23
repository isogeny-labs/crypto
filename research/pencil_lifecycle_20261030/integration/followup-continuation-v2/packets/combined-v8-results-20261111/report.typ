#set page(paper:"a4",margin:16mm)
#set text(size:9pt)
= Combined signed/Kummer v8 result
Terminal producer report · 2026-11-11

Materialization, root library test-binary compilation and driver staging passed. Driver compilation exited101: two serde_json macro errors at driver-controls.rs:122, where nested [0;4] repeat expressions were placed directly inside json!. No root tests, driver tests, full739 calls, forced probes or adapter negatives ran. The separate inline filesystem control passed all four positive and five negative cases. Terminal sealing passed and cleanup completed.

#image("timing.svg",width:100%)

== Preserved artifacts and accounting
The successful root test binary is bound by path and SHA256 in summary.json; this report rehashed its bytes without invoking it. The filesystem-control receipt records 0.003688243 seconds. Materialization recorded4,485 source files and preserved2,165 cache modes including82 executable inputs. Cgroup CPU time:1390.513635 seconds. Runtime controls:2 CPUs,16 GiB memory,zero swap.

The full2,400-worker-second reservation is charged:25,380 charged and104,220 remaining at this terminal boundary; deadline November11,12:00UTC. Observed intervals are nested, not additive; dollar cost remains unmeasured.

== Exact evidence
Native/caller receipts, driver source inventory and materialization receipt are archived with exact hashes. Terminal-manifest entries, completion-bound digest, command output hashes and worker materialization binding are checked. Prior v6 andv7 failures remain preserved. These are producer-report self-checks, not an independent result review. Driver compilation diagnostics and all unattempted stages remain explicit.
