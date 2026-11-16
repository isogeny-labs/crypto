#set page(paper: "a4", margin: (x: 18mm, y: 16mm))
#set text(font: "Liberation Sans", size: 9pt)
#set heading(numbering: none)
#let green = rgb("176c43")
#let amber = rgb("9c5d0b")

= Weighted-Proj componentwise quotient prerequisite

#text(fill: rgb("475569"))[Bounded Lean proof packet · 25 October 2026 · arbitrary field and coefficients]

The pinned Lean file proves two prerequisites for a graded quotient of the weight (1,3,1) sextic. It does not construct the graded quotient, identify the degree-zero localization, or prove a projective chart.

#image("componentwise-dependencies.svg", width: 100%)

#text(size: 8pt, fill: rgb("475569"))[Figure 1. Green boxes are compiler-checked statements; amber boxes remain open proof obligations. Arrows indicate dependency order.]

== Checked statements

- *Finite homogeneous lifts:* every ordinary quotient class has a finite sum of images of weighted-homogeneous polynomial components.
- *Componentwise equality:* quotient representatives agree exactly when their weighted components agree in every degree.

Both theorems are in `WeightedProjDegreeZeroScratch.lean`. Lean 4.34.1 with mathlib `d13f23b` exited 0. The printed axiom audits list only `propext`, `Classical.choice`, and `Quot.sound`.

== Exact evidence

Source SHA-256: `2026d342f368bd89bc60f16267edbdd8d6299c5401d6c625c539202aa3384377`

Compiler log SHA-256: `d8ebceae0a02c6ce59ce3269cbb9cc7cb07ba1b8e4e9face104c67ad73414317`

The replay command, imported-module path, theorem statements, and checked/open rows are in `README.md` and `componentwise-status.csv`. No performance measurement is reported here.

== Open boundary

The next steps are quotient degree pieces and a `GradedRing` instance, then an equivalence from the degree-zero homogeneous localization away from `Z` to the already established ordinary `Z = 1` quotient. Only after those steps can mathlib's `Proj.basicOpenIsoSpec` be specialized to this curve.
