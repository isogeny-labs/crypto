package ecdl

import (
	"fmt"
	"math"
	"math/big"
	"math/rand"
	"testing"
)

func bi(x uint64) *big.Int { return new(big.Int).SetUint64(x) }

// countWith counts a builder's network, optionally fixing a Var to a value.
func countWith(t *testing.T, n *Network, v *Var, val *big.Int) uint64 {
	t.Helper()
	var fixed map[int]bool
	if v != nil {
		bs, ok := uintBits(val, v.Width())
		if !ok {
			t.Fatal("value does not fit")
		}
		fixed = map[int]bool{}
		for i, e := range v.Edges() {
			fixed[e] = bs[i]
		}
	}
	k, err := DFSCounter{}.Count(n, fixed)
	if err != nil {
		t.Fatal(err)
	}
	return k
}

// In exhaustive mode every hint enumerates all values of the Vars it
// writes, so a count of exactly 1 shows the vertices alone pin down every
// witness, and fixing the output shows that witness is the right one.
func TestMulCheckExhaustive(t *testing.T) {
	for _, p := range []uint64{11, 13} {
		ar := NewArith(bi(p))
		for a := uint64(0); a < p; a++ {
			for bv := uint64(0); bv < p; bv++ {
				for _, constA := range []bool{false, true} {
					b := &Builder{Exhaustive: true}
					x, y, c := b.FieldVar(ar, "a"), b.FieldVar(ar, "b"), b.FieldVar(ar, "c")
					xo := Operand{V: x}
					if constA {
						xo = Operand{C: bi(a)}
						b.Support([]*Var{y}, [][]*big.Int{{bi(bv)}})
					} else {
						b.Support([]*Var{x, y}, [][]*big.Int{{bi(a), bi(bv)}})
					}
					b.MulCheck(ar, xo, Operand{V: y}, c, true)
					n := b.Finalize()
					if k := countWith(t, n, nil, nil); k != 1 {
						t.Fatalf("p=%d a=%d b=%d const=%v: %d assignments, want 1", p, a, bv, constA, k)
					}
					if k := countWith(t, n, c, bi(a*bv%p)); k != 1 {
						t.Fatalf("p=%d a=%d b=%d: product is not a*b mod p", p, a, bv)
					}
				}
			}
		}
	}
}

// At b = 7 the transform has L1 != L2 and a stage-1 vertex with no inputs,
// paths the 4-bit sweep above does not reach.
func TestMulCheckExhaustiveUnbalanced(t *testing.T) {
	const p = 101
	ar := NewArith(bi(p))
	if ar.L1 == ar.L2 || ar.N > ar.L2*(ar.L1-1) {
		t.Fatalf("parameters do not exercise the unbalanced path: %+v", ar)
	}
	rng := rand.New(rand.NewSource(16))
	for i := 0; i < 5; i++ {
		a, bv := uint64(rng.Intn(p)), uint64(rng.Intn(p))
		b := &Builder{Exhaustive: true}
		x, y, c := b.FieldVar(ar, "a"), b.FieldVar(ar, "b"), b.FieldVar(ar, "c")
		b.Support([]*Var{x, y}, [][]*big.Int{{bi(a), bi(bv)}})
		b.MulCheck(ar, Operand{V: x}, Operand{V: y}, c, true)
		n := b.Finalize()
		if k := countWith(t, n, nil, nil); k != 1 {
			t.Fatalf("a=%d b=%d: %d assignments, want 1", a, bv, k)
		}
		if countWith(t, n, c, bi(a*bv%p)) != 1 {
			t.Fatalf("a=%d b=%d: product is not a*b mod p", a, bv)
		}
	}
}

func TestLinAndRangeCheckExhaustive(t *testing.T) {
	const p = 13
	ar := NewArith(bi(p))
	for x := uint64(0); x < p; x++ {
		for y := uint64(0); y < p; y++ {
			b := &Builder{Exhaustive: true}
			xv, yv, out := b.FieldVar(ar, "x"), b.FieldVar(ar, "y"), b.FieldVar(ar, "out")
			b.Support([]*Var{xv, yv}, [][]*big.Int{{bi(x), bi(y)}})
			// out = -(2x - y + 5) mod p, and out must be nonzero.
			b.LinCheck(ar, out, true, []Term{{2, xv}, {-1, yv}}, big.NewInt(5))
			b.RangeCheck(ar, out, ar.P, true)
			n := b.Finalize()
			want := ((-(2*int64(x) - int64(y) + 5))%p + p) % p
			wantCount := uint64(1)
			if want == 0 {
				wantCount = 0
			}
			if k := countWith(t, n, nil, nil); k != wantCount {
				t.Fatalf("x=%d y=%d: %d assignments, want %d", x, y, k, wantCount)
			}
			if wantCount == 1 && countWith(t, n, out, big.NewInt(want)) != 1 {
				t.Fatalf("x=%d y=%d: wrong output", x, y)
			}
		}
	}
}

func TestAddGenericExhaustive(t *testing.T) {
	rng := rand.New(rand.NewSource(11))
	c, err := RandomPrimeOrderCurve(rng, 4)
	if err != nil {
		t.Fatal(err)
	}
	ar := NewArith(bi(c.P))
	pts := c.Points()
	for _, p1 := range pts {
		for _, p2 := range pts {
			// Exhaustive on every same-x pair and a sample of the rest;
			// the others still check the sum with computed witnesses.
			b := &Builder{Exhaustive: p1.X == p2.X || rng.Intn(6) == 0}
			P1 := FieldPoint{X: b.FieldVar(ar, "x1"), Y: b.FieldVar(ar, "y1")}
			P2 := FieldPoint{X: b.FieldVar(ar, "x2"), Y: b.FieldVar(ar, "y2")}
			b.Support([]*Var{P1.X, P1.Y}, [][]*big.Int{{bi(p1.X), bi(p1.Y)}})
			b.Support([]*Var{P2.X, P2.Y}, [][]*big.Int{{bi(p2.X), bi(p2.Y)}})
			P3 := b.AddGeneric(ar, P1, P2)
			n := b.Finalize()
			generic := p1.X != p2.X
			k := countWith(t, n, nil, nil)
			if (generic && k != 1) || (!generic && k != 0) {
				t.Fatalf("%v + %v: %d assignments (generic=%v)", p1, p2, k, generic)
			}
			if generic {
				s := c.Add(p1, p2)
				if countWith(t, n, P3.X, bi(s.X)) != 1 || countWith(t, n, P3.Y, bi(s.Y)) != 1 {
					t.Fatalf("%v + %v: wrong sum", p1, p2)
				}
			}
		}
	}
}

func TestValidityCheck(t *testing.T) {
	rng := rand.New(rand.NewSource(12))
	for _, bitsz := range []int{4, 6} {
		c, err := RandomPrimeOrderCurve(rng, bitsz)
		if err != nil {
			t.Fatal(err)
		}
		ar := NewArith(bi(c.P))
		b := &Builder{Exhaustive: bitsz == 4}
		pt := FieldPoint{X: b.FieldVar(ar, "x"), Y: b.FieldVar(ar, "y")}
		// Every pair of b-bit values, including non-canonical ones >= p.
		var rows [][]*big.Int
		lim := uint64(1) << uint(ar.Bits)
		for x := uint64(0); x < lim; x++ {
			for y := uint64(0); y < lim; y++ {
				rows = append(rows, []*big.Int{bi(x), bi(y)})
			}
		}
		b.Support([]*Var{pt.X, pt.Y}, rows)
		b.ValidityCheck(ar, c, pt)
		if k := countWith(t, b.Finalize(), nil, nil); k != c.Order-1 {
			t.Fatalf("b=%d: %d valid encodings, want %d affine points", bitsz, k, c.Order-1)
		}
	}
}

// pointVars defines a FieldPoint (with an infinity flag) holding pt.
func pointVars(b *Builder, ar *Arith, pt Point) FieldPoint {
	fp := FieldPoint{X: b.FieldVar(ar, "x"), Y: b.FieldVar(ar, "y"), Inf: b.NewBit("inf")}
	b.Support([]*Var{fp.X, fp.Y, fp.Inf}, [][]*big.Int{{bi(pt.X), bi(pt.Y), boolInt(pt.Inf)}})
	return fp
}

func TestCompleteAddExhaustive(t *testing.T) {
	rng := rand.New(rand.NewSource(17))
	c, err := RandomPrimeOrderCurve(rng, 4)
	if err != nil {
		t.Fatal(err)
	}
	ar := NewArith(bi(c.P))
	pts := append(c.Points(), Infinity)
	special := 0
	for _, p1 := range pts {
		for _, p2 := range pts {
			// Exhaustive on every pair with O or equal x (doubling, P + -P)
			// and a sample of generic pairs. y = 0 cannot occur on a
			// prime-order curve; see TestCompleteAddTwoTorsion.
			sp := p1.Inf || p2.Inf || p1.X == p2.X
			if sp {
				special++
			}
			b := &Builder{Exhaustive: sp || rng.Intn(8) == 0}
			P3 := b.CompleteAdd(ar, c, pointVars(b, ar, p1), pointVars(b, ar, p2))
			n := b.Finalize()
			if k := countWith(t, n, nil, nil); k != 1 {
				t.Fatalf("%v + %v: %d assignments, want 1", p1, p2, k)
			}
			want := c.Add(p1, p2)
			if countWith(t, n, P3.Inf, boolInt(want.Inf)) != 1 ||
				countWith(t, n, P3.X, bi(want.X)) != 1 || countWith(t, n, P3.Y, bi(want.Y)) != 1 {
				t.Fatalf("%v + %v: wrong sum, want %v", p1, p2, want)
			}
		}
	}
	t.Logf("p=%d: %d pairs, %d special", c.P, len(pts)*len(pts), special)
}

// Prime-order curves have no points with y = 0 (they would have order 2), so
// the y1 = y2 = 0 branch of CompleteAdd needs a curve with 2-torsion:
// y^2 = x^3 + x over F_13 has three such points.
func TestCompleteAddTwoTorsion(t *testing.T) {
	c := &Curve{P: 13, A: 1, B: 0}
	ar := NewArith(bi(c.P))
	var twoTors []Point
	for _, pt := range c.Points() {
		if pt.Y == 0 {
			twoTors = append(twoTors, pt)
		}
	}
	if len(twoTors) == 0 {
		t.Fatal("test curve has no points with y = 0")
	}
	pts := append(c.Points(), Infinity)
	pairs := 0
	for _, p1 := range pts {
		for _, p2 := range pts {
			if p1.Inf || p2.Inf || p1.Y != 0 && p2.Y != 0 {
				continue
			}
			pairs++
			b := &Builder{Exhaustive: true}
			P3 := b.CompleteAdd(ar, c, pointVars(b, ar, p1), pointVars(b, ar, p2))
			n := b.Finalize()
			if k := countWith(t, n, nil, nil); k != 1 {
				t.Fatalf("%v + %v: %d assignments, want 1", p1, p2, k)
			}
			want := c.Add(p1, p2)
			if countWith(t, n, P3.Inf, boolInt(want.Inf)) != 1 ||
				countWith(t, n, P3.X, bi(want.X)) != 1 || countWith(t, n, P3.Y, bi(want.Y)) != 1 {
				t.Fatalf("%v + %v: wrong sum, want %v", p1, p2, want)
			}
		}
	}
	t.Logf("%d points with y = 0, %d pairs checked", len(twoTors), pairs)
}

// Exhaustive over every (x, y) encoding: validity plus membership must
// accept exactly the points of F_i.
func TestMembershipExhaustive(t *testing.T) {
	rng := rand.New(rand.NewSource(18))
	c, err := RandomPrimeOrderCurve(rng, 4)
	if err != nil {
		t.Fatal(err)
	}
	ar := NewArith(bi(c.P))
	fb := NewAffineFactorBases(c, 2, rng)
	for i := range fb.Sets {
		b := &Builder{Exhaustive: true}
		pt := FieldPoint{X: b.FieldVar(ar, "x"), Y: b.FieldVar(ar, "y")}
		b.Hint(nil, []*Var{pt.X, pt.Y}, nil)
		b.ValidityCheck(ar, c, pt)
		b.MembershipCheck(ar, fb, i, pt)
		if k := countWith(t, b.Finalize(), nil, nil); k != uint64(len(fb.Sets[i])) {
			t.Fatalf("F_%d: gadget accepts %d encodings, |F_i| = %d", i, k, len(fb.Sets[i]))
		}
	}
}

// specialTargets enumerates all decompositions and returns the targets that
// some decomposition reaches through a non-generic addition (an O input or
// output, or equal x), plus whether O itself is reachable.
func specialTargets(c *Curve, fb *FactorBases) (special []Point) {
	seen := map[Point]bool{}
	var walk func(i int, s Point, sp bool)
	walk = func(i int, s Point, sp bool) {
		if i == len(fb.Sets) {
			if sp && !seen[s] {
				seen[s] = true
				special = append(special, s)
			}
			return
		}
		for _, a := range fb.Sets[i] {
			n := c.Add(s, a)
			walk(i+1, n, sp || (i > 0 && (s.Inf || s.X == a.X)))
		}
	}
	walk(0, Infinity, false)
	return special
}

// The gadget network must count exactly what the whole-point network counts
// for the same factor bases, including decompositions through doubling,
// P + (-P) and O.
func TestGadgetRelationCounts(t *testing.T) {
	rng := rand.New(rand.NewSource(13))
	c, err := RandomPrimeOrderCurve(rng, 7)
	if err != nil {
		t.Fatal(err)
	}
	ar := NewArith(bi(c.P))
	for _, m := range []int{2, 3} {
		var fb *FactorBases
		var special []Point
		for tries := 0; len(special) == 0; tries++ {
			if tries == 50 {
				t.Fatalf("m=%d: no factor bases with special-case decompositions", m)
			}
			fb = NewAffineFactorBases(c, m, rng)
			special = specialTargets(c, fb)
		}
		targets := special
		if len(targets) > 4 {
			targets = targets[:4]
		}
		targets = append(targets, Infinity)
		for i := 0; i < 3; i++ {
			targets = append(targets, c.Mul(uint64(1+rng.Int63n(int64(c.Order-1))), c.G))
		}
		hitSpecial := 0
		for ti, R := range targets {
			want, err := (&SparseContractor{}).Count(func() *Network { n, _ := RelationNetwork(c, fb, R); return n }(), nil)
			if err != nil {
				t.Fatal(err)
			}
			n, _ := GadgetRelationNetwork(c, ar, fb, R)
			st := n.Stats()
			if !st.IsGraph {
				t.Fatal("gadget relation network is not a graph")
			}
			for _, v := range n.Vertices {
				if _, ok := v.(*SupportVertex); ok {
					t.Fatal("gadget relation network has a support vertex")
				}
			}
			got, err := DFSCounter{}.Count(n, nil)
			if err != nil {
				t.Fatal(err)
			}
			if got != want {
				t.Fatalf("m=%d R=%v: gadget network counts %d, whole-point network %d", m, R, got, want)
			}
			if ti < len(special) && ti < 4 && want > 0 {
				hitSpecial++
			}
		}
		if hitSpecial == 0 {
			t.Fatalf("m=%d: no special-case target had a decomposition", m)
		}
		t.Logf("m=%d: %d targets agree, %d reached through special cases", m, len(targets), hitSpecial)
	}
}

func TestDFSCounterMatchesSparse(t *testing.T) {
	rng := rand.New(rand.NewSource(14))
	for i := 0; i < 100; i++ {
		n := randomNetwork(rng)
		a, _ := (&SparseContractor{}).Count(n, nil)
		b, err := DFSCounter{}.Count(n, nil)
		if err != nil {
			t.Fatal(err)
		}
		if a != b {
			t.Fatalf("network %d: sparse %d, dfs %d", i, a, b)
		}
	}
}

func TestSolveWithGadgets(t *testing.T) {
	rng := rand.New(rand.NewSource(15))
	for i := 0; i < 2; i++ {
		c, err := RandomPrimeOrderCurve(rng, 6)
		if err != nil {
			t.Fatal(err)
		}
		xi := uint64(rng.Int63n(int64(c.Order)))
		s := &Solver{Curve: c, M: 2, Counter: DFSCounter{}, Rng: rng, Gadgets: true}
		got, st, err := s.Solve(c.Mul(xi, c.G))
		if err != nil {
			t.Fatal(err)
		}
		if got != xi {
			t.Fatalf("got %d, want %d", got, xi)
		}
		t.Logf("p=%d r=%d network: %+v relations=%d %v", c.P, c.Order, st.Network, st.Relations, st.Elapsed)
	}
}

func TestGadgetScaling(t *testing.T) {
	sizes := []int{64, 256, 1024, 4096}
	if testing.Short() {
		sizes = sizes[:3]
	}
	for _, bitsz := range sizes {
		p := new(big.Int).Lsh(big.NewInt(1), uint(bitsz-1))
		mul, add, point := GadgetShapes(NewArith(p.Add(p, big.NewInt(1))))
		for name, st := range map[string]NetworkStats{"mul": mul, "add": add, "point": point} {
			if !st.IsGraph {
				t.Fatalf("b=%d %s: not a graph", bitsz, name)
			}
			perB := float64(st.Edges) / float64(bitsz)
			degRatio := float64(st.MaxDegree) / math.Pow(float64(bitsz), 0.75)
			t.Logf("b=%5d %s: edges=%7d edges/b=%5.1f maxdeg=%5d maxdeg/b^0.75=%4.1f", bitsz, name, st.Edges, perB, st.MaxDegree, degRatio)
			if perB > 400 || degRatio > 40 {
				t.Fatalf("b=%d %s: shape out of bounds", bitsz, name)
			}
		}
	}
}

// The coordinate window composes with the hash: the gadget must accept
// exactly the points with x < XBound that pass the hash test, and the window
// must shrink F_i without changing its target size.
func TestWindowedMembershipExhaustive(t *testing.T) {
	rng := rand.New(rand.NewSource(19))
	var c *Curve
	var fb *FactorBases
	for tries := 0; fb == nil || fb.XBound == nil; tries++ {
		if tries == 50 {
			t.Fatal("window was always dropped")
		}
		var err error
		if c, err = RandomPrimeOrderCurve(rng, 4); err != nil {
			t.Fatal(err)
		}
		fb = NewWindowedFactorBases(c, 2, rng, 1)
	}
	ar := NewArith(bi(c.P))
	for i := range fb.Sets {
		for _, pt := range fb.Sets[i] {
			if pt.X >= fb.XBound[i] {
				t.Fatalf("F_%d contains %v outside the window", i, pt)
			}
		}
		b := &Builder{Exhaustive: true}
		pt := FieldPoint{X: b.FieldVar(ar, "x"), Y: b.FieldVar(ar, "y")}
		b.Hint(nil, []*Var{pt.X, pt.Y}, nil)
		b.ValidityCheck(ar, c, pt)
		b.MembershipCheck(ar, fb, i, pt)
		if k := countWith(t, b.Finalize(), nil, nil); k != uint64(len(fb.Sets[i])) {
			t.Fatalf("F_%d: gadget accepts %d encodings, |F_i| = %d", i, k, len(fb.Sets[i]))
		}
	}
}

// sqrtMod must agree with a brute-force square table on every residue.
func TestSqrtMod(t *testing.T) {
	for _, p := range []uint64{11, 13, 17, 29, 97, 101, 241} {
		sq := map[uint64]bool{}
		for y := uint64(0); y < p; y++ {
			sq[mulMod(y, y, p)] = true
		}
		for a := uint64(0); a < p; a++ {
			y, ok := sqrtMod(a, p)
			if ok != sq[a] {
				t.Fatalf("p=%d a=%d: ok=%v want %v", p, a, ok, sq[a])
			}
			if ok && mulMod(y, y, p) != a {
				t.Fatalf("p=%d a=%d: %d^2 != a", p, a, y)
			}
		}
	}
}

// Generating F_i from the coordinate window must yield exactly the points the
// membership predicate accepts.
func TestWindowCandidates(t *testing.T) {
	rng := rand.New(rand.NewSource(29))
	c, err := RandomPrimeOrderCurve(rng, 7)
	if err != nil {
		t.Fatal(err)
	}
	fb := NewWindowedFactorBases(c, 3, rng, 1)
	if fb.XBound == nil {
		t.Fatal("window was dropped")
	}
	for i := range fb.Sets {
		got := fb.WindowCandidates(c, i)
		want := map[Point]bool{}
		for _, pt := range fb.Sets[i] {
			want[pt] = true
		}
		if len(got) != len(want) {
			t.Fatalf("F_%d: window gives %d points, predicate has %d", i, len(got), len(want))
		}
		seen := map[Point]bool{}
		for _, pt := range got {
			if !want[pt] || seen[pt] {
				t.Fatalf("F_%d: window point %v not in F_i or duplicated", i, pt)
			}
			seen[pt] = true
		}
	}
}

// Building the sets from the window must give the same sets, points and
// column index as enumerating the whole group.
func TestWindowedListMatchesFull(t *testing.T) {
	rng := rand.New(rand.NewSource(31))
	c, err := RandomPrimeOrderCurve(rng, 7)
	if err != nil {
		t.Fatal(err)
	}
	fb := NewWindowedFactorBases(c, 3, rng, 1)
	if fb.XBound == nil {
		t.Fatal("window was dropped")
	}
	full := *fb
	full.Sets = make([][]Point, len(fb.Sets))
	full.Points = nil
	full.Index = map[Point]int{}
	full.listAll(c)
	for i := range fb.Sets {
		if len(fb.Sets[i]) != len(full.Sets[i]) {
			t.Fatalf("F_%d: windowed list has %d points, full list %d", i, len(fb.Sets[i]), len(full.Sets[i]))
		}
		for j := range fb.Sets[i] {
			if fb.Sets[i][j] != full.Sets[i][j] {
				t.Fatalf("F_%d: point %d differs", i, j)
			}
		}
	}
	if len(fb.Points) != len(full.Points) || len(fb.Index) != len(full.Index) {
		t.Fatalf("column counts differ: windowed %d/%d, full %d/%d", len(fb.Points), len(fb.Index), len(full.Points), len(full.Index))
	}
	for pt, col := range full.Index {
		if fb.Index[pt] != col {
			t.Fatalf("column of %v differs: %d vs %d", pt, fb.Index[pt], col)
		}
	}
}

// Windowed gadget networks count what the whole-point network counts.
func TestWindowedRelationCounts(t *testing.T) {
	rng := rand.New(rand.NewSource(23))
	c, err := RandomPrimeOrderCurve(rng, 7)
	if err != nil {
		t.Fatal(err)
	}
	ar := NewArith(bi(c.P))
	fb := NewWindowedFactorBases(c, 2, rng, 1)
	if fb.XBound == nil {
		t.Fatal("window was dropped")
	}
	for ti := 0; ti < 4; ti++ {
		R := c.Mul(uint64(1+rng.Int63n(int64(c.Order-1))), c.G)
		want, err := (&SparseContractor{}).Count(func() *Network { n, _ := RelationNetwork(c, fb, R); return n }(), nil)
		if err != nil {
			t.Fatal(err)
		}
		n, _ := GadgetRelationNetwork(c, ar, fb, R)
		got, err := (DFSCounter{}).Count(n, nil)
		if err != nil {
			t.Fatal(err)
		}
		if got != want {
			t.Fatalf("target %d: gadget count %d, whole-point count %d", ti, got, want)
		}
	}
}

// Building m = 2 factor bases for b = 16: whole-group enumeration vs the
// coordinate window at k = 1, 2, 4.
func BenchmarkFactorBaseBuild(b *testing.B) {
	rng := rand.New(rand.NewSource(37))
	c, err := RandomPrimeOrderCurve(rng, 16)
	if err != nil {
		b.Fatal(err)
	}
	for _, k := range []int{0, 1, 2, 4} {
		k := k
		b.Run(fmt.Sprintf("k=%d", k), func(b *testing.B) {
			for i := 0; i < b.N; i++ {
				NewWindowedFactorBases(c, 2, rng, k)
			}
		})
	}
}
