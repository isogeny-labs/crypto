#set page(paper: "a4", margin: 15mm, numbering: "1")
#set text(font: "DejaVu Sans", size: 9pt)
#set par(justify: false)
#heading(level: 1)[Fresh explicit-one-worker guard confirmation]
#text("Run 20261019-040338-boolean-native-serial-gu-6ec5e9. Fresh same-binary confirmation after the initial four-worker regression. Five baseline A/A and five serial0/1 pairs per cell; each one-worker round additionally measures the legacy kernel with all three switches off. Exact input, raw/canonical output, rank, XOR and profile counts match. No host or earlier sample is pooled.")


#text("Host: Intel(R) Xeon(R) Platinum 8488C; rustc 1.99.1 (58cf8f9c3 2026-10-13); c7i.2xlarge on-demand; reserved cores/SMT siblings: one=3,7
four=3,7,2,6. The refined guard caches the explicit RAYON_NUM_THREADS=1 configuration and checks the actual current pool before selecting serial work. With an explicit four-worker configuration, both switch states skip the registry query.")


#text("All four multiworker controls are neutral under the frozen gate.")


#text("All eight one-worker incremental cells qualify faster. Primary n23 ratios are1.0802 F4 and1.0730 F5. Direct same-host legacy to candidate ratios are1.1141 and1.1209, with CIs[1.0980,1.1297] and[1.0608,1.1402]. All12cells have five eligible pairs; eleven rejected attempts remain archived. These ratios do not reprice the earlier complete-solver table.")


#text("Incremental pairs fix single-write/vector switches1 in both arms. Direct legacy ratios compare that round's all-off kernel with the candidate in the same binary, rather than multiplying previous gains. Admission, first eligible of three attempts, exact3125-resample CI and max(2%, twice A/A noise) are unchanged. Every slower, neutral or missing outcome remains in serial-guard-summary.json.")


#text("The switch remains opt-in and requires explicit one-worker configuration; nested multiworker pools keep parallel iteration. Four worker/switch configurations pass the existing GF2 reference tests before native timing. These are matrix calls, not complete recursive roots or decomposition. The eighteen-arm cost table retains its earlier source and serial-clear-off flags. Source: SERIAL_GUARD_PROTOCOL.md, run-serial-guard.sh, analyze-serial-guard.rs and runs/serial-guard/ in the raw archive. The prior slower control is preserved in its own report.")


#image("../../native-serial-clearing.svg", width: 100%)
#pagebreak(weak: true)
#image("../../native-serial-negative.svg", width: 100%)
#pagebreak(weak: true)
#image("../../native-legacy-to-candidate.svg", width: 100%)
#pagebreak(weak: true)
#heading(level: 2)[Incremental pairs and four-worker controls]
#table(columns: 8, inset: 4pt, text("Arm"), text("n"), text("Seed"), text("Workers"), text("Before ms"), text("After ms"), text("Ratio"), text("Decision"), text("matrix-f4"), text("17"), text("230"), text("1"), text("6.1083"), text("5.8542"), text("1.0434"), text("faster"), text("matrix-f4"), text("23"), text("230"), text("1"), text("53.4171"), text("49.6467"), text("1.0802"), text("faster"), text("matrix-f4"), text("23"), text("231"), text("1"), text("53.4533"), text("49.8884"), text("1.0695"), text("faster"), text("matrix-f4"), text("23"), text("232"), text("1"), text("53.7325"), text("49.1784"), text("1.0913"), text("faster"), text("matrix-f5"), text("17"), text("230"), text("1"), text("6.5614"), text("6.3594"), text("1.0276"), text("faster"), text("matrix-f5"), text("23"), text("230"), text("1"), text("56.6003"), text("52.8025"), text("1.0730"), text("faster"), text("matrix-f5"), text("23"), text("231"), text("1"), text("56.1721"), text("52.0773"), text("1.0721"), text("faster"), text("matrix-f5"), text("23"), text("232"), text("1"), text("56.9634"), text("52.8105"), text("1.0853"), text("faster"), text("matrix-f4"), text("17"), text("230"), text("4"), text("8.1520"), text("8.1383"), text("1.0093"), text("neutral"), text("matrix-f4"), text("23"), text("230"), text("4"), text("58.7065"), text("58.0531"), text("1.0180"), text("neutral"), text("matrix-f5"), text("17"), text("230"), text("4"), text("8.3863"), text("8.4727"), text("0.9863"), text("neutral"), text("matrix-f5"), text("23"), text("230"), text("4"), text("57.1523"), text("58.0442"), text("0.9974"), text("neutral"))
#pagebreak(weak: true)
#heading(level: 2)[Direct legacy to candidate; one worker]
#table(columns: 7, inset: 4pt, text("Arm"), text("n"), text("Seed"), text("Legacy ms"), text("Candidate ms"), text("Ratio"), text("Decision"), text("matrix-f4"), text("17"), text("230"), text("6.3206"), text("5.8542"), text("1.0813"), text("faster"), text("matrix-f4"), text("23"), text("230"), text("54.9114"), text("49.6467"), text("1.1141"), text("faster"), text("matrix-f4"), text("23"), text("231"), text("54.8660"), text("49.8884"), text("1.1005"), text("faster"), text("matrix-f4"), text("23"), text("232"), text("55.7328"), text("49.1784"), text("1.1320"), text("faster"), text("matrix-f5"), text("17"), text("230"), text("6.8030"), text("6.3594"), text("1.0691"), text("faster"), text("matrix-f5"), text("23"), text("230"), text("59.2979"), text("52.8025"), text("1.1209"), text("faster"), text("matrix-f5"), text("23"), text("231"), text("57.9081"), text("52.0773"), text("1.1170"), text("faster"), text("matrix-f5"), text("23"), text("232"), text("58.6563"), text("52.8105"), text("1.1057"), text("faster"))