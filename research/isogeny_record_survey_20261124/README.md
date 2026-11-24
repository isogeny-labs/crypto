# Large prime-degree isogenies: primary-source comparison

Survey date: **2026-11-24**. This is a bounded literature comparison for the
per-standard-curve prime-degree campaign. It establishes neither a worldwide
record nor a new accepted map. Six full-text PDFs were inspected and hashed;
one additional source was accessible only through its official abstract and
revision metadata. None of these external computations was reproduced here.

A largest-degree claim needs an exact curve, a representation category, an
individual degree and its primality evidence, and a checkable computation.
Field size, protocol parameter size, the level of a modular polynomial, and the
product of degrees in a walk answer different questions. This survey does not
establish a finite ceiling for future constructions.

## What the primary sources establish

| Observation | Degree or parameter actually stated | Produced object / evidence category | Source location |
| --- | --- | --- | --- |
| Jao–Soukharev ordinary example | 5,000,000,029 | Codomain and point image, with a conjugate-ideal consistency computation | [§5.1, PDF pp. 11–12](https://arxiv.org/pdf/1002.4228v2#page=11) |
| Jao–Soukharev ECCp-109 | Least Elkies prime above p/2; decimal degree not printed | Codomain and challenge base-point image; conjugate-ideal check described | [§5.2, PDF pp. 12–13](https://arxiv.org/pdf/1002.4228v2#page=12) |
| Jao–Soukharev ECCp-239 | Least Elkies prime above p/2 | Ideal-class relation; the authors explicitly leave evaluation unfinished | [§5.3, PDF p. 13](https://arxiv.org/pdf/1002.4228v2#page=13) |
| Sutherland | Level 100,019 | Instantiated Weber modular polynomial over a finite field | [§5, PDF p. 18](https://arxiv.org/pdf/1202.3985v5#page=18) |
| Bröker–Lauter–Sutherland | Level 60,013 | Bivariate Weber modular polynomial over the integers | [§8, PDF p. 27](https://arxiv.org/pdf/1001.0402v2#page=27) |
| Bernstein–De Feo–Leroux–Smith | 6,548,911 appears in the selected field's p+1 factorization | Parameter supporting supplied-kernel evaluation methods | [Appendix A.1, PDF p. 14](https://arxiv.org/pdf/2003.10118v1#page=14) |
| Bostan–Morain–Salvy–Schost | Several benchmark degrees, rather than a certified maximum | Reconstruction from supplied endpoint/degree information | [Algorithm and numerical comparisons, PDF pp. 16–19](https://cs.uwaterloo.ca/~eschost/publications/BoMoSaSc06.pdf#page=16) |
| PRISM / salt-PRISM | Table 2: a=248/376/500 and a=192/256/320 respectively | Compact representations on chosen supersingular curves; these parameters do not identify a largest individual prime | [Table 2, PDF p. 40](https://r98inver.github.io/papers/2607_saltprism.pdf#page=40) |
| ECLIPSE | Exact individual degree not verified in this survey | Author-reported implementation of a prime-degree PRISM variant; abstract-only lead | [Official ePrint 2026/2312](https://eprint.iacr.org/2026/2312) |

The ECCp-239 relation remains unevaluated. These historical evaluation examples
do not supply a locally replayed expanded-map certificate.
[Jao–Soukharev, §§5.1–5.3](https://arxiv.org/pdf/1002.4228v2#page=11)

An instantiated modular polynomial describes potential neighboring curve
invariants. Its level is not evidence that an individual map, kernel, and
rational-function identity were exported and replayed on the campaign's chosen
curve. The 100,019 and 60,013 results stay in their polynomial categories.
[Sutherland](https://arxiv.org/pdf/1202.3985v5#page=18),
[Bröker–Lauter–Sutherland](https://arxiv.org/pdf/1001.0402v2#page=27)

The square-root Vélu algorithm evaluates from an already supplied kernel point.
Its stated goal is the codomain and point images rather than expanded
coefficients. A factor appearing in its parameter appendix is not, by itself,
an archived individual-map record.
[Bernstein–De Feo–Leroux–Smith, pp. 1–2 and Appendix A.1](https://arxiv.org/pdf/2003.10118v1)

BMSS addresses reconstruction in suitable characteristic. It is relevant to the
construction pipeline, but plot axes and supplied-endpoint benchmarks do not
identify a largest certified prime-degree search result.
[Bostan–Morain–Salvy–Schost](https://cs.uwaterloo.ca/~eschost/publications/BoMoSaSc06.pdf)

The main PRISM representation includes an auxiliary isogeny: its response has
degree q(2^a−q). The paper also describes a distinct prime-degree variant.
Table 5 benchmarks salt-PRISM, whereas Table 2 also contains PRISM-sig
parameters. A field parameter, prime-component parameter, and full response
degree must remain distinct; no maximum individual q is inferred here.
[PRISM, §3, §5, Tables 2 and 5](https://r98inver.github.io/papers/2607_saltprism.pdf)

The ECLIPSE landing page reports receipt on October 2 and a latest revision on
November 13, 2026. Its abstract reports a prime-degree implementation and
interoperating implementations. PDF retrieval returned HTTP 403 in this
session. Exact parameters, artifacts, and replay therefore remain unresolved;
the inaccessible PDF is an access gap.
[ECLIPSE official abstract and metadata](https://eprint.iacr.org/2026/2312)

## Comparison rules for this campaign

The principal scoreboard ranks **each registered standard prime-field curve
separately**. It accepts an expanded map only after construction evidence and
independent replay both exist. Aliases of one curve identity do not create
additional curves. Different field representations or curve models require
explicit identity and conversion evidence before results can be transferred.

Keep separate tables for:

1. Expanded maps on each exact registered standard curve.
2. Codomain and point evaluations, including supplied-kernel methods.
3. Compact representations on chosen curves, with dimension and any
   auxiliary/composite degree recorded.
4. Modular-polynomial computations and ideal-class relations.
5. Known-endomorphism constructions and composite chains, with their individual
   and total degrees stated separately.

A comparable accepted row must contain:

- **Identity:** source and target identifiers, field characteristic and
  representation, model coefficients, subgroup/cofactor data where relevant,
  and generator/point identities used by checks.
- **Degree:** the exact integer, primality proof or explicitly qualified
  primality status, separability, kernel order, and field of definition.
- **Representation:** expanded coefficients or the precise compact encoding;
  distinguish the prime component from any auxiliary or composed degree.
- **Custody:** immutable source revision, executable and input hashes,
  deterministic settings or random seeds, command line, and output hashes.
- **Verification:** an independent implementation's checks binding the same
  kernel, source, codomain, map identity and evaluated points; finite point
  agreement alone does not replace the map identity.
- **Cost:** setup, search, construction, certificate production and independent
  replay costs; elapsed/user/system time, resource caps, host/CPU identity,
  and every failed, capped, or interrupted attempt. Report missing fields as
  missing rather than estimating them from successful attempts.

The objective-provided P-192 incumbent **10,453** is a campaign reference,
not a newly revalidated receipt in this survey. No observation in this document
is eligible for promotion to the standard-curve expanded-map scoreboard.

## Reproducible survey artifacts and remaining work

- [sources.json](sources.json) records exact public URLs, versions, PDF byte
  sizes and SHA-256 hashes, inspected PDF page numbers, and access limitations.
- [findings.json](findings.json) classifies nine observations from seven
  sources. Unreported degrees and absent replay results are JSON `null` or
  explicit `not_run` statuses, rather than invented maxima or passing rows.

PDF page numbers start at one. The PDFs and extracted copyrighted text are not
redistributed. Hashes identify the bytes inspected; they do not prove the
mathematics. Search and KB hits were discovery aids; source text was checked
before classification. This bounded search is not a completeness proof.

The next computational milestone remains a reproducible attempt above the
P-192 degree-10,453 reference, followed by independent replay, while preserving
separate boards for every registered standard prime-field curve. The storage
recovery and native comparison check are still prerequisites of the saved
execution path. This literature work neither runs that check nor waives its
gates. External ECLIPSE, PRISM and historical evaluation results require their
own source-pinned reproduction tasks before they can support a record claim.
