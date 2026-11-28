#set page(paper: "a4", margin: 14mm)
#set text(font: "New Computer Modern", size: 10pt)
= N83 dimension-18 direct-subgroup orbit size

Under the frozen native census, 262,144 low-bit x words produced 130,724
curve-liftable x values, 32,710 direct-subgroup x values and 32,643 distinct
full signed-Frobenius orbit classes. The registered K = 16,627 size premise
passes by 16,016 classes. All retained x-orbits have length 83. The first
16,627 classes imply 2,760,082 points after 83 phases and two signs; the
object and its index have not been materialized.

#image("reports/run-20261019T0141Z/counts.svg", width: 100%)

The direct native run exited 0. It took 18,865.461 ms process wall on a
shared arm64 Mac while another release suite was running; this L0 duration
is descriptive. A prior `/usr/bin/time -l` wrapper exited 1 after the child
wrote the same complete CSVs because the sandbox denied a macOS `sysctl`.
The two CSV pairs are byte-identical, and 17 selected representatives passed
general curve/subgroup/Frobenius cross-checks. Both runs use the same
enumerator, so independent full replay remains to be done.

The next step is a content-addressed native export and independent replay
of the selected closure. Compact membership encoding, natural m = 4 relation
and rank, m = 2/3 policies, prebuilt-versus-on-demand cost and matched rho
remain open. `RESULT.md`, `PROTOCOL.md` and `raw-files.sha256` provide the
exact evidence and limits.
