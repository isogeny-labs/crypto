#set page(paper: "a4", margin: 15mm, numbering: "1")
#set text(font: "DejaVu Sans", size: 9pt)
#set par(justify: false)
#heading(level: 1)[Initial serial clearing and failed multiworker control]
#text("Run 20261019-030905-boolean-native-serial-cl-25ad47. Same-binary KIC_GF2_SERIAL_CLEAR=0/1; single-write/vector tables stay enabled in both arms. Five baseline A/A and five A/B pairs for each F4/F5 matrix at n17 seed230 and n23 seeds230/231/232. Four-worker seed230 controls keep the original parallel path. Exact raw/canonical rows, rank and counted XOR/profile work agree.")


#text("Host: Intel(R) Xeon(R) Platinum 8488C; rustc 1.99.1 (58cf8f9c3 2026-10-13); c7i.2xlarge on-demand, reserved physical cores/SMT siblings: one=3,7
four=3,7,2,6. The first condition queries the current Rayon pool. The parallel iterator remains selected with four workers; that does not establish unchanged cold runtime.")


#text("All eight one-worker cells qualify faster. Primary n23 paired ratios are1.0770 F4 and1.0652 F5. The n17 F4 four-worker control qualifies slower at0.8436, CI[0.8284,0.8504]; the other three multiworker cells are neutral. This result blocks a regression-free claim for the unrestricted guard. A subsequent protocol narrows activation to explicitly configured one-worker processes before querying the registry; startup/scheduling is a hypothesis, not a proved diagnosis.")


#text("Admission, first eligible of three attempts, five-pair exact bootstrap and max(2%, twice median absolute A/A noise) are unchanged. Rejected attempts and every slower/neutral/inconclusive result remain in serial-clear-summary.json. Negative-control movements are not interpreted as algorithm gains.")


#text("This is an incremental matrix-call experiment. It does not measure complete recursive roots, decomposition, or a whole IC/rho pipeline. The eighteen-arm solver table retains its prior source and serial-clear-off flags; no complete-solver benefit is inferred from this kernel result. Source: SERIAL_CLEAR_PROTOCOL.md, run-serial-clear.sh, analyze-serial-clear.rs and runs/serial-clear/ in the raw archive. Both kernel-test switch states pass on the native host before timing.")


#image("../../initial-serial-clearing.svg", width: 100%)
#pagebreak(weak: true)
#image("../../initial-serial-controls.svg", width: 100%)
#pagebreak(weak: true)
#heading(level: 2)[All paired results, including the slower control]
#table(columns: 8, inset: 4pt, text("Arm"), text("n"), text("Seed"), text("Workers"), text("Before ms"), text("After ms"), text("Ratio"), text("Decision"), text("matrix-f4"), text("17"), text("230"), text("1"), text("7.4833"), text("7.1817"), text("1.0370"), text("faster"), text("matrix-f4"), text("23"), text("230"), text("1"), text("61.3743"), text("57.7376"), text("1.0770"), text("faster"), text("matrix-f4"), text("23"), text("231"), text("1"), text("62.0068"), text("57.9712"), text("1.0693"), text("faster"), text("matrix-f4"), text("23"), text("232"), text("1"), text("61.4660"), text("57.5380"), text("1.0652"), text("faster"), text("matrix-f5"), text("17"), text("230"), text("1"), text("8.0175"), text("7.7868"), text("1.0280"), text("faster"), text("matrix-f5"), text("23"), text("230"), text("1"), text("65.6287"), text("61.2560"), text("1.0652"), text("faster"), text("matrix-f5"), text("23"), text("231"), text("1"), text("65.3784"), text("61.3875"), text("1.0729"), text("faster"), text("matrix-f5"), text("23"), text("232"), text("1"), text("65.1584"), text("61.3274"), text("1.0620"), text("faster"), text("matrix-f4"), text("17"), text("230"), text("4"), text("8.3357"), text("9.8786"), text("0.8436"), text("slower"), text("matrix-f4"), text("23"), text("230"), text("4"), text("65.1697"), text("64.7849"), text("1.0120"), text("neutral"), text("matrix-f5"), text("17"), text("230"), text("4"), text("9.2373"), text("8.4551"), text("1.0451"), text("neutral"), text("matrix-f5"), text("23"), text("230"), text("4"), text("64.3608"), text("63.6913"), text("1.0081"), text("neutral"))