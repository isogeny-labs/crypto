package ecdl

import (
	"crypto/sha256"
	"encoding/binary"
	"math"
	"math/rand"
)

// PointWidth is the number of bits used to encode a point: an infinity flag
// followed by x and y, each FieldBits wide, least significant bit first.
func (c *Curve) PointWidth() int { return 2*c.FieldBits() + 1 }

// EncodePoint gives the canonical bit encoding of pt (O is the flag alone).
func (c *Curve) EncodePoint(pt Point) []bool {
	b := c.FieldBits()
	v := make([]bool, c.PointWidth())
	if pt.Inf {
		v[0] = true
		return v
	}
	for i := 0; i < b; i++ {
		v[1+i] = pt.X>>uint(i)&1 == 1
		v[1+b+i] = pt.Y>>uint(i)&1 == 1
	}
	return v
}

// DecodePoint inverts EncodePoint. ok is false for non-canonical encodings
// and for points not on the curve: this is the point-validity predicate.
func (c *Curve) DecodePoint(v []bool) (pt Point, ok bool) {
	b := c.FieldBits()
	if v[0] {
		for _, x := range v[1:] {
			if x {
				return Point{}, false
			}
		}
		return Infinity, true
	}
	for i := 0; i < b; i++ {
		if v[1+i] {
			pt.X |= 1 << uint(i)
		}
		if v[1+b+i] {
			pt.Y |= 1 << uint(i)
		}
	}
	return pt, c.OnCurve(pt)
}

// FactorBases holds m randomly hashed factor bases F_1..F_m. Membership in
// F_i is decided by a keyed hash of the point, not by any notion of
// smoothness. Columns for linear algebra are indexed by point, since one
// point can lie in several F_i.
type FactorBases struct {
	Sets   [][]Point
	Points []Point
	Index  map[Point]int
	keys   [][]byte
	mods   []uint64

	// Affine is set for factor bases from NewAffineFactorBases: A is in F_i
	// iff (Alpha_i x + Beta_i y + Gamma_i) mod p < Bound_i.
	Affine                    bool
	Alpha, Beta, Gamma, Bound []uint64
	p                         uint64

	// XBound, if non-nil, adds a compact coordinate constraint: A is in F_i
	// only if x(A) < XBound[i], on top of the hash test. WindowBits is the k
	// with XBound = p >> k. The window is deterministic, so unlike the hash
	// it is not random: it trades some independence between the F_i for
	// membership tests that fix the high bits of x.
	XBound     []uint64
	WindowBits int
}

// NewFactorBases draws m hash keys and sizes each F_i near (2r)^(1/m), so the
// expected number of decompositions of a random target is about 2.
func NewFactorBases(c *Curve, m int, rng *rand.Rand) *FactorBases {
	target := math.Ceil(math.Pow(2*float64(c.Order), 1/float64(m)))
	mod := uint64(math.Max(1, math.Round(float64(c.Order)/target)))
	fb := &FactorBases{Sets: make([][]Point, m), Index: map[Point]int{}}
	for i := 0; i < m; i++ {
		key := make([]byte, 16)
		rng.Read(key)
		fb.keys = append(fb.keys, key)
		fb.mods = append(fb.mods, mod)
	}
	fb.list(c)
	return fb
}

// NewAffineFactorBases draws F_i = {A : (alpha x + beta y + gamma) mod p <
// U} with random alpha, beta, gamma and U chosen so |F_i| is near (2r)^(1/m).
// The hash family is pairwise independent on distinct (x, y) (up to the
// excluded alpha = beta = 0), and unlike
// SHA-256 its membership test is cheap to encode with the low-degree gadgets
// (Builder.MembershipCheck).
func NewAffineFactorBases(c *Curve, m int, rng *rand.Rand) *FactorBases {
	return NewWindowedFactorBases(c, m, rng, 0)
}

// NewWindowedFactorBases is NewAffineFactorBases with the extra constraint
// x < p >> k, so F_i = {A : x(A) < p>>k and hash_i(A) < U_i}. U_i is scaled
// by 2^k to keep |F_i| near (2r)^(1/m). If the window alone would be smaller
// than that, k is lowered until it fits. k = 0 means no window.
func NewWindowedFactorBases(c *Curve, m int, rng *rand.Rand, k int) *FactorBases {
	target := math.Ceil(math.Pow(2*float64(c.Order), 1/float64(m)))
	// The window holds about (r-1) * 2^-k points; it must hold at least target.
	for k > 0 && float64(c.Order-1)/math.Pow(2, float64(k)) < target {
		k--
	}
	xb := c.P >> uint(k)
	if xb == 0 {
		xb = 1
	}
	frac := float64(xb) / float64(c.P)
	bound := uint64(math.Max(1, math.Round(float64(c.P)*target/(float64(c.Order-1)*frac))))
	if bound > c.P {
		bound = c.P
	}
	for {
		fb := &FactorBases{Sets: make([][]Point, m), Index: map[Point]int{}, Affine: true, p: c.P}
		if k > 0 {
			fb.WindowBits = k
			for i := 0; i < m; i++ {
				fb.XBound = append(fb.XBound, xb)
			}
		}
		for i := 0; i < m; i++ {
			a, b := uint64(rng.Int63n(int64(c.P))), uint64(rng.Int63n(int64(c.P)))
			for a == 0 && b == 0 {
				a, b = uint64(rng.Int63n(int64(c.P))), uint64(rng.Int63n(int64(c.P)))
			}
			fb.Alpha = append(fb.Alpha, a)
			fb.Beta = append(fb.Beta, b)
			fb.Gamma = append(fb.Gamma, uint64(rng.Int63n(int64(c.P))))
			fb.Bound = append(fb.Bound, bound)
		}
		fb.list(c)
		empty := false
		for _, set := range fb.Sets {
			empty = empty || len(set) == 0
		}
		if !empty {
			return fb
		}
	}
}

// list fills Sets, Points and Index. With a coordinate window it only visits
// the window; otherwise it enumerates the whole group (toy shortcut; at scale
// F_i is only ever accessed through its membership predicate).
func (fb *FactorBases) list(c *Curve) {
	if fb.XBound != nil {
		fb.listWindow(c)
		return
	}
	fb.listAll(c)
}

func (fb *FactorBases) listAll(c *Curve) {
	m := len(fb.Sets)
	for _, pt := range c.Points() {
		for i := 0; i < m; i++ {
			if fb.Member(i, pt) {
				fb.Sets[i] = append(fb.Sets[i], pt)
				if _, seen := fb.Index[pt]; !seen {
					fb.Index[pt] = len(fb.Points)
					fb.Points = append(fb.Points, pt)
				}
			}
		}
	}
}

// WindowCandidates returns F_i without listing the whole group when the
// factor bases carry a coordinate window: it enumerates the window's points
// by square root (Curve.WindowPoints) and keeps those that pass the hash. Its
// cost is O(XBound_i) per factor base rather than O(|group|), and it never
// touches a point outside the window. Without a window it returns a copy of
// Sets[i], since there is no compact range to enumerate.
func (fb *FactorBases) WindowCandidates(c *Curve, i int) []Point {
	if fb.XBound == nil {
		return append([]Point(nil), fb.Sets[i]...)
	}
	out := make([]Point, 0, len(fb.Sets[i]))
	for _, pt := range c.WindowPoints(fb.XBound[i]) {
		if fb.Member(i, pt) {
			out = append(out, pt)
		}
	}
	return out
}

// listWindow is listAll restricted to the union of the coordinate windows:
// it takes square roots of x^3 + a x + b for x below the largest XBound,
// then applies each F_i's membership predicate. Cost is O(max XBound).
func (fb *FactorBases) listWindow(c *Curve) {
	xmax := uint64(0)
	for _, xb := range fb.XBound {
		if xb > xmax {
			xmax = xb
		}
	}
	for _, pt := range c.WindowPoints(xmax) {
		for i := range fb.Sets {
			if fb.Member(i, pt) {
				fb.Sets[i] = append(fb.Sets[i], pt)
				if _, seen := fb.Index[pt]; !seen {
					fb.Index[pt] = len(fb.Points)
					fb.Points = append(fb.Points, pt)
				}
			}
		}
	}
}

// AffineHash returns (Alpha_i x + Beta_i y + Gamma_i) mod p.
func (fb *FactorBases) AffineHash(i int, pt Point) uint64 {
	p := fb.p
	h := addMod(mulMod(fb.Alpha[i], pt.X, p), mulMod(fb.Beta[i], pt.Y, p), p)
	return addMod(h, fb.Gamma[i], p)
}

// Member is the membership predicate of F_i. O is never a member.
func (fb *FactorBases) Member(i int, pt Point) bool {
	if pt.Inf {
		return false
	}
	if fb.XBound != nil && pt.X >= fb.XBound[i] {
		return false
	}
	if fb.Affine {
		return fb.AffineHash(i, pt) < fb.Bound[i]
	}
	var buf [16]byte
	binary.LittleEndian.PutUint64(buf[:8], pt.X)
	binary.LittleEndian.PutUint64(buf[8:], pt.Y)
	h := sha256.New()
	h.Write(fb.keys[i])
	h.Write(buf[:])
	return binary.LittleEndian.Uint64(h.Sum(nil)[:8])%fb.mods[i] == 0
}

// RelationNetwork encodes the search for A_1 + ... + A_m = R with A_i in F_i
// as a Boolean tensor network. Edges are the bits of A_1..A_m and of the
// partial sums S_2..S_m (S_1 = A_1). Vertices are:
//   - membership(i): A_i is a valid point in F_i (a sparse support tensor);
//   - add(k): S_k = S_{k-1} + A_k on valid points (a function tensor);
//   - target: S_m = R.
//
// Every edge has exactly two endpoints, so this is a graph. At toy scale each
// vertex reads whole points (degree 3w for add, where w = 2b+1); the paper's
// construction splits these into gadgets of degree O(b^(3/4)).
// It returns the network and the edge indices of each A_i.
func RelationNetwork(c *Curve, fb *FactorBases, R Point) (*Network, [][]int) {
	w := c.PointWidth()
	m := len(fb.Sets)
	next := 0
	block := func() []int {
		e := make([]int, w)
		for i := range e {
			e[i] = next
			next++
		}
		return e
	}
	A := make([][]int, m)
	for i := range A {
		A[i] = block()
	}
	n := &Network{}
	member := func(i int) Vertex {
		rows := make([][]bool, len(fb.Sets[i]))
		for j, pt := range fb.Sets[i] {
			rows[j] = c.EncodePoint(pt)
		}
		return NewSupportVertex(A[i], rows)
	}
	n.Vertices = append(n.Vertices, member(0))
	S := A[0]
	for k := 1; k < m; k++ {
		n.Vertices = append(n.Vertices, member(k))
		out := block()
		n.Vertices = append(n.Vertices, &FuncVertex{
			In:  append(append([]int{}, S...), A[k]...),
			Out: out,
			F: func(in []bool) ([]bool, bool) {
				s, ok1 := c.DecodePoint(in[:w])
				a, ok2 := c.DecodePoint(in[w:])
				if !ok1 || !ok2 {
					return nil, false
				}
				return c.EncodePoint(c.Add(s, a)), true
			},
		})
		S = out
	}
	want := bitsKey(c.EncodePoint(R))
	n.Vertices = append(n.Vertices, &PredVertex{
		E: S,
		P: func(vals []bool) bool { return bitsKey(vals) == want },
	})
	n.NumEdges = next
	return n, A
}
