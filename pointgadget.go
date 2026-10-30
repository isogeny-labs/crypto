package ecdl

import "math/big"

// FieldPoint is a point whose coordinates are Vars. Inf, if not nil, is a
// 1-bit Var flagging the identity, encoded canonically as Inf = 1, X = Y = 0.
// A nil Inf means the point is known to be affine.
type FieldPoint struct{ X, Y, Inf *Var }

// AddGeneric defines P3 = P1 + P2 for the generic case x1 != x2, using only
// low-degree gadgets:
//
//	dx = x2 - x1 (nonzero), dy = y2 - y1, lambda*dx = dy,
//	s = lambda^2, x3 = s - x1 - x2, e = x1 - x3, f = lambda*e, y3 = f - y1.
//
// Every intermediate value is range-checked to [0, p), so it is pinned down
// uniquely. Pairs with x1 = x2 (doubling, P + (-P)) and the identity have no
// satisfying assignment.
func (b *Builder) AddGeneric(ar *Arith, p1, p2 FieldPoint) FieldPoint {
	zero := new(big.Int)
	dx := b.FieldVar(ar, "dx")
	b.LinCheck(ar, dx, true, []Term{{-1, p2.X}, {1, p1.X}}, zero)
	b.RangeCheck(ar, dx, ar.P, true)
	dy := b.FieldVar(ar, "dy")
	b.LinCheck(ar, dy, true, []Term{{-1, p2.Y}, {1, p1.Y}}, zero)

	lambda := b.FieldVar(ar, "lambda")
	p := ar.P
	b.Hint([]*Var{dx, dy}, []*Var{lambda}, func(in []*big.Int) [][]*big.Int {
		inv := new(big.Int).ModInverse(in[0], p)
		if inv == nil {
			return nil
		}
		return [][]*big.Int{{inv.Mul(inv, in[1]).Mod(inv, p)}}
	})
	b.RangeCheck(ar, lambda, ar.P, false)
	b.MulCheck(ar, Operand{V: lambda}, Operand{V: dx}, dy, false)

	s := b.FieldVar(ar, "s")
	b.MulCheck(ar, Operand{V: lambda}, Operand{V: lambda}, s, true)
	x3 := b.FieldVar(ar, "x3")
	b.LinCheck(ar, x3, true, []Term{{-1, s}, {1, p1.X}, {1, p2.X}}, zero)
	e := b.FieldVar(ar, "e")
	b.LinCheck(ar, e, true, []Term{{-1, p1.X}, {1, x3}}, zero)
	f := b.FieldVar(ar, "f")
	b.MulCheck(ar, Operand{V: lambda}, Operand{V: e}, f, true)
	y3 := b.FieldVar(ar, "y3")
	b.LinCheck(ar, y3, true, []Term{{-1, f}, {1, p1.Y}}, zero)
	return FieldPoint{X: x3, Y: y3}
}

// ValidityCheck constrains P to be an affine point of c with canonical
// coordinates: y^2 = x^3 + a x + b. The coordinates must be defined already.
func (b *Builder) ValidityCheck(ar *Arith, c *Curve, pt FieldPoint) {
	b.RangeCheck(ar, pt.X, ar.P, false)
	b.RangeCheck(ar, pt.Y, ar.P, false)
	y2 := b.FieldVar(ar, "y2")
	b.MulCheck(ar, Operand{V: pt.Y}, Operand{V: pt.Y}, y2, true)
	x2 := b.FieldVar(ar, "x2")
	b.MulCheck(ar, Operand{V: pt.X}, Operand{V: pt.X}, x2, true)
	x3 := b.FieldVar(ar, "x3")
	b.MulCheck(ar, Operand{V: x2}, Operand{V: pt.X}, x3, true)
	ax := b.FieldVar(ar, "ax")
	b.MulCheck(ar, Operand{C: new(big.Int).SetUint64(c.A)}, Operand{V: pt.X}, ax, true)
	b.LinCheck(ar, nil, false, []Term{{1, y2}, {-1, x3}, {-1, ax}}, new(big.Int).Neg(new(big.Int).SetUint64(c.B)))
}

// EqualsConst constrains v = k block by block.
func (b *Builder) EqualsConst(ar *Arith, v *Var, k *big.Int) {
	blk := b.use(v)
	kb := ar.blocksOf(k)
	for j := range blk {
		want := kb[j]
		b.add(&PredVertex{E: blk[j], P: func(vals []bool) bool { return bitsUint(vals).Cmp(want) == 0 }})
	}
}

// MembershipCheck constrains the affine point pt to lie in F_i of an affine
// factor base: h = alpha x + beta y + gamma (mod p) and h < Bound_i, plus
// x < XBound_i when the factor bases carry a coordinate window.
func (b *Builder) MembershipCheck(ar *Arith, fb *FactorBases, i int, pt FieldPoint) {
	if !fb.Affine {
		panic("ecdl: MembershipCheck needs affine factor bases")
	}
	u1 := b.FieldVar(ar, "alpha*x")
	b.MulCheck(ar, Operand{C: bi64(fb.Alpha[i])}, Operand{V: pt.X}, u1, true)
	u2 := b.FieldVar(ar, "beta*y")
	b.MulCheck(ar, Operand{C: bi64(fb.Beta[i])}, Operand{V: pt.Y}, u2, true)
	h := b.FieldVar(ar, "hash")
	b.LinCheck(ar, h, true, []Term{{-1, u1}, {-1, u2}}, new(big.Int).Neg(bi64(fb.Gamma[i])))
	b.RangeCheck(ar, h, bi64(fb.Bound[i]), false)
	if fb.XBound != nil {
		// Compact coordinate constraint: x < XBound_i, a block-level compare
		// chain of degree O(t) that reads only x.
		b.RangeCheck(ar, pt.X, bi64(fb.XBound[i]), false)
	}
}

func bi64(x uint64) *big.Int { return new(big.Int).SetUint64(x) }

// CompleteAdd defines P3 = P1 + P2 for every input, including the identity,
// doubling and P + (-P), without branching. Every case's intermediate values
// are pinned down uniquely:
//
//	dx = x2 - x1, dy = y2 - y1, zdx = [dx = 0], zdy = [dy = 0]
//	den = zdx ? (zdy ? 2 y1 : 0) : dx,  num = zdx ? 3 x1^2 + a : dy
//	zden = [den = 0]; lambda * den = (zden ? 0 : num), and zden => lambda = 0
//	(x', y') = chord/tangent formulas from lambda, or (0, 0) if zden
//	P3 = P1 is O ? P2 : P2 is O ? P1 : (x', y'), with
//	inf3 = (inf1 and inf2) or (not inf1 and not inf2 and zden).
//
// zden covers both x1 = x2 cases that give O: y2 = -y1 != y1, and y1 = y2 = 0.
func (b *Builder) CompleteAdd(ar *Arith, c *Curve, p1, p2 FieldPoint) FieldPoint {
	zero := new(big.Int)
	dx := b.FieldVar(ar, "dx")
	b.LinCheck(ar, dx, true, []Term{{-1, p2.X}, {1, p1.X}}, zero)
	dy := b.FieldVar(ar, "dy")
	b.LinCheck(ar, dy, true, []Term{{-1, p2.Y}, {1, p1.Y}}, zero)
	zdx, zdy := b.IsZero(dx), b.IsZero(dy)

	x1sq := b.FieldVar(ar, "x1^2")
	b.MulCheck(ar, Operand{V: p1.X}, Operand{V: p1.X}, x1sq, true)
	numD := b.FieldVar(ar, "3x1^2+a")
	b.LinCheck(ar, numD, true, []Term{{-3, x1sq}}, new(big.Int).Neg(bi64(c.A)))
	denD := b.FieldVar(ar, "2y1")
	b.LinCheck(ar, denD, true, []Term{{-2, p1.Y}}, zero)

	t1 := b.Select(ar, zdy, Operand{V: denD}, Operand{C: zero})
	den := b.Select(ar, zdx, Operand{V: t1}, Operand{V: dx})
	num := b.Select(ar, zdx, Operand{V: numD}, Operand{V: dy})
	zden := b.IsZero(den)
	numSel := b.Select(ar, zden, Operand{C: zero}, Operand{V: num})

	lambda := b.FieldVar(ar, "lambda")
	p := ar.P
	b.Hint([]*Var{den, numSel}, []*Var{lambda}, func(in []*big.Int) [][]*big.Int {
		if in[0].Sign() == 0 {
			return [][]*big.Int{{new(big.Int)}}
		}
		inv := new(big.Int).ModInverse(in[0], p)
		return [][]*big.Int{{inv.Mul(inv, in[1]).Mod(inv, p)}}
	})
	b.RangeCheck(ar, lambda, ar.P, false)
	b.MulCheck(ar, Operand{V: lambda}, Operand{V: den}, numSel, false)
	b.ZeroIf(ar, zden, lambda)

	s := b.FieldVar(ar, "s")
	b.MulCheck(ar, Operand{V: lambda}, Operand{V: lambda}, s, true)
	xr := b.FieldVar(ar, "x'")
	b.LinCheck(ar, xr, true, []Term{{-1, s}, {1, p1.X}, {1, p2.X}}, zero)
	e := b.FieldVar(ar, "e")
	b.LinCheck(ar, e, true, []Term{{-1, p1.X}, {1, xr}}, zero)
	f := b.FieldVar(ar, "f")
	b.MulCheck(ar, Operand{V: lambda}, Operand{V: e}, f, true)
	yr := b.FieldVar(ar, "y'")
	b.LinCheck(ar, yr, true, []Term{{-1, f}, {1, p1.Y}}, zero)

	x3 := b.Select(ar, zden, Operand{C: zero}, Operand{V: xr})
	y3 := b.Select(ar, zden, Operand{C: zero}, Operand{V: yr})
	if p2.Inf != nil {
		x3 = b.Select(ar, p2.Inf, Operand{V: p1.X}, Operand{V: x3})
		y3 = b.Select(ar, p2.Inf, Operand{V: p1.Y}, Operand{V: y3})
	}
	if p1.Inf != nil {
		x3 = b.Select(ar, p1.Inf, Operand{V: p2.X}, Operand{V: x3})
		y3 = b.Select(ar, p1.Inf, Operand{V: p2.Y}, Operand{V: y3})
	}

	// inf3 from the flags; absent flags are constant false.
	inf3 := b.NewBit("inf")
	var flags []*Var
	hasInf := []bool{p1.Inf != nil, p2.Inf != nil}
	for _, fl := range []*Var{p1.Inf, p2.Inf, zden} {
		if fl != nil {
			flags = append(flags, fl)
		}
	}
	rule := func(vals []bool) bool {
		i := 0
		get := func(present bool) bool {
			if !present {
				return false
			}
			v := vals[i]
			i++
			return v
		}
		i1, i2 := get(hasInf[0]), get(hasInf[1])
		z := get(true)
		return (i1 && i2) || (!i1 && !i2 && z)
	}
	b.Hint(flags, []*Var{inf3}, func(in []*big.Int) [][]*big.Int {
		vals := make([]bool, len(in))
		for i, x := range in {
			vals[i] = x.Sign() != 0
		}
		return [][]*big.Int{{boolInt(rule(vals))}}
	})
	var edges []int
	for _, fl := range flags {
		edges = append(edges, b.use(fl)[0]...)
	}
	edges = append(edges, b.use(inf3)[0]...)
	b.add(&PredVertex{E: edges, P: func(vals []bool) bool {
		return rule(vals[:len(vals)-1]) == vals[len(vals)-1]
	}})
	return FieldPoint{X: x3, Y: y3, Inf: inf3}
}

// GadgetRelationNetwork is RelationNetwork built entirely from low-degree
// gadgets, for affine factor bases. Each A_i is proposed by a hint (which
// lists F_i; with a coordinate window that listing is generated from the
// window by square root, not from the whole group) and constrained only by ValidityCheck and
// MembershipCheck; partial sums use CompleteAdd. It has no support vertex,
// and it counts exactly what RelationNetwork counts for the same factor bases.
func GadgetRelationNetwork(c *Curve, ar *Arith, fb *FactorBases, R Point) (*Network, []FieldPoint) {
	b := &Builder{}
	A := make([]FieldPoint, len(fb.Sets))
	for i, set := range fb.Sets {
		A[i] = FieldPoint{X: b.FieldVar(ar, "x"), Y: b.FieldVar(ar, "y")}
		set := set
		if fb.XBound != nil {
			set = fb.WindowCandidates(c, i)
		}
		b.Hint(nil, []*Var{A[i].X, A[i].Y}, func([]*big.Int) [][]*big.Int {
			rows := make([][]*big.Int, len(set))
			for j, pt := range set {
				rows[j] = []*big.Int{bi64(pt.X), bi64(pt.Y)}
			}
			return rows
		})
		b.ValidityCheck(ar, c, A[i])
		b.MembershipCheck(ar, fb, i, A[i])
	}
	S := A[0]
	for k := 1; k < len(A); k++ {
		S = b.CompleteAdd(ar, c, S, A[k])
	}
	b.EqualsConst(ar, S.Inf, boolInt(R.Inf))
	b.EqualsConst(ar, S.X, bi64(R.X))
	b.EqualsConst(ar, S.Y, bi64(R.Y))
	return b.Finalize(), A
}

// GadgetShapes builds, without evaluating, a field multiplication, a
// complete point addition, and the per-point checks of a relation network
// (validity plus membership) for ar's modulus, and returns their statistics.
func GadgetShapes(ar *Arith) (mul, add, point NetworkStats) {
	b := &Builder{}
	x, y, c := b.FieldVar(ar, "x"), b.FieldVar(ar, "y"), b.FieldVar(ar, "c")
	b.Hint(nil, []*Var{x, y}, nil)
	b.RangeCheck(ar, x, ar.P, false)
	b.RangeCheck(ar, y, ar.P, false)
	b.MulCheck(ar, Operand{V: x}, Operand{V: y}, c, true)
	mul = b.Finalize().Stats()

	curve := &Curve{A: 3, B: 7}
	b = &Builder{}
	p1 := FieldPoint{X: b.FieldVar(ar, "x1"), Y: b.FieldVar(ar, "y1"), Inf: b.NewBit("i1")}
	p2 := FieldPoint{X: b.FieldVar(ar, "x2"), Y: b.FieldVar(ar, "y2"), Inf: b.NewBit("i2")}
	b.Hint(nil, []*Var{p1.X, p1.Y, p1.Inf, p2.X, p2.Y, p2.Inf}, nil)
	p3 := b.CompleteAdd(ar, curve, p1, p2)
	b.RangeCheck(ar, p3.X, ar.P, false)
	b.RangeCheck(ar, p3.Y, ar.P, false)
	b.EqualsConst(ar, p3.Inf, new(big.Int))
	add = b.Finalize().Stats()

	fb := &FactorBases{Affine: true, Alpha: []uint64{5}, Beta: []uint64{9}, Gamma: []uint64{1}, Bound: []uint64{1 << 20}}
	b = &Builder{}
	pt := FieldPoint{X: b.FieldVar(ar, "x"), Y: b.FieldVar(ar, "y")}
	b.Hint(nil, []*Var{pt.X, pt.Y}, nil)
	b.ValidityCheck(ar, curve, pt)
	b.MembershipCheck(ar, fb, 0, pt)
	point = b.Finalize().Stats()
	return mul, add, point
}
