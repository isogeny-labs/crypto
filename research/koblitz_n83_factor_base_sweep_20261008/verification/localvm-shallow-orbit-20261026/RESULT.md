# Matched N83 shallow-circuit search on retained K=1,182 and K=2,048 bases

The schoolbook circuit completed the 5,000-conflict diagnostic at both
retained bases on native ARM64 Linux. Shallow Karatsuba completed the same
diagnostic at K=1,182 but reached the 280-second outer cap at K=2,048. This
is a source-matched result for one public query under a fixed solver budget.
Every completed search returned zero SAT models, zero verified relations and
rank zero, so the measured stage cannot yet rank the bases by complete
single-target runtime.

The primary `a=0` curve uses the binary field with polynomial
`z^83+z^45+z²+z+1`, subgroup order `2417851639230796216685689`, and the
public fixture point `(6261116474805304534310590,
4505575220639647215230954)`. Both `public_x_hash` bases use seed
`2026100801`. K=1,182 has 196,212 distinct stored points and 1,183 matrix
columns including the target column; K=2,048 has 339,968 stored points and
2,049 columns. The exact compressed-object SHA-256 digests are in the
[frozen plan](evidence-01/rosetta-plan.json). This comparison deliberately
varies both the base size and the product circuit. It does not treat the
orbit count as the actual point count.

All eight arms used source commit `88893c8deb6fa19e490c945f5d89d2a7220bf8c8`,
five summands, the signed-Frobenius orbit-phase domain, fixture zero, one
trial, 150,000 SAT variables, 1,000,000 domain clauses, at most 32 models,
5,000 conflicts, one CPU, 4 GiB hard container memory, zero swap, no
network, and a 280-second outer wall cap. The frozen order within each
architecture was K=1,182 schoolbook, K=2,048 shallow, K=1,182 shallow,
K=2,048 schoolbook. The outer wall includes base import, worker startup,
search and supervisor validation; it is a stage diagnostic, not the
single-target online interval. The ARM64 repetition was chosen after the
initial x86-emulated panel to remove emulation overhead. Both binaries came
from the same source commit, but their architectures and compilers differ.

| Linux worker | Base | Circuit | Outer wall (s) | Status | Conflicts | Models | Verified relations | Rank | Cgroup peak (MiB) |
| --- | ---: | --- | ---: | --- | ---: | ---: | ---: | ---: | ---: |
| x86-64 via Rosetta | K=1,182 | schoolbook | 233.040 | `UNKNOWN_solver_cap` | 5,000 | 0 | 0 | 0 | 425.0 |
| x86-64 via Rosetta | K=2,048 | shallow | 280.560 | `UNKNOWN_wall_cap` | unknown | unknown | unknown | unknown | unknown |
| x86-64 via Rosetta | K=1,182 | shallow | 246.052 | `UNKNOWN_solver_cap` | 5,000 | 0 | 0 | 0 | 456.0 |
| x86-64 via Rosetta | K=2,048 | schoolbook | 232.342 | `UNKNOWN_solver_cap` | 5,000 | 0 | 0 | 0 | 581.3 |
| native ARM64 | K=1,182 | schoolbook | 173.087 | `UNKNOWN_solver_cap` | 5,000 | 0 | 0 | 0 | 421.8 |
| native ARM64 | K=2,048 | shallow | 280.501 | `UNKNOWN_wall_cap` | unknown | unknown | unknown | unknown | unknown |
| native ARM64 | K=1,182 | shallow | 208.757 | `UNKNOWN_solver_cap` | 5,000 | 0 | 0 | 0 | 453.0 |
| native ARM64 | K=2,048 | schoolbook | 154.245 | `UNKNOWN_solver_cap` | 5,000 | 0 | 0 | 0 | 578.1 |

At K=1,182, shallow's smaller pre-domain circuit required 35.670 seconds
more outer wall than schoolbook natively, and 13.012 seconds more under
Rosetta. At K=2,048 the shallow worker was killed by the declared outer
cap before a summary, so its conflict count and memory peak are unknown.
The supervisor retained the base-ready and relation-collection-started
events for those censored arms. For each completed arm, the worker's
original-curve query replay passed. The supervisor recorded clean source,
binary, base and fixture hashes and no container cleanup error in every
arm. An end-of-panel query of the dedicated VM showed no remaining N83
search containers.

The local Colima VM ran on host model `Mac16,7` through macOS
Virtualization.Framework with two ARM64 vCPUs, 6 GiB VM memory, virtiofs and
Docker 29.5.2. The worker used one CPU inside that VM. These are local VM
diagnostics without cloud Tier 1 noise/calibration receipts. The x86-64
worker SHA-256 is
`a1d29a516aafd247c352e6de24a85927a7c0f5b4e403ad6f39e4eb993ffc6967`;
the native ARM64 worker SHA-256 is
`7e6069ae87c836e41355a990732ac3622e862aa9989e2c4b023d18e489aa5f5d`.
The ARM64 worker was built with Rust 1.98.0 and Zig 0.16.0 using the exact
command in [environment.json](evidence-01/environment.json). The original
worker binaries and compressed base objects remain at their recorded local
paths outside Git; the eight configs, worker outputs, source snapshot and
file hashes are retained in [evidence-01](evidence-01/).

Run `python3 audit.py` in this directory to verify the retained file set,
source snapshot, binary identities reported by each supervisor, exact
input/cap matching, statuses, completed-arm target identity and replay
receipts. It returns `PASS_bounded_receipts_only`. Its checks do not turn
a capped search into a natural relation-yield estimate. The next solver
step is to retain in-call conflict progress before a wall cap and test
whether a stronger orbit-domain encoding can produce verified natural
relations under a fixed resource envelope. A base choice then requires
measured relation yield, rank novelty, column logs, target descent and a
paired same-point rho interval under the IC benchmark tier contract.
