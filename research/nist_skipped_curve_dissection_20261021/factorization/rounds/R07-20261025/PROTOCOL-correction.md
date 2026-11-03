# Command spelling correction before actual filtering

The initial protocol wrote `-min_rels 1`; this native CLI names the option
`-minrels 1`. The first invocation printed its unknown-option help and
returned zero before any filtering ran. Its stdout, command, input hashes,
unmodified relation-file hash and initial receipt remain retained. An exit
code of zero alone therefore does not establish that this experiment ran.

Use the corrected spelling in a new output directory under the same native
300-second/outer 360-second bounds. The dispatch-only purpose and all
arithmetic/filter acceptance conditions stay unchanged. The failed parser
attempt consumed no native filtering allocation. Require a native filtering
start log before classifying a later zero exit as completed filtering.

The redundant relation-file copy of the parse-only attempt may be replaced
with a symlink to its unchanged, hash-bound R06 source. The full source data
remain in R06's retained archive; the failed attempt keeps that explicit
input dependency and its original command/output. The actual filtering
attempt uses its own independent copy.
