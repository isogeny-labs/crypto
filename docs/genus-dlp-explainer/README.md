# Why discrete logarithms are cheaper on Jacobians of genus 2 and above

Explainer with diagrams and equations, 8 pages. Literature summary only: no
measurement, no new result, every cost statement is an asymptotic exponent
quoted from the cited paper and is `[pcc:unverified]` in this repository's
sense.

- `report.pdf` — the deliverable (start on page 1 for the answer and Diagram 1;
  page 4 for the exponent graph; page 7 for what it means for a prime-field
  elliptic curve).
- `report.typ` — editable Typst source. Build:
  `/tmp/report-venv/bin/python -c "import typst; typst.compile('report.typ', output='report.pdf')"`
  (Typst via a venv, as the `research-visuals` skill prescribes; the native
  diagrams are drawn in Typst, so no plotting library is needed).
- `make_figures.py` — pure-Python generator for `fig_exponents.svg`, the
  exponent-versus-genus graph; `fig_exponents.json` holds the plotted values.

Contents: Mumford representation and the exact 1/g! smoothness law; the
relation-versus-linear-algebra balance giving 2 - 2/(g+1), and the published
large-prime ladder to 2 - 2/g; exponents by genus against Pollard rho g/2;
Diem's line relations on plane quartics (Õ(q) for non-hyperelliptic genus 3,
2 - 2/(g-1) in general); Smith's isogeny transfer, Weil descent, the
decomposition attack and the L(1/2), L(1/3) large-genus regime; the field-size
floor that keeps every transfer out of a prime-field curve above rho.
