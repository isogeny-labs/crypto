#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: true)
#show raw: set text(size: 7.6pt)
= #text("Joint construction v1")

#text("Run algebra-v1, started 2026-10-24 00:03:57 UTC. Protocol: joint S5 construction (../../PROTOCOL.md). Frozen constructor: 054b22b41d8e8c0d4296c9119b06e88df5d95fab.")


#image("../../figures/construction-profile.png", width: 100%)

#text("The native validator passes all 297 requested tasks, 694 system files and 63 case-arm sets. Each arm has 84 fixed-query tasks plus separately labelled positive and regression controls. Phi has 97 tasks and 194 files; doubling has 100 and 200; partial-symmetry pure L0 has 100 and 300. The pure count includes the additional cubic-only geometric negative control.")


#text("Recorded extension term ranges are 3,960; 5,444–5,445; and 1,484–1,485. Recorded total degrees are 32,40,16 and U degrees are 24,32,8, respectively. The whole-stream incidence audit passes 4,110 unique signed relations in 5,433 presentations. It verifies actual group sums and resultant vanishing. Sixteen independent Gaussian determinant probes check each finite system.")


#text("Evidence: native validation (../../results/algebra-v1-validation.json), runs/algebra-v1/metadata/, runs/algebra-v1/exports/, runs/algebra-v1/audit/. Exact public point encodings are supplied; lifts are not reconstructed as a cold DLP setup in this run. The 21 cases use public scalar 271 and conditioned historical query streams.")


#text("Execution is a bounded local construction diagnostic under the busy reservation, one thread, 8 GiB and a 1,200 s watchdog per phase. No wall timing, root-completeness, end-to-end cost or performance admission follows from construction success. R=O is implemented but not exercised by these 297 tasks. The native validator binds commands, source, workload, public hashes, coefficients, dimensions, statistics and exported msolve inputs.")
