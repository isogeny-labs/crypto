#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: true)
#show raw: set text(size: 7.6pt)
= #text("Current-main integration gate passes all ten phases")

#text("The ordinary clean merge of fetched main at 092d1ff47e changes seven runtime files. The earlier 4,488-test gate is retained; the new gate checks the new root inputs and is not inferred from that earlier result.")


#image("../../figures/latest-main-software-gates.png", width: 100%)

== #text("Observed results")

#text(size: 6.8pt)[#table(columns: 2, inset: 4pt, text("Phase"), text("Actual outcome"), text("Release library compilation and tests"), text("4,492 passed, zero failed, 119 existing ignores"), text("Touched examples"), text("One integer-certificate test and three structure tests pass"), text("Curve identity self-test and naming"), text("Both exit zero"), text("Validation-gate tests and base comparison"), text("Both exit zero"), text("Official large-file check"), text("Exit zero"))]

#text("All ten declared phases, including separate compilation, have exit zero. The library log reports 978.66 seconds under the unchanged 1,200-second watchdog. This is an unadmitted test duration, not a solver comparison. Tests are serial under 8 GiB after separate compilation without the address-space cap. Heavy work uses the existing shared busy reservation.")


== #text("Retained launch failure and exact scope")

#text("The first launcher, post-latest-main-v2, exits 127 before compilation because managed systemd PATH cannot find rustc. Its executed script, partial startup metadata and journal are retained. Corrected v3 explicitly selects the existing /home/ec2-user/.cargo/bin toolchain; no resource, test or evidence gate is weakened. No scientific workload runs in v2.")


#text("The v3 source label is 1055dcbcdb4998e78e22c900ab737ddc9913de70, with actual root source hashes, empty touched-source diff, commands, host, Rust version, outputs and exits in results/publication/post-latest-main-v3/metadata/. The native point-replay suite's earlier 81-test pass has its own source and frozen runner; it is not a newly repeated test under this root label.")


#text("The gate starts at 10:25:18 UTC and finishes at 11:11:56 UTC. Later metadata and report commits do not change these root runtime inputs. The 43-directory scientific checkpoint remains fixed. This software pass does not establish new mathematical completeness, a scientific speedup, published-head GitHub CI or PR merge. Applicable review and CI still concern the published head.")
