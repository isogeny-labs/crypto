#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Materializer mapping correction: independent review
[pcc:unverified] Published candidate ec632eadbadadae6d378162e3334d6dcae9e6068 removes only the explicit RLIMIT_AS assignment. All other source bytes match the predecessor. No blocking source-delta defect found; machine status remains not_verified. No new attempt released.
#image("scope.svg",width:100%)
== Native evidence
Prior Git ls-tree exited 128 with a pack-mapping allocation error, zero materialized files and no compiler. Virtual-address pressure is consistent with that record, not established as the definitive cause. Git supports windowed mappings: pack size alone does not prove the failing mapping size. Resident-memory demand remains unknown.
== Source trace
Line 10 removes the address-space assignment. Lines 61–66 require the expected cgroup and CPU ≤2, memory 4 GiB, swap zero and 512 tasks before external commands. Deadlines/heartbeats, owned-group cleanup, input hashes, final rechecks and byte/path limits are unchanged. These are source observations, not fresh runtime enforcement checks.
Removing setrlimit does not raise inherited limits. Future admission should record those effective limits or preserve the unresolved risk. Proposal alone lacks fresh configuration/helper bindings and is not an executable release. Fresh unique paths, lease, published admission and finite additional-attempt release remain required. Prior attempt remains consumed; 900 additional seconds are only proposed.
== Review method
Preread plan frozen before candidate inspection. Reviewer did not author candidate; disclosed failure and proposed direction were known. No test, mock, compiler or materializer was run. Primary references: Python resource.RLIMIT_AS and Git core.packedGitLimit documentation, linked in REPORT.md. Candidate hash, source joints and limitations are in result.json and joints.csv. No scientific status promotion.
