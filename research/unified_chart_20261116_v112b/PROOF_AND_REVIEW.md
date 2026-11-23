# Exact common-kernel chart union

Let two disjoint affine charts have dimension d and direction-space
intersection C of dimension d-1. Choose independent extra directions a,b
and offsets p,q. Then every point in their union has exactly one encoding
theta=p+C*gamma+t*a+s*(p+q)+(s*t)*(a+b).
Here gamma has d-1bits; t chooses the extra direction and s chooses the
chart. Each branch is injective; disjointness gives global injectivity.
The actual d=67maps use66shared bits plus t,s, giving68inputs and one new
selector product. No extra old q bit is dropped or treated as independent.

The importer treats s*t as an already computed wire. Complete affine forms
are XORs of primary inputs and this wire, not an assumption of independence
among generators. Gate induction and pruning preserve every source output.
The guards hold identically by coverage proof, even if the circuit has not
syntactically simplified every equivalent zero expression.

64exhaustive tiny unions test injectivity, complete guarded image and full
old/new root sets for random circuits.64actual controls compare every old
wire output, all fourteen numeric predicates and old guard after recovery.
Both chart graphs and all earlier negative evidence remain preserved.

This is a circuit/formulation experiment, not a cold runtime comparison to
genuine F4/F5/F6 or complete original339-variable solve. One core4096MiB,
90s outer supervisor,78/84s finalization boundaries. Native promotion only
when BOTH variable and clause counts improve the same C105source control.
Affine decomposition, Shannon selection and common-subspace factoring are
known ideas; a novel general rootfinding algorithm is not established.
