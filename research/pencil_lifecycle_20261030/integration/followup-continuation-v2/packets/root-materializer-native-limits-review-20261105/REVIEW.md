# Native limits independent review

Source correction supported; runtime admission pending.

**Ordering.** Only 22 lines inserted after cgroup checks and before first cmd(findmnt). All subprocess creation remains inside cmd.

**Policy.** Worker records PID, proc/self/limits text/hash/size and both soft/hard AS/DATA values. All four must equal RLIM_INFINITY; explicit raise rejects finite values.

**Failure paths.** Read, decode, getrlimit and checkpoint failures exit the try body before children. Partial fields survive where storage permits. Decode failure need not retain undecodable raw bytes. Storage failure may prevent a complete receipt but cannot advance into a child.

**Preserved limits.** No setrlimit introduced. Physical cgroup checks, helper, final input verification, deadlines and cleanup are unchanged. Infinity policy is conservative and does not guarantee mapping success.

**Binding.** Exact published packet and all referenced local hashes match. Candidate SHA4e01f95c and helper identity verified; minimal 22-line insertion.

**Admission.** Reviewed false, publication null, lease null, disabled and approval-required remain. Additional attempt unapproved. Remaining42300seconds and Nov6 04:00UTC deadline unchanged.

Qualified is a policy snapshot, not materialization success or external terminal ownership. External privileged mutations after the snapshot are outside this source invariant. No candidate import or execution.
