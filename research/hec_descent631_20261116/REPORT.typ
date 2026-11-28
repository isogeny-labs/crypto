#set page(paper:"a4",margin:24mm)
#set text(size:11pt)
= Verified F631 staged index-calculus solve
Genus 3 · 15 November 2026

#text(size:17pt,weight:"bold")[Recovered scalar: 15,648,086]

Public scalar multiplication verifies B = k D1 on the original hyperelliptic source and on the recovered plane-quartic destination classes. The source also rejects k + 1. No expected scalar was supplied.

#table(columns:(1fr,1fr,1fr),[Input],[Successful / attempted conics],[Projected log],[D1],[4 / 20],[53659370],[B],[4 / 20],[40380864])

== Descent and exact controls
Each input uses two rational representatives and all ten pairs among five fixed runtime anchor points. Sixteen nonsplit residuals per input remain in the record. All successful descents agree.

Every candidate resultant is compared with an independent Sylvester determinant at all631field elements. Accepted residuals are reduced, disjoint and split; the actual target Jacobian verifies the conic divisor identity.

The full four-point canonical divisor is used. Point values and derived class logs use the same fourfold projection. The calibration image has its own factor four, reflected in its projected log7720582. The common projection factor cancels in the final ratio modulo59296481.

== Evidence chain
Public source and quotient construction → calibration and withheld alignment → algebraic inverse systems → rational class recovery with adjacent sign checks → fresh sparse point relations → all597geometric point-log checks → conic descent → source and destination scalar verification.

== Custody and remaining objective
Freezeaa1dce88ec63 and checkpoint20261116T025045Z precede one execution. Exit zero; stderr empty. All exclusions are retained.

This is staged correctness with previously constructed setup and artifacts. It is not a cold timing measurement or proof of the global correspondence. The earlier failed charged comparison remains unchanged. Full cold integration, charged comparisons, broader applicability and genus-two/four mechanisms remain. No tenfold gain or new complexity bound is claimed.
