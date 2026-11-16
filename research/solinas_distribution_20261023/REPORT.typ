// Some definitions presupposed by pandoc's typst output.
#let blockquote(body) = [
  #set text( size: 0.92em )
  #block(inset: (left: 1.5em, top: 0.2em, bottom: 0.2em))[#body]
]

#let horizontalrule = [
  #line(start: (25%,0%), end: (75%,0%))
]

#let endnote(num, contents) = [
  #stack(dir: ltr, spacing: 3pt, super[#num], contents)
]
#show terms: it => {
  it.children
    .map(child => [
      #strong[#child.term]
      #block(inset: (left: 1.5em, top: -0.4em))[#child.description]
      ])
    .join()
}

#let conf(
  title: none,
  authors: none,
  date: none,
  abstract: none,
  cols: 1,
  margin: (x: 0.8in, y: 0.8in),
  paper: "us-letter",
  lang: "en",
  region: "US",
  font: ("Libertinus Serif", "New Computer Modern", "DejaVu Sans"),
  fontsize: 10pt,
  sectionnumbering: none,
  doc,
) = {
  set page(
    paper: paper,
    margin: margin,
    numbering: "1",
  )
  set par(justify: true)
  set text(lang: lang,
           region: region,
           font: font,
           size: fontsize)
  set heading(numbering: sectionnumbering)

  if title != none {
    align(center)[#block(inset: 2em)[
      #text(weight: "bold", size: 1.5em)[#title]
    ]]
  }

  if authors != none {
    let count = authors.len()
    let ncols = calc.min(count, 3)
    grid(
      columns: (1fr,) * ncols,
      row-gutter: 1.5em,
      ..authors.map(author =>
          align(center)[
            #author.name \
            #author.affiliation \
            #author.email
          ]
      )
    )
  }

  if date != none {
    align(center)[#block(inset: 1em)[
      #date
    ]]
  }

  if abstract != none {
    block(inset: 2em)[
    #text(weight: "semibold")[Abstract] #h(1em) #abstract
    ]
  }

  if cols == 1 {
    doc
  } else {
    columns(cols, doc)
  }
}
#show: doc => conf(
  cols: 1,
  doc,
)


= Do Solinas-shape primes change the distribution of `#E(F_p)`?
<do-solinas-shape-primes-change-the-distribution-of-ef_p>
Protocol: #link("PROTOCOL.md")[`PROTOCOL.md`], frozen in commit
`588bdf5ac` before the run. Instrument:
#link("../../examples/solinas_order_census.rs")[`examples/solinas_order_census.rs`].
Raw rows: #link("runs/census.jsonl")[`runs/census.jsonl`] (one line per
prime), summary
#link("runs/census.summary.txt")[`runs/census.summary.txt`], trace
`runs/census.log`. Figure:
#link("figures/census_pvalues.svg")[`figures/census_pvalues.svg`].
Conductor task T-4.

== Result
<result>
Each prime had `N = 4000` uniformly random curves `y² = x³ + ax + b`,
counted exactly. The sample was 29 Solinas-shape primes
(`2^n − 2^k ± 1`, 16–22 bits, `6 ≤ k ≤ 17`), 58 residue-matched random
primes (same size, same `p mod 480`) and 58 generic random primes. Run
time: 9 min 23 s on 4 threads (the protocol estimated about 20 min).

#align(center)[#table(
  columns: 6,
  align: (col, row) => (left,right,right,right,right,right,).at(col),
  inset: 6pt,
  [role], [primes], [F1 `v₂(#E)` rejects], [F3 `3 ∣ #E` rejects], [F4
  `5 ∣ #E` rejects], [F2 control rejects],
  [Solinas-shape],
  [29],
  [0],
  [0],
  [0],
  [29 / 29],
  [residue-matched random],
  [58],
  [0],
  [0],
  [0],
  [58 / 58],
  [generic random],
  [58],
  [0],
  [0],
  [0],
  [58 / 58],
)
]

Bonferroni threshold for F1/F3/F4 over 435 tests at `α = 0.01`:
`2.3·10⁻⁵`. The F2 control uses `α / 145 = 6.9·10⁻⁵`; its largest
p-value anywhere was `1.2·10⁻⁶`, and its median noncentrality was 76 on
4 degrees of freedom.

#strong[Verdict, in the protocol\'s wording.] The instrument is valid
(F2 rejected the wrong-class model for every prime). Solinas-shape
primes are #strong[within the matrix model for their own residue class
at this resolution], as are the residue-matched and generic primes. By
§4 of the protocol this licenses \"Solinas shape adds nothing beyond its
residue class\" for these three statistics at this size.

Descriptive, not part of the decision rule: the smallest target p-values
were `6.0·10⁻⁴` (F4), `2.5·10⁻²` (F1) and `2.6·10⁻²` (F3). With 435
tests the smallest expected p-value under the null is a few `10⁻³`, so
none is unusual.

== What the model says about the Solinas class (exact, no sampling)
<what-the-model-says-about-the-solinas-class-exact-no-sampling>
The enumeration gives a second result that does not depend on the
sample. For all 29 targets, the total-variation distance between the
prime\'s own-residue model of `min(v₂(#E), 5)` and the generic model
(mean over primes with the same `p mod 4`) is `0` for the 21 targets
with `p ≡ 7 mod 8` and `0.0026` for the 8 with `p ≡ 1 mod 8`, whatever
`k` is (6 to 17). So having `p ≡ ±1 mod 2^k` for large `k` changes the
`v₂(#E)` distribution no more than `p mod 8` already does.

== What this does not show
<what-this-does-not-show>
- It does not test the claim at 224 bits, at `a = −3`, for structured
  curves (GLV, CM), for the odd part of `#E` beyond divisibility by 3
  and 5, or for index-calculus relation yield. It says nothing about
  ECDLP cost.
- It cannot resolve a difference of the size the model itself predicts
  between the Solinas class and the generic class (`≤ 0.003`). That
  would need about `10⁶` curves per prime. The result is \"fits the
  model\", not \"differs from generic\".
- A pattern the matrix model does not describe, for instance one that
  appears only at much larger `p`, would not be seen here.

== Corrections made before the run
<corrections-made-before-the-run>
The first positive control (the generic model) had no power, because the
generic model differs from the own-residue model by at most `0.003`. It
was replaced by the wrong-class model before any real data, and this is
disclosed in `PROTOCOL.md` §6.

#image("figures/census_pvalues.png")
