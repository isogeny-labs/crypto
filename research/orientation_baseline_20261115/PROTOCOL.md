# Binary two-point decomposition baseline

Frozen before native correctness runs. Domain: ordinary binary curves, b nonzero, supported tiny fields GF(4), GF(8), GF(16), GF(32), GF(256). Public known-answer fixtures only.

Compare the complete ordered index pairs (i,j) with F[i]+F[j]=T, retaining duplicate labels. Arms: quadratic direct pair scan; scalar target subtraction and full-point lookup; batch-inverted binary target subtraction and the identical lookup. Exact ordered outputs must match on every case. All return paths include T=O, base O, target equal to/inverse of a base point, doubling, x=0, repeated labels, empty bases and absent relations. No pair-sum table or cross-target relation cache.

Check every target on six curves per field at degrees 2,3,4, full and non-negation-closed bases. Add GF(256), a=0,b=1 bases of sizes 8,16,32,64, and eight deterministic public targets including O and the finite x=0 point. Every nonzero field element must have a valid inverse. Compare measured arithmetic counters (multiplications, squarings, inversions; inversion internals are counted), membership lookups, and group additions. Charge membership construction per target in the cold contract; report inserts and output rows separately. Operation counters are algorithm development checks, not calibrated wall-time measurements. Stop on any mismatch.

The previous orientation and hybrid correctness controls run unchanged as an imported test module. Use the old pinned source and log without rewriting them.

Timing follow-up (not admitted here): EKS L2, one warmup, five alternating measured rounds, same inputs and all ordered outputs, A/A baseline, full cold membership construction and output materialization, separate reusable-setup diagnostic. Include the current fastest applicable external implementation on matched inputs before any fastest claim. Proposed job ceiling USD 5 including build/failures, 4 CPU/4 GiB, 900 seconds absolute execution limit, zero retries. Admission requires live pool capacity and account daily/monthly remaining-budget readback. No local or CI-runner timing substitutes for missing L2. No rho/pipeline/leaderboard claim is planned for this component.
