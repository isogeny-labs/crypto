# Native paired optimizations

Run 20261019-013858-boolean-native-vector-gr-799a77. Exact frozen baseline and candidate builds on one c7i.2xlarge on-demand VM. Five A/A and A/B pairs; independent outputs/counts checked.

CPU: Intel(R) Xeon(R) Platinum 8488C. Scalar/portable ablations, direct legacy-to-candidate ratios and all native decisions remain in optimization-summary.json. Samples are not pooled with another host.

The generalized API dispatches to an existing SIMD Gray kernel and retains exact counts and scalar fallback. The matrix switch remains opt-in. These are bounded solver improvements; no asymptotic or whole-IC/rho claim is implied.
