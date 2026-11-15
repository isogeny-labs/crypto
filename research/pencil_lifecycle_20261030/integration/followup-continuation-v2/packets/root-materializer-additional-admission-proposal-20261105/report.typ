#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Additional materializer admission — inert proposal

#text("The reviewed mapping correction and unchanged safety helper are copied byte-for-byte, with their hashes in bindings.json. preparation.json and launch.json specify new unique proposed paths and an exact prospective command. Both carry approval_required=true and launch_disabled=true. The configuration additionally has reviewed=false, published_github_commit=null and lease=null: the unchanged worker rejects it before creating output. The descriptive launch flags are operator gates, not fields enforced by the legacy worker. No launcher is provided or invoked here.")

== Before any release

#text("Obtain explicit authorization for one additional attempt; the old 1/1 allowance is consumed. Create a successor admission binding that authorization, current budget, a live runtime reservation and fence, new source/config/command publication and prospective independent review. Do not silently toggle this frozen packet or reuse the source-preparation lease. Reconfirm absence of all proposed directories and unit at actual release. If any exists, refuse; a changed path requires a new published binding. Prior failure evidence remains unchanged.")

#text("CPU remains 2, physical memory 4 GiB, swap zero, tasks512, one active payload. Proposed total charge is900seconds including supervision, cleanup and sealing; native880, systemd885+5stop, timeout895+5kill remain unchanged. 46800 total minus4500 charged leaves42300; an approved900 charge would leave41400, then1800 validation leaves39600. Deadline remains2026-11-06 04:00UTC. This is arithmetic, not allocation or permission.")

== Effective limits and ownership readbacks

#text("Before release record `systemctl show --property=DefaultLimitAS --property=DefaultLimitASSoft --property=DefaultLimitDATA --property=DefaultLimitDATASoft`; these manager defaults are observations, not the actual worker limits. The exact command introduces no new LimitAS/LimitDATA override. Immediately after launch capture the live InvocationID, MainPID, ControlGroup, boot ID, actual `/proc/<captured MainPID>/limits`, unit LimitAS/LimitASSoft/LimitDATA/LimitDATASoft, and cpu.max/memory.max/memory.swap.max/pids.max plus cgroup.procs/events. Capture limits only for the matched invocation/PID. Removing setrlimit does not remove inherited limits. If the worker ends before live limits can be read, preserve the missing readback and continuing mapping risk; never invent unlimited effective limits. This manual observation is not an enforced pre-Git handshake. No native success can be inferred from command exit0 alone.")

#text("Use the already reviewed OPERATE.md ownership procedure: capture identity, stop only that invocation when appropriate, verify captured group empty/absent, bind native receipt and durable output hashes. A different or unknown owner blocks further work. No automatic retry or compiler. Record actual filesystem via existing findmnt; do not copy old XFS assumptions. Preserve partial files and all errors on failure.")

== Materialization and validation boundary

#text("All historical input hashes remain pinned, including revision29861c9a55f9ae5a33e1e09b3866aa397fbde605 and candidate-pencil555a5c62339a927579599d46c7e94265e52497dc56bacf0b13bb53c2dcd133c4. Complete archive/tool/cache hashing is future worker work. No materialized-inputs.json is fabricated. Only successful preparation can produce and seal that manifest; validation must then publish it with its own exact concrete admission. Existing validation scope stays default-feature offline locked root-library compile, four ordered exact controls including actual Some/None, and full unfiltered suite under1800seconds/2CPU/16GiB. The source-selection difference between materializer and validation must remain explicit.")

== Owner-waiver helper source inspection

#text("owner-waiver-delta.patch compares the independently reviewed durable-v2 helper to the existing owner-waiver-v3 helper. The delta changes schema, replaces finite dollar allocation checks with hash-bound owner instruction/scope/approved fields, and adds exact deadline plus integer remaining-worker checks (at most46500). Test-list equality, attempt1/no automatic retry, recipe hashes, publication receipt, input digest, lease fields, paths, durable output handling, sealing and cleanup code are unchanged. Current42300 fits the historical ceiling; this ceiling is not a live budget ledger. This producer inspection is not an independent review, and neither helper was imported or executed. The separate independent-review gap is the waiver delta plus its future concrete admission; do not infer it passed from the v2 or mapping reviews. Owner-waiver approval must not be used as additional-attempt approval.")

== Sources and status

#text("Sources checked: frozen mapping review, failed attempt cleanup/receipt, prior OPERATE.md, durable-v2 recipe/worker/review and owner-waiver source. Prior primary getrlimit and Git configuration documentation explain the virtual-address versus physical-memory distinction; exact failed mapping length remains unknown. KB tools/wrapper remain unavailable in this sparse runtime; frozen bound source is the fallback. No new literature or performance claim. Source-only, PCC unverified; no compile, test, native execution, runtime resource or retry was started.\n")

#image("gates.svg", width:100%)
