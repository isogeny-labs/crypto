#set page(paper: "a4", flipped: true, margin: 16mm)
#set text(size: 10pt)

= Variable effects in the degree-fall toy system

#text(fill: luma(90))[2026-11-18 · Run 20261118T0135Z · Singular 4.4.1 on Linux x86-64]

#image("variable_effects.svg", width: 100%)

*Regularity.* Pin each of $x$, $y$, and $z$ in turn at the shared root coordinate $1$. Compare the first degree with no standard monomials before and after adding the pin. Any pin changes that degree from 4 to 1 for the first three dense quadrics. With four to six dense quadrics it is already 1. [pcc:unverified]

*Diffusion.* For each source variable, compare each equation with that variable set to $2$ and to $1$, leaving the other variables symbolic. An arrow counts equations where their difference depends on the destination variable. Every dense pair is coupled in every equation. The diagonal control has no arrows. The chain control has direct links through $y$ and no direct $x$–$z$ link. These are local structural measurements on the listed fixtures. [pcc:unverified]

*Evidence.* The Singular script checks the four dense baselines against the #link("https://github.com/isogeny-labs/crypto/blob/main/research/degree_fall_trace_20261117/reports/run-20261117T1511Z/degree_fall_dataset.json")[earlier Rust trace]. The figure reads 18 rows from #raw("variable_effects.csv"), covering four nested dense systems and two sparse controls. See #raw("measure_variable_effects.sing") for the equations and #raw("plot_variable_effects.py") for the rendering.
