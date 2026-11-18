# R128 handoff

Source/protocol were pushed at `6f9c2c745` before computation. At the degree-7 double base root `(a,90)`, the exact F128 Newton hull is `(0,1),(2,0),(7,0)`; the positive edge residual is `84+110Z²`. The transverse derivative coefficient `84` is nonzero and `84*6=110`, so the next construction is a direct implicit-function lift `s(u)=6u²+…`. Result SHA-256: `cce71416b01d600abbeedbd86f56f647c6c91b7d2ee536d6f730d5e9c79a739d`.

Independent Horner composition, byte replay, changed-source rejection and Lean finite edge checks passed. Do not promote the Newton slope alone to a normalized ramification claim; check the direct lift first.
