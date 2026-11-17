#set page(paper:"a4",margin:18mm)
#set text(font:"DejaVu Serif",size:10pt)
= Candidate reduction of prime chain syzygies
*Unvalidated research direction. No new FFD/LFD result or production run.*

The cubic development maps expose the cost of direct coefficient matrices: the prospective m9 domain has882816columns. This search asks whether the original27-selector systems can be analyzed by graded modules instead. The existing conditional FFD intervals7–17 remain authoritative.
#image("candidate.svg",width:100%)
*Literature checked.* Lundqvist–Nicklasson,Theorem2.7 and Section3, gives the strong Lefschetz property for multiplication by the coordinate sum when the characteristic exceeds the socle degree. The block socle degree is27, below the83-bit prime. This establishes a block fact,not the chain argument.

*Proposed chain argument.* Decompose each squarefree block into graded strings. In a primitive sector of total degree K,syzygy coefficients have degree d-4-K. Chain quartics have degree at most two in each block sum. The proposed sufficient no-truncation condition is2d-6≤27,covering d≤16. Give block sums weight m and intermediate U_j weight m-2-j. The candidate leading monomials have pairwise disjoint supports. If these checks establish a regular sequence in the free block-sum ring,transport its Koszul syzygies through the strings and then through selector-square reduction.

*Open obligations.* Prove the graded decomposition and transport,check every boundary and source normalization,compare sector kernels against the retained direct maps,and include bad-characteristic/truncation/leading-term negative controls. Freeze any all42-case production certificate run separately. Existing upper17 evidence can only be joined after exact source/base identity checks. Independent scientific review and formalization remain pending.

No exactFFD17 claim is made for higher m here. No LFD,solving-degree or elite-base conclusion follows automatically. PRIME_LEFSCHETZ_CANDIDATE.md records the full proposed argument and evidence checklist. The direct cubic development data are preserved as reference controls.

#link("https://arxiv.org/html/1604.06820")[Lundqvist–Nicklasson,On the structure of monomial complete intersections in positive characteristic].
#link("https://arxiv.org/html/2112.05579")[Caminata–Gorla,Solving degree,last fall degree,and related invariants].
