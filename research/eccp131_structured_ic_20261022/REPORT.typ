#set page(paper: "us-letter", margin: (x: 0.72in, y: 0.62in))
#set text(font: "New Computer Modern", size: 10pt)
#set heading(numbering: none)
= ECCp-131 structured-coset relation screen
_Exact-curve bounded observation · 22 October 2026_

The published ECCp-131 curve and target were checked at full coordinates.
The field has $p-1=2 dot 5 dot r$ with prime
$r=155003179783434785924857641481313994241$, so a Kummer
$2^t$-root tower over this field permits only $t <= 1$.
We screened five-root and ten-root multiplicative cosets as separately
frozen structured-base controls.

#image("workflow.svg", width: 100%)

== Frozen workload

For each representative $g=1, dots, 16$, lift $g mu_5$ or $g mu_(10)$ to the curve.
Twelve and sixteen cosets respectively had at least two points. Each was paired with an equal-column
small-abscissa base. A complete signed pair table plus subtraction of one
signed point tested three-point relations. Seeds 131 and 132 each tested
256 setup and 256 published-target candidates per arm, with eight planted
witnesses per arm.

== Observations and replay

#table(columns: (0.9fr, 1.6fr, 1.3fr, 1.3fr, 1.0fr),
  [Base / seed], [Completed arms], [Planted controls], [Natural rows / rank], [Cold wall],
  [μ5 / 131], [12 coset + 12 control], [192 / 192], [0 / 0], [12.82 s],
  [μ5 / 132], [12 coset + 12 control], [192 / 192], [0 / 0], [19.03 s],
  [μ10 / 131], [16 coset + 16 control], [256 / 256], [0 / 0], [38.44 s],
  [μ10 / 132], [16 coset + 16 control], [256 / 256], [0 / 0], [43.05 s],
)

All 896 planted point identities, both public points, subgroup order, and
the factorization were independently checked in PARI/GP. At matched base
size, the coset and small-abscissa arms had the same distinct signed pair
coverage $2 B^2$ for $B=2,dots,8$. Peak RSS was below 25 MB in each process.
The shared host and four cold runs support no speed ranking.

== Scope and next gate

A base with at most eight abscissae has no more than $8 B^3$ signed triples;
its uniform-candidate support on this group is below $2^(-118)$.
The zero-row observation concerns these cosets and budgets. The exact
published-target scalar remains undetermined by this screen.
The next candidate is an isogeny-form PKM tower with a certified full-width
auxiliary curve and map, followed by one independently replayed natural
relation before a rank and target-descent trial. Full cold cost is required.

_Raw JSON, commands, timings, replay source, source hashes, and the frozen
protocol: research/eccp131_structured_ic_20261022/._
