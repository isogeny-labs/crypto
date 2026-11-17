# Independent instrument review, 2026-10-31

This is a source and retained-development-artifact review of the three prepared
tracks. It is **not an experimental result**. This reviewer did not execute
the exhaustive audit, elliptic F4 panel, or degree-3/4/5 construction ladder,
and did not launch a substitute or reduced panel. The parent is responsible
for execution admission and the eventual result report.

The mathematical scope is in [MATHEMATICS.md](MATHEMATICS.md). No defect was
found in the supplied secant identity, finite-target completeness argument, or
the stated presentation-specific top-ideal degrees. This review does not
establish literature-wide novelty, measured F4 degree reduction, improved
end-to-end index calculus, or any result on P-256 or 17-term relations.

## Track 1: complete fixed-fixture audit

The source enumerates both prescribed signed bases, all 18 finite targets,
all four sign branches and all 17^3 assignments per branch. Dense comparison
uses all 17^2 abscissa pairs. The independent reference enumerates group point
pairs; it is not obtained by filtering through the equations being tested.
Every accepted root is converted to its signed points and replayed through
group addition. Inverse pairs at the identity are checked separately from
the finite-target system. The acceptance predicate includes dense-witness
group replay failures as well as structured/dense false positives and false
negatives.

The all-target aggregation checks now require 56 finite ordered relations,
56 model roots, eight doubles, 32 finite unordered relations, eight ordered
inverse pairs, paired-target count symmetry, and an even SAT-target count for
each eight-point base. These checks are independent consequences of the full
signed population; the expected coverage values in MATHEMATICS.md remain
analytical calibration expectations until a complete admitted run exists.

Both source-only exporters preserve the complete target population. The F4
export binds the intended ordinate polynomial, emits the four secant branches
and a genuinely two-variable dense system, and does not enumerate expected
roots. Source-only export does not constitute completion of this audit.

Field counters explicitly exclude polynomial construction, integer loop work,
hashing, serialization and IO. Inversions include their exponentiation work;
they cannot be priced twice. Payload bytes, named readback bytes and the Linux
RSS snapshot are distinct from physical disk traffic and per-phase list
storage. Resource enforcement and the checkpoint/backend receipt remain the
external caller's responsibility.

## Track 2: actual stock F4 adapter and verifier

The C adapter follows the pinned upstream msolve-neogb initialization, core F4
and export path at revision
`3672113ee2c696a2ac9c637636e0f7214f500ba1`. The algorithm source is unchanged;
the adapter uses one thread, DRL and exact sparse linear algebra, with a fixed
defensive libc seed. This is not a Magma/Singular comparison or the full
msolve CLI.

The native verifier reconstructs every coefficient of the intended dense or
secant input independently. Candidate splitting is checked coefficient-wise.
It constructs the branch reference from group pairs and the intended signed
ordinate polynomial before checking the equations. It rejects an equation
set that excludes valid reference assignments. This repairs an earlier
reference-oracle weakness: filtering reference pairs through the input
constraints could have credited a malformed overconstrained system with an
empty reference.

Preflight requires every affine target on each supplied curve, both candidate
and matched control roles, exactly one dense system and all four distinct
secant sign branches per base and target. The signed inventory and intended
ordinate polynomial are bound across branches and targets. Dense systems
instantiate two variables; a padded free slope variable is not retained.

The root extractor recursively specializes equations, prunes only on exact
univariate evaluations or nonzero constants, and otherwise enumerates the
remaining field coordinate. Thus a completed traversal enumerates all
rational roots of the returned basis. The two-million tested-value cap is a
failure/incomplete outcome, not an UNSAT certificate. The verifier checks all
input normal forms and the Buchberger S-pair criterion, then compares the
complete rational root set with the independent group reference. Reverse
ideal membership and scheme multiplicity equality remain explicitly
uncertified. No claim of full ideal equality follows from rational-root
equality alone.

The metrics are kept distinct:

- maximum imported-generator degree;
- maximum degree of a selected F4 pair in the stock trace;
- maximum exported-basis degree;
- `observed_f4_degree`, explicitly the maximum of the first two;
- the independently derived homogeneous top-ideal degree.

The selected-pair degree is not certified to be an inherited sugar degree.
Missing selected-degree evidence when pairs were reduced leaves the observed
degree unavailable. Stock matrix maxima reported as dimensions of the
largest-area matrix are not independent row/column peaks; separately parsed
trace shapes include the final/intermediate basis-reduction matrices.

A prelaunch artificial case exposed an upstream API-context trap. `core_f4`
prints counters from a local metadata context; its caller's context retains
zeros. Reading those outer zeros would undercount actual F4 work. The final
adapter reports its direct counters as null and no longer prints a duplicate
outer zero summary. The native verifier normalizes counts from the first
actual core statistics in preserved stderr. The retained artificial case
visibly has degree-3 and degree-4 rounds, two pairs, seven rows and a final
3-by-7 matrix. Its older duplicate-zero trace is retained for a parser
regression. This is software verification on a non-elliptic ideal, not the
requested elliptic experiment.

Full field-operation counts remain null. Adapter timings are explicitly L0
shared-host diagnostics, not qualified speed measurements. Timeout, raw
output, verification failure and unattempted cases must remain visible in the
eventual report. The source imposes a 10-second child deadline, checked 2 GiB
inherited address-space limit and an admitted outer deadline. No successful
elliptic F4 panel is present in this reviewed work.

## Track 3: prospective split-graph ladder

The frozen panel is p=83,101,127 and d=3,4,5, seed 315, with 256 curve
proposals per field and 512 graph proposals per cell. Curve proposals are
selected before graph outcomes and require nonsingularity, the stated
exception exclusions and full prime group order. Graph proposals interpolate
hash-selected signed points and accept only exact degree-d polynomials with
2d distinct rational roots. Complete coefficient factorization is replayed.
No success rate or existence result is inferred from this construction code.

The low-x and hash-x controls have the same abscissa cardinality, independent
ordinate orientation and mandatory explicit membership constraints. Duplicate
signed point-set controls remain recorded. The exporter retains all finite
targets, dense inputs and four signed secant branches per arm, with independent
group-pair expected roots checked against exported constraints. It asserts
K^2-K finite ordered relations, K doubles, K inverse pairs, K^2/2 finite
unordered relations and target-negation symmetry.

Equal-cardinality signed bases have equal mean ordered yield over the complete
target population. SAT coverage, its distribution and duplicate structure may
differ. Row count is not row rank. An accepted graph's derived degree d+1 is
not a measured F4 degree; actual F4 results belong to Track 2. For these ladder
sizes, an exact control interpolant fitting 2d curve abscissae must have degree
at least d, so the stated control top-ideal calculation does not silently
invoke the quadratic exceptional case.

Construction, control and reference-validation counters are separated. The
accepted factorization-certificate work is explicitly named as a subset of
construction work after the completed proposal entry. Coefficient export and
artifact hashing remain null, and physical disk traffic and peak RSS are not
inferred. This is not a complete end-to-end operation count. ICV1/FB1 encoding
parity is checked against the existing fixed fixture; central-registry/EC1
joins remain pending.

Completed proposals and cells are journaled. Failed identities retain exact
coefficient or target/root/generator context. A hard kill can leave RUNNING and
partial journals without a final manifest; such a directory is partial, not a
complete nine-cell result. The global soft limit and outer hard limit remain
prospective execution limits rather than proof of completion.

## Development evidence and reproducibility limits

This reviewer read the retained logs; the reviewer did not rerun these tests.
The logs report six audit tests, nine F4 tests and seven ladder tests passing,
with strict Clippy and release builds. They are development checks, not the
scientific panels. The F4 adapter's two artificial smoke ideals exercise its
external path; the curve inputs, all-target audit and ladder remain unrun.

The reviewed principal source hashes are recorded below. The source revision,
complete frozen protocol, dependency locks, binary hashes, raw inputs/outputs
and admission receipt must all accompany a later run. Source changes after
these hashes require review before carrying these conclusions forward.

| Relative path under `parallel_20261031/` | SHA-256 |
| --- | --- |
| `audit/src/lib.rs` | `e31decae0897ef1847819388a23846646ccc8e0dc8d4e5b083e822389d18a129` |
| `audit/src/main.rs` | `66e90abad09f25ff58b08387f017519d02a645600457c58080ce3af21d082d32` |
| `audit/protocol.json` | `bd3be34dc6dacc2bcfb2c488719cd4096f0b812d7df3843bf118bff10c442d0d` |
| `f4/main.rs` | `e201eda231b41820b1733eefd725841036075aadceb9d4cada92926893025842` |
| `f4/neogb_driver.c` | `94f812b3843a3a8782463636fb666843869533ed65e3c19c94a7bbb57148f653` |
| `f4/build-neogb.sh` | `6f4ceef2b41fc1906fd702c1ac5e2ddb690aa1dd74afe3a6e0acb4cce5c8ee9f` |
| `ladder/main.rs` | `2ee8a2d184478ac62823ccc8a8dcdbf1519d0395ebcda320254a608456364ed2` |
| `ladder/core.rs` | `7fbfcb718437b09aba894d02703f6d7359a8a87fee4a2bb118d5a6da3d9d9ece` |
| `ladder/spec.json` | `e605b759b529ca27eb19169eb1c6a0da07d3d52ad15ebb7f8ddd336ca9b340c7` |

The final local F4 driver hash recorded by its build owner is
`9f6097cede0a27aeafa2896f0eb688bc0d494d25e54d3ecb196ec28fe48a76b8`.
Compiler/CPU portability and rebuild reproducibility still need the retained
build environment; this binary hash alone is not a performance certificate.

No remaining defect was identified in the inspected source that would
invalidate the prepared toy comparisons under their stated limits. That is a
review finding, not verification of unexecuted outputs. Completing the user's
three experimental requests still requires admitted execution, preserved
complete result populations and interpretation of all failures and costs.
