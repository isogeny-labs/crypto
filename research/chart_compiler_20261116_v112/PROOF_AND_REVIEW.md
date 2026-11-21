# Different-dimension chart importer

Old source inputs76 and new inputs67 are distinct. Every old input is an exact
affine form in new inputs. XOR/NOT propagate those forms; nonlinear gates
emit them lazily. Boolean constant/equal/complement identities are exact.
Induction proves every imported output equals the original on recovered theta.
Ancestor pruning retains all inputs and all required gates. The same importer
accepts already computed generator wires: forms then represent XORs of those
wire functions, not an assumption that derived wires are independent inputs.

Each C111chart is an injective map. Its two images are disjoint and exhaust
the hull-compatible guarded necessary family. Comparing the UNION of complete
small original root sets tests completeness, rather than only pointwise value
agreement. Empty/inconsistent charts and noninjective test maps are included.
All repeated-root predicates and original q semantics are preserved.

Both chart costs are charged. Two individually smaller circuits can be worse
in total. Native search promotion requires total variables AND clauses below
the frozen C105control; circuit counts are not end-to-end speed ratios.
One core4096MiB,90second outer worker,78/84second finalization checks. No
original339-variable all-roots or broad field performance claim is made.
