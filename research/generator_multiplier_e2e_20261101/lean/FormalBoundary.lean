/-!
# Formal boundary for generator-multiplier factor bases

This file proves only the representation and relation-transport statements used
by the accompanying experiment.  It deliberately makes no running-time or
asymptotic claim.

* An invertible additive coordinate change preserves pair relations exactly.
* A split additive embedding into an extension preserves transported pair
  relations exactly.
* In an odd-order subgroup, two quadratic trace-zero points cannot add to a
  nonzero Frobenius-fixed endpoint.
* A generator-multiplier vertex contributes a known weight times one seed
  column.  Consequently a list of such vertices evaluates to a weighted
  linear form in the seed logs; it does not introduce one unknown per vertex.

The extension statements do not rule out higher-degree decomposition
algorithms or factor bases that are not merely transported along the supplied
maps.  The end-to-end cost of the multiplier representation is measured, not
proved, by the frozen prime and binary panels next to this file.
-/

namespace GeneratorMultiplierE2E

/-! ## Coordinate and extension transport -/

/-- A carrier with the addition operation needed for pair relations. -/
structure AdditiveSystem (Point : Type) where
  add : Point → Point → Point

/-- An explicitly invertible additive coordinate change. -/
structure AdditiveCoordinateEquiv (Source Target : Type) where
  source : AdditiveSystem Source
  target : AdditiveSystem Target
  toFun : Source → Target
  invFun : Target → Source
  leftInv : ∀ P, invFun (toFun P) = P
  mapAdd : ∀ P R, toFun (source.add P R) = target.add (toFun P) (toFun R)

theorem coordinate_toFun_injective {Source Target : Type}
    (e : AdditiveCoordinateEquiv Source Target) {P R : Source}
    (h : e.toFun P = e.toFun R) : P = R := by
  rw [← e.leftInv P, ← e.leftInv R, h]

/-- Coordinate isomorphisms neither create nor remove transported pair relations. -/
theorem coordinate_pair_relation_iff {Source Target : Type}
    (e : AdditiveCoordinateEquiv Source Target) (P R T : Source) :
    e.source.add P R = T ↔
      e.target.add (e.toFun P) (e.toFun R) = e.toFun T := by
  constructor
  · intro h
    rw [← e.mapAdd, h]
  · intro h
    apply coordinate_toFun_injective e
    rw [e.mapAdd, h]

/-- An additive embedding equipped with a left inverse on its image. -/
structure SplitAdditiveEmbedding (Source Extension : Type) where
  source : AdditiveSystem Source
  extension : AdditiveSystem Extension
  embed : Source → Extension
  retract : Extension → Source
  retractEmbed : ∀ P, retract (embed P) = P
  mapAdd : ∀ P R, embed (source.add P R) = extension.add (embed P) (embed R)

theorem split_embedding_injective {Source Extension : Type}
    (e : SplitAdditiveEmbedding Source Extension) {P R : Source}
    (h : e.embed P = e.embed R) : P = R := by
  rw [← e.retractEmbed P, ← e.retractEmbed R, h]

/-- Merely embedding a base-field relation into an extension creates no new relation. -/
theorem transported_extension_pair_relation_iff {Source Extension : Type}
    (e : SplitAdditiveEmbedding Source Extension) (P R T : Source) :
    e.source.add P R = T ↔
      e.extension.add (e.embed P) (e.embed R) = e.embed T := by
  constructor
  · intro h
    rw [← e.mapAdd, h]
  · intro h
    apply split_embedding_injective e
    rw [e.mapAdd, h]

/-! ## The quadratic trace-zero obstruction -/

/--
The abstract properties needed for the quadratic Frobenius control.
`selfNegEqZero` holds on the odd-order subgroup under test.
-/
structure FrobeniusSystem (Point : Type) where
  add : Point → Point → Point
  neg : Point → Point
  zero : Point
  frob : Point → Point
  addComm : ∀ P R, add P R = add R P
  frobAdd : ∀ P R, frob (add P R) = add (frob P) (frob R)
  frobInvolutive : ∀ P, frob (frob P) = P
  negAdd : ∀ P R, neg (add P R) = add (neg P) (neg R)
  selfNegEqZero : ∀ P, P = neg P → P = zero

/-- The extension-field trace `P + Frobenius(P)`. -/
def frobeniusTrace {Point : Type} (F : FrobeniusSystem Point) (P : Point) : Point :=
  F.add P (F.frob P)

/-- Frobenius traces are fixed, hence lie in the base-field eigenspace. -/
theorem frobenius_trace_fixed {Point : Type} (F : FrobeniusSystem Point)
    (P : Point) : F.frob (frobeniusTrace F P) = frobeniusTrace F P := by
  rw [frobeniusTrace, F.frobAdd, F.frobInvolutive, F.addComm]

/-- Trace-zero points are closed under addition. -/
theorem trace_zero_add_closed {Point : Type} (F : FrobeniusSystem Point)
    (P R : Point) (hP : F.frob P = F.neg P) (hR : F.frob R = F.neg R) :
    F.frob (F.add P R) = F.neg (F.add P R) := by
  rw [F.frobAdd, hP, hR, ← F.negAdd]

/--
An odd-order quadratic trace-zero sum that is also Frobenius-fixed is zero, so
this construction cannot supply a nonzero base-subgroup pair endpoint.
-/
theorem trace_zero_sum_fixed_is_zero {Point : Type} (F : FrobeniusSystem Point)
    (P R : Point) (hP : F.frob P = F.neg P) (hR : F.frob R = F.neg R)
    (hfixed : F.frob (F.add P R) = F.add P R) :
    F.add P R = F.zero := by
  apply F.selfNegEqZero
  exact hfixed.symm.trans (trace_zero_add_closed F P R hP hR)

/-! ## Known-weight multiplier columns -/

/-- The additive fragment of scalar encoding used by the relation table. -/
structure CyclicEncoding (Point : Type) where
  point : Nat → Point
  add : Point → Point → Point
  mapAdd : ∀ a b, add (point a) (point b) = point (a + b)

/-- A table vertex names a seed column and a known scalar weight. -/
structure MultiplierVertex (Column : Type) where
  column : Column
  weight : Nat

/-- Scalar exponent represented by one multiplier vertex. -/
def vertexExponent {Column : Type} (seedLog : Column → Nat)
    (v : MultiplierVertex Column) : Nat :=
  v.weight * seedLog v.column

/-- Point represented by one multiplier vertex. -/
def vertexPoint {Point Column : Type} (E : CyclicEncoding Point)
    (seedLog : Column → Nat) (v : MultiplierVertex Column) : Point :=
  E.point (vertexExponent seedLog v)

/-- A two-sum endpoint is a known weighted linear form in two seed columns. -/
theorem pair_endpoint_eq_weighted_columns {Point Column : Type}
    (E : CyclicEncoding Point) (seedLog : Column → Nat)
    (left right : MultiplierVertex Column) :
    E.add (vertexPoint E seedLog left) (vertexPoint E seedLog right) =
      E.point (vertexExponent seedLog left + vertexExponent seedLog right) := by
  exact E.mapAdd _ _

/-- Exponent represented by a complete relation row. -/
def relationExponent {Column : Type} (seedLog : Column → Nat) :
    List (MultiplierVertex Column) → Nat
  | [] => 0
  | v :: rest => vertexExponent seedLog v + relationExponent seedLog rest

/-- Point endpoint represented by a complete relation row. -/
def relationEndpoint {Point Column : Type} (E : CyclicEncoding Point)
    (seedLog : Column → Nat) : List (MultiplierVertex Column) → Point
  | [] => E.point 0
  | v :: rest => E.add (vertexPoint E seedLog v) (relationEndpoint E seedLog rest)

/--
Every relation row evaluates to its weighted seed-column exponent.  Thus table
vertices may outnumber matrix unknowns without changing relation semantics.
-/
theorem relation_endpoint_eq_weighted_columns {Point Column : Type}
    (E : CyclicEncoding Point) (seedLog : Column → Nat)
    (vertices : List (MultiplierVertex Column)) :
    relationEndpoint E seedLog vertices = E.point (relationExponent seedLog vertices) := by
  induction vertices with
  | nil => rfl
  | cons vertex rest ih =>
      simp only [relationEndpoint, relationExponent, vertexPoint, ih]
      exact E.mapAdd _ _

/-- Two vertices on one seed column collapse to the sum of their known weights. -/
theorem same_column_pair_collapse {Point Column : Type}
    (E : CyclicEncoding Point) (seedLog : Column → Nat)
    (column : Column) (leftWeight rightWeight : Nat) :
    E.add (E.point (leftWeight * seedLog column))
        (E.point (rightWeight * seedLog column)) =
      E.point ((leftWeight + rightWeight) * seedLog column) := by
  rw [E.mapAdd, Nat.add_mul]

end GeneratorMultiplierE2E
