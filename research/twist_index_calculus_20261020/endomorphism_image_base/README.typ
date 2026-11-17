#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: true)
#show raw: set text(size: 7.6pt)
= #text("Twist image bases: completed relation, rank and cold-cost controls")

#text("2026-10-23. Seven quartic extension fields, three seeded records each, ten arms. The requested resumed 189-case run and 21 doubling controls are complete, with 210 independent exact-set/rank/recovery audits. Every polynomial run has zero undetermined systems. All 105 complete timed units (ten arms plus original A/A) and all 630 non-timing replays have been reconciled after Spot interruption. Resource admission is per record and whole round, not inferred from job success.")


== #text("What is attributable to the twist?")

#text("On the identical 9,658 frozen residuals, phi=(1-psi) supplies 61 extra signed physical tuples, a measured diagnostic change of 3.3814%, relative to the same-size doubling image. Its distinct projective augmented row delta is -7 (-0.4106%). Both active bases reach full rank in 21/21 cases and recover the target and every usable factor logarithm. Distinct projective rows need not be linearly independent.")


#text("Through each arm's first full-rank query, phi consumes 5.8688% more finite polynomial systems in aggregate than doubling. These aggregates describe separate targets; they are not an amortized DLP price. Per-target complete elapsed ratios below go in both directions. This experiment establishes no consistent twist-specific complete-cost advantage. A geometry change, extra decompositions and a lower stopping cost are separate properties.")


#text("The mixed-component image base is explicitly constructed and verified. Its L0+L3 image is disjoint from the old L0/L2 base, and its logs use existing columns. Generic doubling also creates a disjoint known-log image of the same size. The exact phi coordinate equation is degree three; the current peeled S4 solver does not retain that equation in its solve. See the mathematical argument (MATHEMATICS.md) and certificate IDC1h7e0104841476f369.")


#image("figures/mechanism.svg", width: 100%)

== #text("Requested requirements and evidence")

#text(size: 6.8pt)[#table(columns: 2, inset: 4pt, text("Requirement"), text("Status and evidence"), text("Finish resumed 189 cases and deterministic A/A"), text("Verified complete: 189 first and 189 A/A exits 0, byte-identical outputs"), text("Finish the doubling control"), text("Verified complete: 21 reference and 21 polynomial cases, first/A/A"), text("Unique physical and folded relation yield"), text("Verified complete: exact same 9,658 residuals; independent signed-set equality"), text("Matrix rank and recovery"), text("Verified complete: 210 independent audits, 189 successful full-rank cases and 21 deliberately inactive-column censored controls"), text("Complete cost of this instrumented implementation"), text("Verified complete under actual fixture boundary (BOUNDARY.md): 105 admitted whole timed rounds and 630 non-timing replays"), text("Twist-specific attribution"), text("Measured comparison against psi, pure L0 and generic doubling; no uniform positive result"), text("Complete GAE price, matched strong rho, independent scalar sampling, growing-size security conclusion"), text("Unresolved; not supplied by these controls"), text("External complete GB comparison"), text("378 fixed systems, eight passes, 3024 checked basis pairs; component comparison only"))]

== #text("Unique yield on the full frozen stream")

#image("figures/full-stream-yield.svg", width: 100%)

#text(size: 6.8pt)[#table(columns: 5, inset: 4pt, text("Arm"), text("Unique signed tuples"), text("Distinct projective rows"), text("Finite signed peels"), text("Full rank / 21"), text("original"), text("1473"), text("1432"), text("1913256"), text("21"), text("psi-control"), text("1473"), text("1432"), text("1913256"), text("21"), text("phi-l2"), text("1472"), text("1449"), text("1913256"), text("21"), text("phi-l0"), text("1865"), text("1698"), text("1957992"), text("0"), text("union-l2"), text("2945"), text("2881"), text("3826512"), text("21"), text("union-all"), text("4810"), text("4579"), text("5784504"), text("21"), text("union-l0"), text("3338"), text("3130"), text("3871248"), text("21"), text("phi-l0-active"), text("1865"), text("1698"), text("1957992"), text("21"), text("l0-peel"), text("441"), text("417"), text("1957992"), text("21"), text("double-l0-active"), text("1804"), text("1705"), text("1957992"), text("21"))]

#text("The psi control reproduces exactly the original signed sets: automorphism copies do not add folded rank. The full-column phi-L0 arm retains unused L2 unknowns and is rank deficient in every case. The active-column arm checks that discarded coefficients are zero; it solves every case. Unions gain raw relations by doing more systems and carrying more columns; they must be priced by their complete stopping costs.")


== #text("Cost per public target: twist image versus doubling")

#text("One target per row, five paired whole rounds, alternating arm order. Ratio = doubling cold / phi cold within each same-origin round. The table gives the median and observed range of those five ratios, not a ratio of independently selected fastest runs. Above 1 favors phi. These are complete instrumented elapsed intervals for the supplied coordinate-encoding fixture; parsing, startup and known-answer construction are excluded. See the boundary clarification (BOUNDARY.md).")


#image("figures/paired-cold-cost.svg", width: 100%)

#text(size: 6.8pt)[#table(columns: 6, inset: 4pt, text("p / seed"), text("Phi cold median (s)"), text("Doubling cold median (s)"), text("Paired ratio median"), text("Five-round range"), text("Original A/A range"), text("37 / 0"), text("0.068650"), text("0.069758"), text("1.001154"), text("0.989790–1.028708"), text("1.000381–1.015917"), text("37 / 1"), text("0.125541"), text("0.123523"), text("0.978979"), text("0.977502–0.987318"), text("0.983755–1.015499"), text("37 / 2"), text("0.125255"), text("0.124436"), text("0.990042"), text("0.982966–0.996666"), text("0.993915–1.009741"), text("73 / 0"), text("0.241243"), text("1.334657"), text("5.524357"), text("5.497047–5.596725"), text("0.999426–1.006326"), text("73 / 1"), text("0.167776"), text("0.752636"), text("4.476432"), text("4.437333–4.504181"), text("0.996332–1.004380"), text("73 / 2"), text("0.974077"), text("0.828428"), text("0.847570"), text("0.842071–0.853045"), text("0.998212–1.003659"), text("97 / 0"), text("1.820251"), text("3.973863"), text("2.180789"), text("2.175206–2.191123"), text("0.997844–1.001333"), text("97 / 1"), text("6.745508"), text("4.019464"), text("0.595827"), text("0.593999–0.604832"), text("0.999529–1.021967"), text("97 / 2"), text("3.336158"), text("7.712071"), text("2.311662"), text("2.276324–2.338670"), text("0.994230–1.002044"), text("157 / 0"), text("1.853565"), text("2.014505"), text("1.085886"), text("1.076037–1.091555"), text("0.995017–0.998806"), text("157 / 1"), text("4.949680"), text("2.006529"), text("0.405438"), text("0.404821–0.406946"), text("0.995883–1.012040"), text("157 / 2"), text("2.343618"), text("1.190425"), text("0.508169"), text("0.506477–0.508620"), text("0.994873–1.005784"), text("229 / 0"), text("22.061632"), text("14.677746"), text("0.665205"), text("0.660190–0.670090"), text("0.989843–1.010847"), text("229 / 1"), text("15.088636"), text("17.048593"), text("1.122205"), text("1.109753–1.133316"), text("0.968125–1.002312"), text("229 / 2"), text("23.465120"), text("12.124593"), text("0.519497"), text("0.509440–0.525666"), text("0.997579–1.004294"), text("241 / 0"), text("16.733039"), text("22.759671"), text("1.359694"), text("1.346917–1.363669"), text("0.981440–1.013276"), text("241 / 1"), text("15.959578"), text("12.211535"), text("0.765154"), text("0.760456–0.768259"), text("0.992172–1.003424"), text("241 / 2"), text("23.920537"), text("42.715544"), text("1.792158"), text("1.752688–1.798900"), text("0.984299–1.012300"), text("337 / 0"), text("25.907803"), text("14.240316"), text("0.549708"), text("0.546135–0.550980"), text("0.996583–1.007085"), text("337 / 1"), text("20.724869"), text("29.929771"), text("1.440101"), text("1.431508–1.445216"), text("0.996345–1.008408"), text("337 / 2"), text("25.332803"), text("11.485804"), text("0.452564"), text("0.449587–0.454562"), text("0.977374–1.004549"))]

#text("The same declared 8 GiB address-space cap, 1,200 s watchdog, one Rayon thread and reserved whole SMT core apply to every measured arm on c7i.2xlarge / Intel Xeon Platinum 8488C hosts. Every admitted interval has exit 0, uncontended isolation and no user thread left on its reserved core. Complete successful intervals include construction/lifts, images and coefficients, symbolic setup, all failed peels, F4, roots/group checks, matrix work, recovery and independent point verification. Setup and five exclusive online phases are stored separately and their exact sums checked. Memory fields include process peak RSS in KiB and separately measured Rust allocation telemetry. No process RSS is inferred from allocation counts.")


#image("figures/all-cold-costs.svg", width: 100%)

#text("All ten arms and every per-target cost remain in cold-costs.csv (results/cold-costs.csv), paired-cold-costs.csv (results/paired-cold-costs.csv) and summary.json (results/summary.json). An asterisk denotes consumed censored work, never a successful cheap solution. Complete rounds are selected chronologically under the frozen resume rule (SUPPLEMENT_COLD_RESUME.md). Original failed, interrupted and contended records remain in the archive. A copied checkpoint is not a new timing sample.")


== #text("Rank trajectories")

#image("figures/rank-progress-1.svg", width: 100%)

#image("figures/rank-progress-2.svg", width: 100%)

#text("The rank axis uses the actual matrix dimension for each arm. Original uses L0 and L2 columns; pure and active-image controls use the justified L0 columns. Query counts alone cannot compare their work, because peel counts and solve costs differ. full-stream.csv (results/full-stream.csv) records exact columns, first full-rank indices and finite systems. The full-rank query field is zero based; these figures show the count of queries consumed.")


== #text("External complete-GB control")

#text("Official msolve 0.11.0 commit cc044551f8e202d00691990fd3fd0b1226a88ea2, one thread, seed 0, grevlex, flags -t 1 -v 2 -g 2 -r 1, is compared on exactly 378 frozen positions (21 records × three arms × three fixed peel positions × two signs). Five positions have nonunit ideals; 373 do not. Inputs were selected before solving. Warmup, first, A/A and five timed passes yield 3,024 matching native/msolve reduced bases, each independently checked by input and S-pair reduction. The timer patch only increases printed precision. Build, options, library/executable hashes and raw outputs are preserved.")


#image("figures/external-gb.svg", width: 100%)

#text("The comparison uses internal complete-basis timers on the same isolated c7i.xlarge host, excluding startup/parsing/printing. Each group contains the same 18 fixed systems in each round. Native/msolve above 1 means native is slower. This does not price an msolve-based complete DLP solver. Magma, FGb, Singular and OpenF4 were not reproduced; fastest-external status is unknown. The observed msolve gap remains a backlog item (../../../backlog/XP-20261023-quartic-twist-image-base/README.md).")


#text(size: 6.8pt)[#table(columns: 7, inset: 4pt, text("p / arm"), text("Native internal ms"), text("msolve internal ms"), text("Native/msolve median"), text("Five-round range"), text("Native A/A"), text("msolve A/A"), text("37 / original"), text("15.003302"), text("14.596226"), text("1.029396"), text("0.996377–1.056698"), text("1.010498"), text("0.979161"), text("37 / phi-l0-active"), text("15.245855"), text("14.526784"), text("1.039499"), text("1.035443–1.049500"), text("0.989531"), text("1.004391"), text("37 / double-l0-active"), text("14.997233"), text("14.598490"), text("1.030647"), text("1.006926–1.033431"), text("0.999100"), text("0.988070"), text("73 / original"), text("15.217642"), text("14.487801"), text("1.055972"), text("1.032729–1.075527"), text("1.005649"), text("0.989025"), text("73 / phi-l0-active"), text("15.170199"), text("14.497698"), text("1.052228"), text("1.032141–1.057748"), text("0.991448"), text("1.007731"), text("73 / double-l0-active"), text("15.287369"), text("14.605640"), text("1.056638"), text("1.037290–1.089624"), text("1.008156"), text("0.989346"), text("97 / original"), text("15.047991"), text("14.413478"), text("1.045285"), text("1.028933–1.074409"), text("0.984633"), text("0.980374"), text("97 / phi-l0-active"), text("15.365781"), text("14.550267"), text("1.062530"), text("1.032282–1.076409"), text("0.989421"), text("0.969586"), text("97 / double-l0-active"), text("15.223387"), text("14.347432"), text("1.061697"), text("1.035022–1.066132"), text("0.975632"), text("0.971008"), text("157 / original"), text("15.397865"), text("14.527320"), text("1.051682"), text("1.047673–1.083013"), text("0.989845"), text("1.016876"), text("157 / phi-l0-active"), text("15.235802"), text("14.419197"), text("1.040759"), text("1.024412–1.063383"), text("0.977498"), text("0.989569"), text("157 / double-l0-active"), text("15.342985"), text("14.627573"), text("1.039590"), text("1.023495–1.083348"), text("0.995048"), text("1.040114"), text("229 / original"), text("15.376205"), text("14.471710"), text("1.061580"), text("1.005761–1.092633"), text("0.976644"), text("0.999781"), text("229 / phi-l0-active"), text("15.322049"), text("14.447986"), text("1.066544"), text("1.054762–1.097192"), text("0.999181"), text("0.985233"), text("229 / double-l0-active"), text("15.387014"), text("14.399529"), text("1.072364"), text("1.051473–1.088881"), text("0.992812"), text("1.001293"), text("241 / original"), text("15.218322"), text("14.542282"), text("1.046807"), text("1.037788–1.072078"), text("1.008208"), text("1.037221"), text("241 / phi-l0-active"), text("15.256400"), text("14.405847"), text("1.046861"), text("1.040671–1.074604"), text("1.007443"), text("1.000143"), text("241 / double-l0-active"), text("15.283102"), text("14.696180"), text("1.039937"), text("1.018126–1.062228"), text("0.998248"), text("1.000590"), text("337 / original"), text("15.292063"), text("14.568626"), text("1.055506"), text("1.026989–1.082315"), text("0.968781"), text("0.999726"), text("337 / phi-l0-active"), text("15.273493"), text("14.505745"), text("1.061121"), text("1.041556–1.079674"), text("0.957374"), text("1.004517"), text("337 / double-l0-active"), text("15.276665"), text("14.505743"), text("1.048874"), text("1.044945–1.074879"), text("0.985464"), text("0.993356"))]

== #text("Interpretation and outstanding questions")

#text("For a uniform residual, a fixed peel is a translation and leaves the residual distribution unchanged. Equal-size disjoint phi and doubling images therefore have the same exact first moment of signed physical tuples; their correlations and rank trajectories may differ. The historical stream is not uniform, so the measured tuple difference is conditional on this tape. Every seeded record also uses the same public scalar 271. Three seeds are trajectory variations, not independent secret samples, and timing repeats are repeated measurements of the same problem. No confidence interval for a population of curves or unknown scalars is inferred.")


#text("The actual distinguishable mathematical feature is phi's low-degree image equation. A joint S5/elimination solver could retain it and share work across image peels. That solver is not implemented or measured here. Orbit representatives alone exchange a peel loop for a transformed-residual loop; they do not remove those systems. No asymptotic obstruction to all joint algorithms is claimed.")


#text("The frozen cold supplement's assertion that no precomputed lift is available needs a precise correction: encoded public points are present for sign alignment and checks, while every required lift is recomputed and charged. The candidate does not read the tape's known scalar or reference hits/ranks, but this is source-audited rather than a process-isolated blinded input. The boundary document (BOUNDARY.md) preserves that distinction and names every excluded cost.")


#text("The field/group and F4 row-reduction counters are partial. Complete instrumented elapsed time is available; a complete GAE price, the 0/0.1/1 price band and a protocol-matched strongest rho reference are missing. No VALIDATION.json is fabricated and no new headline enters the leaderboard or benchmarks/README.md. Benchmark PDFs and figures are copied as unlisted diagnostics. The precise remaining problem is quartic-twist-polynomial-ic-pricing (../../../problems/quartic-twist-polynomial-ic-pricing.md).")


== #text("Reproduction, controls and publication")

#text("ARTIFACTS.md (ARTIFACTS.md) gives the content-addressed raw archive, frozen source hashes, extraction and exact native analysis commands. All launched-job reports (reports/INDEX.md) preserve build failures, timeouts, interruptions and invalid resource receipts. MATHEMATICS.md (MATHEMATICS.md) separates proved coordinate algebra, checked group actions, elementary counting arguments and untested joint-solver hypotheses. VALIDATION_STATUS.md (VALIDATION_STATUS.md) records the actual software gates, including failed and interrupted attempts. The archive also retains the rejected coefficient/record mutations, corrupted input hash control, original baseline parity and whole-round resume control.")


#text("Canonical graph review: docs/ic/leaderboard.json, docs/ic-leaderboard.html, docs/ic/progress-timeline.json, docs/index-calculus-scoreboard.html and the curve registry were checked for the new image study. No new curve transport or validated operation-price/rho result exists, so their existing values remain current. The new measured coverage is represented in the study-local figures and benchmark copies only. This research deliverable is part of PR #2368 (https://github.com/aburan28/crypto/pull/2368); software review status is distinct from mathematical validation.")
