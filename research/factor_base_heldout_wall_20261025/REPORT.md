# N12 frozen held-out paired wall comparison

**Observed status:** No valid isolated 12-process schedule; 0/5,040 timed target-arm rows. The A/A ratio is null. The five candidate/rho round ratios are null. Their median is null and minimum is null. These are wall-process ratios only. The prior independent-host operation-count ratio (2.4799052224835276 at its representative price) is a separate result and is not pooled here.

The fixed 420-target list and `fb1-any-j32` selector are identified in [PROTOCOL.md](PROTOCOL.md). The protocol requires two rho A/A processes, five alternating candidate/rho pairs, source-scalar verification in each target row, matching counters against the frozen source, reserved CPUs 0 and 16, an uncontended isolation receipt, and I/O pressure at most 5% at both boundaries. [wall-ratios.svg](wall-ratios.svg) shows all valid round ratios or the null status. [DATA_DICTIONARY.md](DATA_DICTIONARY.md) defines each field and null.

The source and frozen selector were not retuned. The result is limited to this host and cohort. No wall ratio is inferred from incomplete or contended observations. At the 09:03 UTC start gate the benchmark lock remained occupied, so the wall ratio is null. [OBSTRUCTION.md](OBSTRUCTION.md) and [the problem record](../../problems/n12-heldout-wall-isolation.md) give the exact observed condition and exit criteria.
