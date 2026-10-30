# Preparation and development checks

This record describes development, not the requested baseline or cutoff run.

- Literature and pinned reconstruction/acceptance audit read before the design.
- Conductor scope check returned 127 because conductor is not installed.
  Continue with explicitly recorded repository paths; no reservation is claimed.
- Native build 001 failed at link: cannot find -lpari-gmp-tls. The installed
  PARI library has the versioned name libpari-gmp-tls.so.9 without an unversioned
  linker alias. Corrected the link to that exact installed library name.
- Full 60,000 accepted-curve run, extended retrospective sweep, independent
  output verification, percentiles, simultaneous range inference and production
  image/runtime qualification remain unperformed. No numeric result is invented.
- The sampler deliberately has no implicit resume/overwrite. Production
  scheduling, contiguous-prefix recovery and an independent full-journal
  verifier must be qualified before a production launch. A launch-acknowledgment
  argument does not replace checkpoint.sh gate or a committed protocol.
- Builds 002, 004 and 005 exit zero. Build 003's shell session disappeared on
  environment restart; an empty stderr is not reported as its exit status.
  Builds 004/005 were fresh explicit successful builds after that restart.
- All four native development self-tests exit zero. Separate changed-order
  and changed-seed cohort controls exit one with the expected refusal.
  These are tests on inherited fixtures and toy inputs, not new baseline data.
- Review found the sampler's initial status line said 24 toy cases, although
  the b=2 singular case is correctly skipped in each of three toy fields.
  Actual nonsingular coverage is 21. Preserved the earlier stdout locally,
  corrected the label and added an assertion of the exact executed case count.
- The selected managed environment restarted and is ready at configuration58,
  with no injected identities or additional EKS connection. The HTTP proxy at
  port8080 refused the two normal Git fetch attempts after restart. Proxy
  settings and TLS verification were preserved; no direct-network bypass.
- Fresh build006 exits zero after the case-count correction. Its sampler
  self-test exits zero and reports 21 existing model fixtures plus exactly21
  nonsingular toy cases. The other three binaries have unchanged hashes from
  the final005 test set. BUILD.json binds these development identities.
- No full-journal verification, random-curve percentile, 5000-cutoff replay,
  canonical execution freeze or study checkpoint has been completed. Backend
  clarification was requested; no reply or permission is inferred from time.
