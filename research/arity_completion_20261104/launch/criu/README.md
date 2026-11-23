# Arity completion on the CRIU/EBS command mode

- Runtime archive `inputs/recovery-runtime/be4a5aea….tar.gz`
  (`taskq/deploy/eks-karpenter/resume/runtime.command.json`, build record
  `runtime-BUILD.json`): the pinned runtime `3ab3cf23…` with only
  `isolab-eks-recovery` rebuilt (Rust 1.99.1, musl) from this PR's controller
  source; `criu` 4.2.1, its libraries, `tar` and `isolab-criu-progress` are
  byte-identical. The default `runtime.json` pin is unchanged.
- Image `image-criu.json` (`isolab/tasks@sha256:fd4a039b…`): the prime-v2 study
  image plus one layer with `/opt/isolab/recovery/` (the runtime above),
  the CRIU-mode `job-lib.sh`, `job-criu-qual.sh`, and the resume records under
  `/opt/study/resume/{koblitz,prime}/` (attempt-2 records; the prime copy omits
  the interrupted `isogeny-m5-t2` files; no old provenance files). User 0:0,
  as the lab's runtime images.
- `request-qual.json`: the qualification run (prime-tower control
  ctl-m3-n15-kummer, checkpoint every 120 s, up to 10 restores, 8 GiB, 16 GiB
  PVC). It passes only if the restored computation's rows equal the recorded
  rows except timing fields, after at least one worker loss.
