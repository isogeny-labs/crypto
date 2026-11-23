# Kernel proof and data bridge

The source is ../lean/WorkLimits.lean, pinned to Lean4.34.1. The four vectors
follow the declared ten-feature order in ../plan.json. Native export.cpp
copies their literal values from the independently checked profile.tsv into
../lean/count-data.json; literal-token matching then binds the Lean definitions
to that record. It does not derive the counters using Lean or Python.

The proof assumes a cost is coordinatewise nondecreasing in those ten counts
and its comparison is transitive. It proves that accepting prime239v1's vector
under an upper budget necessarily accepts both named omitted vectors. It
also checks the four exact dominance relations, including prime239v3. These
assumptions are not established for an original caller or actual runtime.
No mathematical prime certificate or ECDLP security statement is formalized.

Download the exact official release in toolchain-release.json, check its
580432872byte length and SHA256
47bf4bbd78f70c2e9670598ab7124d92b6efb7330ff33e5fbb4030f6fd72e4e4,
and unpack Lean plus its libraries. From the lean/ source directory, with
the verified binary directory on the command-local PATH:

```sh
lean --threads=1 --memory=1024 -o /tmp/nist-work-proof/WorkLimits.olean WorkLimits.lean
```

Create the fresh output directory first. Run the unchanged repository auditor
from the repository root, pointing LEAN_PATH at that compiled-module directory:

```sh
LEAN_PATH=/tmp/nist-work-proof lean --threads=1 --memory=4096 --run tools/lean/AuditAxioms.lean -- WorkLimits
```

The actual build and auditor both succeed. All eight theorem declarations
have empty transitive axiom lists. No sorry/admit/user axiom or trusted native
evaluation is accepted. The unavailable lean4checker was not run. The standard
repository proof gate uses this same auditor; here the verified standalone
compiler and auditor were invoked directly because elan/lake were absent.

attempts.json and the raw archive retain all earlier environment/elaboration
failures. A4GiB virtual-address cap prevented thread startup; the successful
build uses one worker and a1GiB internal allocation limit, the auditor4GiB,
with no virtual-address cap. These flags are limits, not measuredRSS or speed.
