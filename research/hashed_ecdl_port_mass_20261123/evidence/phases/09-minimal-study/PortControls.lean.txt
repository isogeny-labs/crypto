import CryptoLean.Combinatorics.PortMass
open scoped BigOperators

namespace NativePortAccounting

/-- Endpoint tags lie in Fin radix; parallel incidences retain multiplicity. -/
theorem source_tags (radix : Nat) (edges : Multiset (Fin radix × Fin radix)) :
    (∑ g : Fin radix, ((edges.map Prod.fst).count g + (edges.map Prod.snd).count g)) =
      2 * edges.card :=
  CryptoLean.PortMass.grouped_port_mass edges

/-- An old vertex's group is independent of its degree, so isolates remain. -/
theorem source_vertices (radix : Nat) (oldLabels : Multiset (Fin radix)) (packets : Nat) :
    (∑ g : Fin radix, (oldLabels.count g + packets)) =
      oldLabels.card + radix * packets := by
  simpa using CryptoLean.PortMass.grouped_vertex_mass oldLabels packets

/-- Nonempty parallel endpoint tags are not a set of distinct parent edges. -/
theorem parallel_example :
    (∑ g : Fin 2,
      ((({(0, 1), (0, 1), (1, 1)} : Multiset (Fin 2 × Fin 2)).map Prod.fst).count g +
      (({(0, 1), (0, 1), (1, 1)} : Multiset (Fin 2 × Fin 2)).map Prod.snd).count g)) = 6 := by
  simpa using CryptoLean.PortMass.grouped_port_mass
    ({(0, 1), (0, 1), (1, 1)} : Multiset (Fin 2 × Fin 2))

#print axioms CryptoLean.PortMass.grouped_port_mass
#print axioms CryptoLean.PortMass.grouped_port_weight
#print axioms CryptoLean.PortMass.grouped_vertex_mass
#print axioms source_tags
#print axioms source_vertices
#print axioms parallel_example
end NativePortAccounting
