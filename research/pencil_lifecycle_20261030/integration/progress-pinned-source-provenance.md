# Located progress source and direct ownership correction

Read-only local evidence, 2026-10-30; no build, payload or cloud operation.

The runtime BUILD.json declares base committed source revision 5499eeb9d72e6e777cb0d28b0ade41afbfff90b7 and source archive ea578888e0cb6c829387579d416561025d9b25b3d9ebabe6835d7d0f4cda7f73. Local Git objects in /home/ec2-user/crypto-pencil-signed-graph-20261029 contain:

`5499eeb9d72e6e777cb0d28b0ade41afbfff90b7:taskq/deploy/eks-karpenter/resume/progress/src/main.rs`

A read-only copy is retained as main.rs here. SHA2567108c2548c8b0bb444effccb6c4b00b0ed1c31b44c8236526da95c38762012aa.

The same commit's `taskq/deploy/eks-karpenter/resume/validation/20261021-ecbench/progress-source-binary.sha256` pairs that exact source hash with executable SHA256144db760cf6d59103380f59f7855625459a1927e0cacd3a444512ba8ca097dd4. The retained actual executable at `/home/ec2-user/pencil-controller-review-20261029/image-publication-20261029/archive-comparison/runtime/isolab-criu-progress` hashes to144db760..., matching runtime/SHA256SUMS. The runtime overlay receipt says only supervisor/BUILD/SHA256SUMS changed, preserving this executable. This is retained source/build provenance, not a fresh reproducible rebuild or remote attestation.

## Ownership correction

main.rs:153–166 constructs ecbench run arguments and calls Unix CommandExt::exec(). It does NOT spawn and wait for a nested ecbench child. Successful exec replaces the progress launcher in the SAME PID; supervisor.rs:284–287 therefore owns the direct ecbench process after successful exec. Existing source labels require care only for pre-exec failure: run() can return an error and main exits1 at170–174. A supervisor wait status alone cannot distinguish that launcher failure from ecbench exit1.

The minimal owner change can stay in trusted root supervisor: verify ecbench executable identity on the owned PID (and original PID starttime), capture raw monotonic/boot/time-namespace metadata, replace waitpid observation with successful wait4/rusage observation, and hash-bind exact runner preparation/journal after quiescence. A valid final preparation/journal and independently verified runner identity support attributing the observed termination to the ecbench session. Missing exec attestation/files or startup failure remains a partial enclosing-payload observation; do not manufacture runner closure. Sampling /proc after exec may miss a very short lifetime, so lack of that observation must stay unknown unless a separately reviewed reliable exec transition mechanism is added.

No progress-source or controller change has been made. The direct-parent correction supersedes the source-absence/possible-intermediate-wrapper statements in the preceding owner-closure-interface-inventory.md; that report remains unchanged except its already-added authorization clarification. Fresh epoch still has the separate shell/direct-child integration described there.

Search accounting: an initial lookup in the wrong crypto-autoresearcher repository attempted an unsuccessful Git lazy fetch; that lookup was stopped. All successful source retrieval used GIT_NO_LAZY_FETCH=1 against the crypto repository's existing local objects. No AWS/EKS calls, jobs, builds or payloads occurred.
