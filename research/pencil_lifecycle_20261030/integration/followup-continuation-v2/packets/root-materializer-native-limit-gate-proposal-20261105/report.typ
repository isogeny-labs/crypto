#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Native inherited-limit gate — source proposal

#text("The candidate adds a native receipt and fail-closed AS/DATA qualification before the first findmnt or Git subprocess. It supersedes the manual post-launch limit-observation gap in the inert additional-admission proposal. No native execution, compiler, tests, imported candidate or resource limit mutation occurred. PCC unverified; independent source review pending.")

== Concrete policy and source order

#text("After the existing owned-output and physical-cgroup checks, the worker checkpoints phase native-inherited-limit-qualification and an incomplete native_limits record. It reads /proc/self/limits as bytes, retains the decoded text, byte count and SHA256, and flushes that receipt. It obtains resource.getrlimit(RLIMIT_AS) and resource.getrlimit(RLIMIT_DATA), retaining exact soft/hard values and their comparisons with resource.RLIM_INFINITY. It checkpoints qualified only when all four values equal infinity; otherwise it records rejected and raises before any child command. Read, decoding, getrlimit or durable checkpoint failures propagate to the existing failure/terminal receipt path and cannot reach findmnt. If output storage itself fails, complete receipts may remain unavailable, but no child is launched from this gate.")

#text("This conservative policy uses no guessed mapping-size allowance and never calls setrlimit. It can reject an environment whose finite limits would actually suffice. Such a rejection must be retained; changing the policy requires separate review, not a silent retry. Unlimited AS/DATA does not imply sufficient resident memory or successful mapping: the unchanged hard4GiB cgroup, kernel mapping constraints and other failures remain. Snapshot observations occur at gate time; external privileged mutations after the snapshot are outside this source invariant. The policy is enforced by an explicit conditional raise, not by a Python assert. Existing admission guards retain their original assert semantics and exact published python invocation has no optimization flag.")

== Receipt compatibility and preserved behavior

#text("The historical native receipt keys status, revision, compiler_invoked, errors, files, phase, elapsed_seconds, cgroup and child_cleanup remain. native_limits is additive and contains policy, pid, status, limit_mutation_performed=false, proc_self_limits_utf8, proc_self_limits_sha256, proc_self_limits_bytes, getrlimit and rlim_infinity_constant when available. A partially filled record means incomplete observation. A qualified record is not materialization success or ownership proof. Existing final errors, owned-group cleanup, manifest custody and status checks remain required.")

#text("The source diff adds only this gate after cgroup checkpoint and before the existing first cmd call. The helper is byte-identical. Source/input pin29861c9a55f9ae5a33e1e09b3866aa397fbde605 and candidate-pencil555a5c62339a927579599d46c7e94265e52497dc56bacf0b13bb53c2dcd133c4 remain. Existing native880seconds and outer900seconds,2CPU,4GiB,zero swap,512tasks,one owner/no overlap and no automatic retry remain. There is no compiler in this stage.")

== Inert configuration and accounting

#text("The successor config and command use new prospective paths/unit, the new candidate hash and unchanged helper. approval_required=true and launch_disabled=true remain; reviewed=false, source review commit=null, publication=null and runtime lease=null. The unmodified admission branch rejects before owned output. Do not use the source-preparation task lease as runtime authorization. This packet is not an additional attempt approval. Prior1/1 remains consumed; total46800 minus4500charged leaves42300. Proposed900 preparation and1800 subsequent validation are unchanged arithmetic; no new worker debit was incurred here. Deadline stays2026-11-06 04:00UTC. An admitted successor must be independently reviewed, fully bound and published after explicit additional-attempt authorization. No materialized-input manifest or native limit values are fabricated.")

== Source reasoning and review boundary

#text("All subprocess creation is through cmd. In the existing linear try-body the new qualification precedes both initial findmnt commands and later Git calls; no cmd call occurs earlier. Exceptions leave that body for the existing failure/finalization path. The new checks record this process's values, avoiding the external /proc/PID observation race. A successful native gate establishes only the stated source precondition before child creation, not full runtime correctness. Static inspection only; independently review the exact candidate.patch, result.json hashes and expected source-order.csv before any prospective run. Prior mapping review applies to the parent source, not this new gate.")

#text("Sources checked: frozen dbe4d4f9 materializer; prior receipt and cleanup; inert proposal714e95ca; mapping reviewd7222eee; existing reviewer finding on missing native qualification. KB tools/wrapper remain unavailable; exact repository source and prior primary getrlimit documentation are the fallback. No new mathematical or novelty claim.\n")

#image("gates.svg", width:100%)
