# Materializer V2 delta review plan

Written before reading materializer-v2. Inherit independent V1 preread plan and findings in ed2db1be72. Root reports exact published producer64afb6b895, source digestd99f29d81de5efdc6b5277181a713295ff174182ea173beb7ed43813efae34bb. Reviewer did not author the producer; proposed corrections were disclosed, so this is not blind to design. KB was not loaded and wrappers are absent in the sparse checkout; exact published primary sources and prior source review govern this operational delta. No new mathematical mechanism is claimed. [pcc:unverified]

M1: trace read-once/hash-same-buffer/copy and destination comparison to the approved candidate; inspect final initial-input rechecks. Attack: change source between first read and copy; or substitute destination before manifest seal. Require a failed check or pinned copied content, not a freshly accepted arbitrary digest. [pcc:unverified]

M2: trace owned group cleanup even after leader exit, finite wait time within remaining allowance, retained cleanup errors and outer-cgroup requirement. Attack: unreaped process, vanished leader with descendants, changing group membership or insufficient cleanup time. Bounded failure must not promote success. [pcc:unverified]

Compare the source delta for unintended changes to revision, path/include closure, archive checksum safety, cache separation, resource caps, deadline and retry behavior. Inspect the supplied mock script before execution; run only if it extracts isolated functions and mocks native subprocesses, never imports or executes the materializer body. Preserve exact command/output and source hashes; passing mocks do not qualify the live materializer. [pcc:unverified]

Record V1 findings intact and V2 dispositions separately, with concrete file/line evidence. No actual materialization/compiler/scientific payload is authorized by this review. [pcc:unverified]
