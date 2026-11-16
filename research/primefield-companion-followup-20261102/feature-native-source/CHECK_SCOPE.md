# Native source integration checks

One rustc build (120 s), one filtered integration_tests invocation (30 s).
Three methods: exact raw/seed/ratio/resultant/partial match for original three
setup records; three length-three windows at O,P,-P on F509/r479, with checker-only
Euler expected signs; partition and domain rejection. Calibration is included.
These are bounded local software integration checks, not a full recovery cohort,
performance experiment or replacement for the original 4563-case EKS plan.
Dependency test modules are compiled but filtered out. Preserve failed attempts.
