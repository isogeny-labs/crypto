# Native plan preparation 01

Classification: SOFTWARE, planning only. Use the retained clean-build-01 crypto
binary and the seven immutable controller metadata files from source package 01.
Do not generate cells, execute cryptography, freeze an execution protocol, or
claim that a plan is an executed population.

Invoke `crypto dataset plan` exactly seven times: the six frozen native cohort
definitions, then planner-coverage. Preserve parameter order and both twists.
Use each cohort's exact seed/limits/policies; the broad plan uses seed42,
factor-base size2, degree4, work1, matching the native controller's population
fixture. Each command has a 15-second timeout and 5-second termination grace.
Stop on first failure; no automatic retry or extra seed search.

Retain separate argv, stdout, stderr and exit records; exact plan-file hashes;
native typed plan hashes; and the native V2 protocol-scope records. No external
implementation may reconstruct those hashes. Require the frozen counts
192,384,64,64,64,64,580608 and zero generated cells. Leave protocol_sha256 absent
or null. These are immutable unbound inputs for later child-protocol preparation,
not executable frozen acceptance plans.

Verify source package identity and retained binary before and after, along with
all seven metadata hashes. Save all outputs under a new attempt directory and
archive them with a fresh S3 GET comparison. Planning can run while Cargo tests
use the separate shared target: the retained executable and source package are
immutable and planning starts no worker or additional compiler.

The broad plan preserves n23..31 and47..61, m2..15, all three curve families,
twists, twelve method spellings, two policy forms, two curve seeds, two
generators and three keys. This software cross-product does not replace the
larger production dataset design or assert supported native execution in every
cell. Runtime estimates and workflow readiness remain unset until measured
evidence is available; these planning receipts do not satisfy that gate.
