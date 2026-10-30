# Cross-track review controls

Scope: internal cross-track source review, independently authored from the reviewed implementations. This is not an external formal review. Source is read-only; all review artifacts stay here.

Before targeted controls, inspected the prime fixture/MITM/strict oracle, independent Jacobian verifier, disk-backed fixed-Macaulay reducer and checkpoint code, resource launcher, quantum resource model, finite Fourier law and their frozen protocols/reports.

Controls to execute:

1. Copy strict result/certificate files; alter only a certificate's target ID to another recorded target on the same curve. A complete workload-bound verifier should reject. Original scientific records stay unchanged.
2. On a copied prime fixture, submit four repeated copies of a valid factor-base point, with their exact sum as the copied public target and a `distinct_abscissae:true` certificate. A strict constraint verifier should reject.
3. Empty copied certificate file: a verifier claiming coverage of found rows should reject missing certificates.
4. Independently recompute one retained r=5,Q=16,d=2 outcome law by direct 2D DFT, not FFT. Check all output probabilities, Parseval normalization and butterfly count. Check the exact-congruence/resource rows using separate integer arithmetic.

No expensive research trial, solving-degree run, exponent fit or performance benchmark will be repeated. Any failures found here will be reported with their limited scope; they do not automatically invalidate correctly generated frozen records.
