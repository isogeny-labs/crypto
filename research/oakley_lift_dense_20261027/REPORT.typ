#set page(margin: 1.6cm)
#set text(size: 10pt)
= Twisted line on the Oakley EC2N curves

Run run-20261027T1445Z, seed 0xAC120261027, 200 samples per row. Source of every number: `reports/run-20261027T1445Z/rows.json`.

#table(
  columns: 8,
  align: (left, right, right, right, right, right, right, right),
  [curve], [N], [d], [n], [Tr(a)], [u found (lines)], [twisted lift], [standard lift],
  [Oakley 3], [155], [31], [5], [0], [yes (6)], [200/200], [104/200],
  [Oakley 4], [185], [37], [5], [0], [yes (3)], [200/200], [99/200],
  [c2pnb176v1], [176], [44], [4], [1], [yes (3)], [0/200], [122/200],
  [c2pnb208w1], [208], [52], [4], [0], [yes (1)], [200/200], [108/200],
  [c2pnb272w1], [272], [68], [4], [1], [yes (2)], [0/200], [103/200],
  [c2pnb304w1], [304], [76], [4], [1], [yes (5)], [0/200], [98/200],
  [c2pnb368w1], [368], [92], [4], [0], [yes (3)], [200/200], [109/200],
)

Positive control behaves as predicted. On both Oakley curves a u with
T(u) = T(c/u) = 0 exists and was verified directly in $F_(2^N)$; all sampled
elements of the line $u F_q$ lift. If the twisted line is used as the
Gaudry-Diem base for n = 5, each nonzero x lifts, so a decomposition trial
has 2^5 = 32 times the relations of the standard base. This is a per-trial
relation-yield statement, not a measured cost; composition with the planned
subfield-base lane and the GHS genus-16 transfer pricing are not examined.

#figure(
  table(columns: 3, [search to first u (s)], [Oakley 3], [Oakley 4], [], [0.046], [0.051]),
  caption: [Timing on a 4-core Xeon 2.10 GHz VM; whole run 64.5 s.],
)
