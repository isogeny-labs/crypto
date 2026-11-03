# N83 K1107 versus K2304 native bridge

The [32-point matched bridge](https://github.com/aburan28/crypto/actions/runs/39096257170)
completed on four distinct, overlapping GitHub Actions runners from source
commit `77c8ebe307c047c78a7f66f6a7f41a0efef60584`. All 64 one-target IC
solves and their same-point rho arms passed native relation, factor-base,
scalar, resource and calibration replay. The runs used the
[frozen protocol](protocol.json), SHA-256
`afafcd2a389f25ee7116326d3943bb1b9a05f4eae0ebcccce0d712a3aab96ce1`.
The four downloaded ZIPs were each bound to GitHub's artifact digest; local
native replay regenerated each
archived `analysis.json` **byte for byte**. The [provider audit](results/39096257170/provider-audit.json)
passed with four exact jobs and four exact artifacts. The raw ZIPs also have
write-once, fresh-download-verified [S3 custody](results/39096257170/provider-inventory.json)
so replay does not depend on GitHub's 14-day artifact retention.

The frozen [native selector](results/39096257170/selection.json) and
[independent Python statistical audit](results/39096257170/selection-audit.json)
agree on every point, paired sign count, p-value, mean and descriptive 95%
interval. A [changed S3 GET length](results/39096257170/provider-mutation-control.json)
and a [changed selected ID](results/39096257170/selection-mutation-control.json)
were each rejected. Times are milliseconds below; a negative online log difference
favors K2304, while a positive cold difference favors K1107.

| One-target measure | K1107 mean | K2304 mean | Paired direction | Exact two-sided sign p | Frozen choice among these two |
| --- | ---: | ---: | --- | ---: | --- |
| Online target interval | 77.997 | 16.615 | K2304 faster on 26/32; geometric K2304/K1107 = 0.2246, descriptive 95% log interval [−2.2322, −0.7550] | 0.0005351 | K2304 |
| Fresh-process internal cold | 112,684.923 | 245,288.293 | K1107 faster on 32/32; K2304 − K1107 = 132,603.370 ms [130,859.772, 134,346.967] | 4.66×10⁻¹⁰ | K1107 |
| Full IC child-process wall | 112,864.181 | 245,698.153 | K1107 faster on 32/32; K2304 − K1107 = 132,833.971 ms [131,069.821, 134,598.121] | 4.66×10⁻¹⁰ | K1107 |

Both IC configurations beat their same-point rho **online** interval on all
32 public points. For the full child-process wall, IC was slower than rho on
all 32 points for each candidate: the independent audit reports median
IC/rho wall ratios of 40.66 for K1107 and 103.16 for K2304. The two
candidates had full rank in every admitted run. Calibration relative spread
was below 1% on every row. The protocol's conservative charge is the full
four-job cap, **79,200 seconds**, with a 64,800-second producer-cap subtotal;
these research-allocation caps are reported separately from the measured
one-target times.

The [K1107](candidate_IC1N83Ckb1a1fb183762PDP4rootRCguidedLAgaussTDdirectISO0hc54c7b61d40a.json)
and [K2304](candidate_IC1N83Ckb1a1fb382464PDP4rootRCguidedLAgaussTDdirectISO0h7513107d16d1.json)
candidate manifests identify exact point-defined
bases for the degree-83 **a=1 diagnostic curve with a 53-bit subgroup**.
Their S3 objects under the requested prefix passed the independent
[factor-base custody audit](../n83_bridge_s3_custody_20261018/RESULT.md);
all 64 point-specific provider copies match those stored bytes. The primary
Koblitz `a=0` N83 sweep has an 81-bit subgroup and is a different workload.
This bridge selects K1107 for cold time **among these two a=1 candidates**;
it does not establish the globally fastest factor base or a primary-curve
total index-calculus runtime. The primary `selected_best_total_runtime`
remains `null`.

Validation on this branch includes the [serial release library suite](results/39096257170/validation-lib.json):
4,162 passed, 117 ignored and zero failed. Its full deterministic
[compressed log](results/39096257170/validation-lib.log.gz) is retained with
SHA-256 in the receipt. The relevant Python discovery suite passed 63 tests;
the native bridge provider run passed the touched example tests at the
protocol-pinned source, and no Rust source changed afterward.
