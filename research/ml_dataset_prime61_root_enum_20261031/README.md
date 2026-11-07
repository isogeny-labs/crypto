# Prime p=2^61-1 bounded root and enumeration pilot

This is a four-cell administrative and native-evidence pilot on the requested
high-size prime modulus. The frozen plan crosses `root` and `enumerate` with
both twists at `n=61,m=2`, one curve seed, one generator and one key per twist.
The methods share the public input within each twist. The fixed population is
four cells; every timeout, construction failure, partial result, unsupported
outcome, and successful result remains in the denominator. A constructed curve
is labelled as the requested modulus only after the native public artifact
records `2305843009213693951` and exact verification passes.

`protocol.json` is the frozen `experiment.protocol/v1`; `plan.json` binds
its exact file hash. Both were committed before any scientific attempt.
`make_protocol_draft.py` records the pre-data construction of the draft. The
protocol has no timing comparison or method advantage claim. Native work is
bounded to 30 seconds per case, 90 seconds per outer generate command, 2 GiB
memory and 64 MiB trace per shard; S3 custody and exact replay are separate
promotion gates. An independently recovered scalar label requires native
group replay, and a bounded root or enumeration trace does not establish a
negative PDP or exact fall degree.

The selected local backend uses a predeclared XFS output root because the
cloud worker IAM and cumulative-budget admission for this campaign are not
established. This four-cell pilot does not replace the full requested matrix.

The completed four-cell results, independent archive audit, export checker,
and S3 readback sample are linked in [report/run.md](report/run.md). The
quantitative outcome figure is [report/outcomes.svg](report/outcomes.svg),
with a PDF rendering in [report/run.pdf](report/run.pdf).
