# Restricted-root incidence admission: a deterministic requirement

This note turns the observed restricted-producer failure into a quantitative research criterion. It does not prove a general index-calculus lower bound or rule out large-prime algorithms, which can reduce matrix cost. No new scientific execution, benchmark retuning or novelty claim. [pcc:unverified]

## What the retained failure actually says

restricted-pencil/GATES.tsv records0–8acceptedpartialrows per genus4workload over128anchors. Every prefix agrees with the independent solver, and none determines the target. Thus the fixed producer produces very sparse usable input on these targets. The records do not retain the entire terminal matrix, LPsupport graph or all rejected candidates; they do not independently identify how much deficit comes from coverage versus dependence. Do not label an unmeasured cause as established.

## Deterministic incidence inequality

Fix an anchor panel, and let M be the number of squarefree fully split degree-nresiduals satisfying the same support/multiplicity rules, n=g+1. Fix a small-prime subset. Let X_i count small roots of full residual i. A restricted producer that knows only small roots and factors a residual quotient of degree at most2 can accept only rows with X_i>=k=n-2=g-1. Let A count such rows, and I=sum_i X_i. Exactly, with no randomness assumption,

    k*A <= I.

Let rho be the fraction of eligible rational signed factor-base points in the small subset, not merely its fraction of orbit columns. If M>0 and rho>0, define its measured incidence enrichment

    kappa=I/(n*rho*M).

Then

    (A/M)/rho <= kappa*n/k.

Proof: multiply k*A<=I by1/(k*rho*M). The definition of kappa measures concentration; it is not an assumed free advantage. Orbit weights/sign cancellation do not change root-incidence counts, but actual useful rank must still be measured separately.

If point-scan work scales withrho, this expression is the gain in accepted-full-row rate per scan visit over a full scan of the SAME fixed panel. It is not full runtime: parameter arithmetic, factoring, touched-bucket storage, setup, LPelimination and changed stopping anchors remain. Complete-scan baseline records that stop early do not supply M or I for a later128anchor panel; these quantities have NOT been measured here.

## Uniform-marginal consequence, without independence

If conditional on full splitting each residual root has small-subset marginalrho, then expected I=n*rho*M. Taking expectations gives expectedA<=n*rho*M/k. Independence of roots is unnecessary for this bound. Therefore the ratio of expectedacceptedrowyield toscanbudget is bounded byn/k:3,2,5/3 for genera2,3,4. This bounds only that declared ratio under uniform marginals, not expected reciprocal stopping time or useful-rank rate. Structured pencils can violate uniform marginals; measurekappa rather than claiming they do not.

Under the stronger independent-root model, F(rho)=Pr(Binomial(n,rho)>=k) gives exact heuristic proportions. Its sharper polynomial ratios are

    g2: F/rho=3-3rho+rho²
    g3: F/rho=6rho-8rho²+3rho³
    g4: F/rho=10rho²-15rho³+6rho4.

These are modeling equations, not observed probabilities. They cannot rescue incomplete cases or justify a selected favorable cut after execution.

## Tenfold claim requirements

For a tenfold accepted-row-per-scan gain attributed to concentration alone, the deterministic inequality requireskappa>=10*k/n: at least10/3,5,6 for genera2,3,4. These are necessary conditions for that specific scan-rate claim, NOT necessary conditions for every possible tenfold full algorithm. A method can instead improve the final matrix, the fraction of independent target-bearing information per row, setup, or arithmetic. Those effects need explicit complete-cost evidence and cannot be inferred from A,I or cycle counts alone.

An admissible next candidate should identify its source of concentration or rank/matrix advantage before a campaign. Examples of obligations: a rule selecting small primes without uncharged full-target scans; a proof or prospectively frozen estimate of its incident-root distribution; retained independent weighted-rank checks; same public source normalization; final source/target verification; all pilot/training/storage and failed-attempt costs. Target-independent reuse is amortized work and must be labeled/charged accordingly. No candidate currently meets a demonstrated broad tenfold case.

## Literature boundary

Primary Improved sieving on algebraic curves, https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/SievingB.pdf sections2.2–2.3, supplies the classical requirement ofg-1small roots followed by a quadratic remainder and the importance of sparse parameter bookkeeping. The counting inequality here is elementary double counting applied to that method; no priority claim. Existing gain-graph rank results distinguish LPleft-nullity from independent label rank. The full genus2/3/4 mathematical objective remains open.
