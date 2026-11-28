#set page(paper: "a4", margin: 16mm)
#set text(font: "DejaVu Sans", size: 9pt)
= Native Vieta/coset scale measurements
Rust component study · local CPU4 · 15 November 2026

*The Rust port is merged in isogeny-labs/crypto PR 4340.* The measured extractor
avoids interpolation and returns complete ordered relations through Vieta
remainders, gcds and a separate diagonal branch. All statements here are producer
observations, PCC unverified; no independent signed standing is asserted.

#table(columns:(1.4fr,1fr,1.3fr,1.3fr),inset:5pt,
 [Conditioned two-term queries],[Coset m],[Vieta multiplications],[Batched lookup multiplications],
 [62-bit field],[8,192],[628,385,351],[49,364],
 [P-224],[2,048],[41,002,860],[13,419],
 [P-224],[8,192],[10 retained deadlines],[All 10 retained controls completed])

Values are native medians over five retained conditioned targets per successful
row. Median paired Vieta/lookup multiplication ratios are 12,729.6 at 62-bit/m8192
and 3,055.6 at P-224/m2048. Inverses and square roots include their primitive
multiplications. Additions, hashing, allocation and sorting are not priced by
this metric. These are generic u128/BigUint references; the library's optimized
Montgomery and cached square-root backends were not measured.

#image("two-counts.svg", width:100%)

#pagebreak()
== Scope, controls and retained failures
The original attempt covered four fields (17,31,62,224 bits), all 20 two-term
configurations at m=32,128,512,2048,8192, and all 12 three-term configurations at
m=16,64,128. Each configuration has two warmups plus ten retained targets,
interleaved arms and A/A controls, with random-rational and conditioned-sum
populations kept separate. Random targets are not assumed unsatisfiable.

The original attempt recorded 2,100 of 2,112 planned arm calls: 2,088 succeeded
and 12 Vieta calls at P-224/m8192 hit their 120-second stage deadlines (two warmups,
ten retained). Its final three-term configuration stopped after round 6 because
an untimed fixture clock incorrectly included previous arm time; twelve planned
calls were never reached. The incomplete epoch is preserved.

A separately frozen source epoch repaired only that fixture lifetime and repeated
the complete affected configuration: all 96 arm calls succeeded. Separate-process
wide scalar replay checked all 2,184 successful calls across both epochs with no
set mismatch, retaining the twelve algorithm failures. The repaired epoch is not
pooled with its incomplete predecessor. Raw records and native summaries retain
both populations, all warmups and all failures.

== Cold cost is a different boundary
#image("cold-ratios.svg", width:100%)

For one target, the candidate pays descriptor plus extraction; lookup additionally
pays factor-base construction and the membership set. At P-224/m32 and m128 the
median conditioned cold multiplication ratios are 0.4566 and 0.5514; at m512 and
m2048 they rise to 1.4705 and 5.1788. The small cold reductions are specific to
this generic reference, whose plain Tonelli–Shanks setup is expensive. They do
not establish an advantage over optimized factor-base construction.

#pagebreak()
== Three-term comparison
#image("three-counts.svg", width:100%)

Streaming Vieta uses more multiplication work than the matched pair-table MITM
control throughout this panel. At repaired P-224/m128 its median conditioned
warm ratio is 637.9; charging table construction gives a cold ratio of 8.1072.
Nested batched lookup is also retained. Pair-table construction is a separate
charged phase; warm query figures never silently amortize it into a one-target
claim. The repaired P-224/m128 point uses five conditioned targets; the old
incomplete epoch has only three and remains separately recorded.

#pagebreak()
== Host, custody and admission
The host is a 5-vCPU KVM VM reporting AMD EPYC 9V74 and kernel 6.18.57. The
unmodified isolated runner reserved CPU4, moved 80 threads, detected 2.23 seconds
of other process CPU over the original 1,918.50-second driver interval, and marked
that interval uncontended. Peak process RSS was 26,712 KiB. PSI telemetry is
absent and host-owned threads could not be moved. *All clocks are below-L2
diagnostics, with wall_valid=false for L2 comparisons.* No timing speedup is
claimed. No builds, tests or heavy analysis ran during either measured interval.

The user explicitly authorized isolated local execution with Git-only checkpoints.
Source, protocols, copied qualifier hashes and distinct pre-call manifests were
pushed before calls; finished panels were checkpointed during the attempt. Raw
bytes, failed rows, original and repaired qualifiers and native analysis are
retained. No cloud spending occurred. S3/independent-admission gates are not
claimed satisfied and no canonical benchmark/IC leaderboard row is promoted.

== Export qualifications
The primary two-term export carries every planned row, including its twelve
timeouts. The original three-term export explicitly reports the twelve missing
calls and its checker flags the shortfall. The repaired protocol accidentally
retained a stale copied instrument commit field; the executable hash and actual
source publication were pinned before calls. The export records the actual
repair source and the checker flags this metadata mismatch. A post-call correction
was refused by the protocol tool and is retained as a draft/erratum; frozen
protocols were not rewritten. These qualifications prevent a claim of flawless
protocol adherence while leaving raw outputs and executable custody reviewable.

== A separate last-fall-degree argument
LAST_FALL_BOUND.md gives a hand argument for the exact binary power-chain lift:
for m=2^d, p>3, m dividing p-1 and nonzero c, the standard last fall degree is at
most 2d+2. In particular, the bound is 28 at m8192. The proof uses compressed
projectors of degree d in the gate variables, not degree m-1 in x; local slope
algebras cover both nonzero and zero target ordinates. This argument awaits
independent or formal validation and is not inferred from the measurements.

The degree bound does not price a Macaulay computation, bound lexicographic
elimination degree by a constant, cover the joint three-term ideal, or establish
an ECDLP speedup. The executable does not construct the proof's projectors or
perform interpolation. The measured schoolbook polynomial implementation remains
expensive as m grows.

Sources and reproducibility: PROTOCOL.md, LOCAL_AMENDMENT.md,
FIXTURE_CLOCK_AMENDMENT.md, frozen protocols, native/ and scale-20261115T2053Z/.
Prior work includes Petit–Kosters–Messeng section 3.3 and repository prime_fast,
prime_tower, pkm_smooth_tower and fp_backend. The last-fall definition was checked
against Huang–Kosters–Yang–Yeo, arXiv:1505.02532, definitions 2.1 and 2.5. KB was
unavailable; primary/internal sources were read directly. No novelty claim.
