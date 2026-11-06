# Pre-exec PID gate: independent source review

## Source finding

No blocking discrepancy found at the explicitly accepted boundary: actual kernel PID-limit observation before the tested executable exec. This is source review of a proposed helper; it is not a compiled or executed qualification result.

## Kernel mapping

The gate requires PID1 and real/effective UID/GID1000, exact 0::/ membership, one read-only cgroup2 mount at the expected root, no-symlink directory traversal, cgroup2 filesystem identity and read-only mount flags. It reads pids.max=1, pids.current=1 and cgroup.procs containing only 1 using bounded exact byte comparisons. The limit/current observations are repeated before receipt publication.

## Exec and receipt path

Only Q1/Q2/Q3 and a lowercase fixed-length UUID-shaped nonce are accepted. A bounded JSON line records case, nonce, identity, cgroup device/inode and monotonic observation timestamp. It is fully written before execve replaces this process with a fixed executable/argument vector. Q1 uses the retained request digest; Q2 and Q3 use fixed incorrect hashes. Internal descriptors close before exec. No fork, clone, shell or thread-creation call appears in the gate. Pre-exec errors return126; exec failure returns127.

## Parser boundaries

Bounded reads reserve a terminator byte and reject a full payload-cap read without proven EOF. Exact comparison rejects missing newline, extra bytes, embedded NUL and alternative numeric spellings. Mount parsing rejects writable, wrong root, missing and duplicate expected cgroup2 mount records; overlong mountinfo fails closed. It intentionally supports only the declared private unified layout. Administrative cgroup changes after observation and container startup before the gate are outside the claim.

## Required next software bindings

The fixed control plan names positive/negative parser categories but does not yet provide a sealed runnable test translation unit and exact fixtures. Pin that helper and fixtures before native control admission. Also bind qualified musl compiler/header/spec inputs, one120s compile, ELF/call-path readback and the gate binary digest. Verify host PID is not1 before the fixed host-negative test. No native compile or control was run during this review.

## Supervisor integration

Retain one container, existing20s case and300s sequence caps, readonly executable helper mount, fixed UID/mount/entrypoint checks and no image-content change. Require exactly one first receipt with matching case/nonce and the original payload exit/output afterward. Gate126/127 cannot satisfy Q2/Q3 negative cases. Record before-tested-exec=true only after actual successful receipts; before-container-start remains false. A source-only no-fork inspection does not replace runtime qualification.

## Primary reference

Linux kernel cgroup-v2 documentation, PID controller and namespaces: https://docs.kernel.org/admin-guide/cgroup-v2.html . It describes task-count limits and namespace-relative membership; administrative migrations remain outside fork/clone enforcement. No scientific protocol, campaign budget or runtime publication is changed.
