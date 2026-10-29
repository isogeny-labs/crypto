package ecdl

import (
	"math/rand"
	"testing"
)

func TestIsPrime(t *testing.T) {
	primes := map[uint64]bool{2: true, 3: true, 4: false, 97: true, 561: false, 65537: true, 1<<31 - 1: true, 3215031751: false}
	for n, want := range primes {
		if IsPrime(n) != want {
			t.Errorf("IsPrime(%d) = %v, want %v", n, !want, want)
		}
	}
}

func TestCurveGroupLaw(t *testing.T) {
	rng := rand.New(rand.NewSource(1))
	for i := 0; i < 5; i++ {
		c, err := RandomPrimeOrderCurve(rng, 12)
		if err != nil {
			t.Fatal(err)
		}
		if !c.OnCurve(c.G) || c.G.Inf {
			t.Fatal("bad generator")
		}
		if got := c.Mul(c.Order, c.G); !got.Inf {
			t.Fatalf("[r]G = %v, want O", got)
		}
		a, b := c.Mul(17, c.G), c.Mul(40, c.G)
		if c.Add(a, b) != c.Mul(57, c.G) || c.Add(a, b) != c.Add(b, a) {
			t.Fatal("addition is not consistent with scalar multiplication")
		}
		if c.Add(a, a) != c.Mul(34, c.G) || !c.Add(a, c.Neg(a)).Inf {
			t.Fatal("doubling or negation is wrong")
		}
		for _, pt := range []Point{Infinity, a, b} {
			got, ok := c.DecodePoint(c.EncodePoint(pt))
			if !ok || got != pt {
				t.Fatalf("encoding round trip failed for %v", pt)
			}
		}
	}
}

// randomNetwork builds a small network mixing all three vertex shapes.
func randomNetwork(rng *rand.Rand) *Network {
	n := &Network{NumEdges: 10}
	pick := func(k int) []int { return rng.Perm(n.NumEdges)[:k] }
	for i := 0; i < 2; i++ {
		e := pick(3)
		var rows [][]bool
		for r := 0; r < 8; r++ {
			if rng.Intn(2) == 0 {
				rows = append(rows, []bool{r&1 == 1, r&2 == 2, r&4 == 4})
			}
		}
		n.Vertices = append(n.Vertices, NewSupportVertex(e, rows))
	}
	// A function whose inputs are already bound by the support vertices.
	bound := append(append([]int{}, n.Vertices[0].Edges()...), n.Vertices[1].Edges()...)
	in := []int{bound[0], bound[4]}
	var out []int
	for _, e := range rng.Perm(n.NumEdges) {
		if e != in[0] && e != in[1] && len(out) < 2 {
			out = append(out, e)
		}
	}
	n.Vertices = append(n.Vertices, &FuncVertex{In: in, Out: out, F: func(x []bool) ([]bool, bool) {
		return []bool{x[0] != x[1], x[0] && x[1]}, !(x[0] && !x[1])
	}})
	pe := []int{bound[1], out[0]}
	n.Vertices = append(n.Vertices, &PredVertex{E: pe, P: func(v []bool) bool { return v[0] || v[1] }})
	return n
}

func TestSparseContractorMatchesBruteForce(t *testing.T) {
	rng := rand.New(rand.NewSource(2))
	for i := 0; i < 200; i++ {
		n := randomNetwork(rng)
		fixed := map[int]bool{}
		for e := 0; e < n.NumEdges; e++ {
			if rng.Intn(4) == 0 {
				fixed[e] = rng.Intn(2) == 0
			}
		}
		want, err := BruteForceCounter{}.Count(n, fixed)
		if err != nil {
			t.Fatal(err)
		}
		got, err := (&SparseContractor{}).Count(n, fixed)
		if err != nil {
			t.Fatal(err)
		}
		if got != want {
			t.Fatalf("network %d: sparse count %d, brute force %d", i, got, want)
		}
	}
}

func TestFindWitness(t *testing.T) {
	rng := rand.New(rand.NewSource(3))
	for i := 0; i < 100; i++ {
		n := randomNetwork(rng)
		all := make([]int, n.NumEdges)
		for e := range all {
			all[e] = e
		}
		w, err := FindWitness(&SparseContractor{}, n, all)
		if err != nil {
			t.Fatal(err)
		}
		total, _ := BruteForceCounter{}.Count(n, nil)
		if (w == nil) != (total == 0) {
			t.Fatalf("witness presence %v but count %d", w != nil, total)
		}
		if w != nil {
			if k, _ := (BruteForceCounter{}).Count(n, w); k != 1 {
				t.Fatalf("witness is not a satisfying assignment")
			}
		}
	}
}

func TestLeftKernelVector(t *testing.T) {
	rng := rand.New(rand.NewSource(4))
	const r = 10007
	for trial := 0; trial < 20; trial++ {
		ncols := 1 + rng.Intn(15)
		rows := make([]map[int]uint64, ncols+1)
		for j := range rows {
			rows[j] = map[int]uint64{}
			for k := 0; k < 3; k++ {
				rows[j][rng.Intn(ncols)] = uint64(rng.Intn(r))
			}
		}
		c, ok := LeftKernelVector(rows, ncols, r)
		if !ok {
			t.Fatal("no dependency found")
		}
		nonzero := false
		sum := make([]uint64, ncols)
		for j, cj := range c {
			nonzero = nonzero || cj != 0
			for col, v := range rows[j] {
				sum[col] = addMod(sum[col], mulMod(cj, v, r), r)
			}
		}
		if !nonzero {
			t.Fatal("zero combination")
		}
		for col, s := range sum {
			if s != 0 {
				t.Fatalf("column %d sums to %d", col, s)
			}
		}
	}
}

func TestRelationNetworkIsGraph(t *testing.T) {
	rng := rand.New(rand.NewSource(5))
	c, err := RandomPrimeOrderCurve(rng, 10)
	if err != nil {
		t.Fatal(err)
	}
	for m := 2; m <= 4; m++ {
		fb := NewFactorBases(c, m, rng)
		net, _ := RelationNetwork(c, fb, c.G)
		st := net.Stats()
		w := c.PointWidth()
		if !st.IsGraph || st.Edges != (2*m-1)*w || st.MaxDegree != 3*w {
			t.Fatalf("m=%d: unexpected network shape %+v", m, st)
		}
	}
}

func TestSolve(t *testing.T) {
	rng := rand.New(rand.NewSource(6))
	cases := []struct{ bits, m, curves int }{{10, 2, 6}, {12, 2, 3}, {10, 3, 3}, {12, 3, 2}}
	if testing.Short() {
		cases = cases[:1]
	}
	for _, tc := range cases {
		for i := 0; i < tc.curves; i++ {
			c, err := RandomPrimeOrderCurve(rng, tc.bits)
			if err != nil {
				t.Fatal(err)
			}
			xi := uint64(rng.Int63n(int64(c.Order)))
			Q := c.Mul(xi, c.G)
			s := &Solver{Curve: c, M: tc.m, Counter: &SparseContractor{}, Rng: rng}
			got, st, err := s.Solve(Q)
			if err != nil {
				t.Fatalf("b=%d m=%d p=%d: %v", tc.bits, tc.m, c.P, err)
			}
			if got != xi {
				t.Fatalf("b=%d m=%d p=%d: got %d, want %d", tc.bits, tc.m, c.P, got, xi)
			}
			t.Logf("b=%d m=%d p=%d r=%d N=%d relations=%d attempts=%d %v",
				tc.bits, tc.m, c.P, c.Order, st.FactorBaseSize, st.Relations, st.Attempts, st.Elapsed)
		}
	}
}
