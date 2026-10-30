package ecdl

import (
	"errors"
	"fmt"
	"math/big"
	"math/rand"
	"time"
)

// Relation records [U]P + [V]Q = A_1 + ... + A_m.
type Relation struct {
	U, V   uint64
	Points []Point
}

// Stats describes one Solve call.
type Stats struct {
	FactorBaseSize int // N, the number of distinct factor-base points
	Relations      int
	Attempts       int // targets tried, including ones with no decomposition
	Restarts       int // fresh factor bases after V = 0 or a failed check
	Network        NetworkStats
	Elapsed        time.Duration
}

// Solver runs index calculus with m randomly hashed factor bases, finding
// relations with a Boolean tensor contraction Counter.
type Solver struct {
	Curve   *Curve
	M       int
	Counter Counter
	Rng     *rand.Rand
	// MaxRestarts bounds the number of fresh factor-base draws (default 8).
	MaxRestarts int
	// Gadgets uses affine factor bases and builds relation networks from the
	// low-degree arithmetic gadgets (GadgetRelationNetwork) instead of
	// whole-point vertices. The Counter must then support hinted schedules,
	// e.g. DFSCounter.
	Gadgets bool
	// WindowBits > 0 (with Gadgets) adds the coordinate window x < p>>k to
	// every factor base; see NewWindowedFactorBases.
	WindowBits int
	// Logf, if set, receives progress messages.
	Logf func(format string, args ...interface{})

	arith *Arith
}

// Solve returns xi with Q = [xi]P, where P is the curve's generator. It only
// returns a value it has checked, so an answer is never wrong.
func (s *Solver) Solve(Q Point) (uint64, Stats, error) {
	start := time.Now()
	var st Stats
	c := s.Curve
	if s.M < 2 {
		return 0, st, errors.New("ecdl: need m >= 2 factor bases")
	}
	if !c.OnCurve(Q) {
		return 0, st, errors.New("ecdl: Q is not on the curve")
	}
	if Q.Inf {
		return 0, st, nil
	}
	maxRestarts := s.MaxRestarts
	if maxRestarts == 0 {
		maxRestarts = 8
	}
	r := c.Order
	for restart := 0; restart < maxRestarts; restart++ {
		st.Restarts = restart

		// Step 1: factor-base selection.
		var fb *FactorBases
		if s.Gadgets {
			fb = NewWindowedFactorBases(c, s.M, s.Rng, s.WindowBits)
		} else {
			fb = NewFactorBases(c, s.M, s.Rng)
		}
		st.FactorBaseSize = len(fb.Points)
		need := len(fb.Points) + 1
		s.logf("restart %d: |F_i| = %v, N = %d, collecting %d relations", restart, setSizes(fb), len(fb.Points), need)

		// Step 2: relation finding by tensor contraction.
		rels := make([]Relation, 0, need)
		for len(rels) < need {
			st.Attempts++
			u := uint64(s.Rng.Int63n(int64(r)))
			v := uint64(s.Rng.Int63n(int64(r)))
			R := c.Add(c.Mul(u, c.G), c.Mul(v, Q))
			rel, ns, err := s.findRelation(fb, R)
			st.Network = ns
			if err != nil {
				return 0, st, err
			}
			if rel == nil {
				continue
			}
			rel.U, rel.V = u, v
			rels = append(rels, *rel)
			if len(rels)%50 == 0 {
				s.logf("  %d/%d relations after %d attempts", len(rels), need, st.Attempts)
			}
		}
		st.Relations += len(rels)

		// Step 3: linear algebra over F_r.
		rows := make([]map[int]uint64, len(rels))
		for j, rel := range rels {
			rows[j] = map[int]uint64{}
			for _, pt := range rel.Points {
				col := fb.Index[pt]
				rows[j][col] = (rows[j][col] + 1) % r
			}
		}
		coef, ok := LeftKernelVector(rows, len(fb.Points), r)
		if !ok {
			return 0, st, errors.New("ecdl: linear algebra found no dependency")
		}
		var U, V uint64
		for j, cj := range coef {
			U = addMod(U, mulMod(cj, rels[j].U, r), r)
			V = addMod(V, mulMod(cj, rels[j].V, r), r)
		}

		// Step 4: [U]P + [V]Q = O, so xi = -U/V.
		if V == 0 {
			s.logf("V = 0, restarting")
			continue
		}
		xi := subMod(0, mulMod(U, invMod(V, r), r), r)
		if c.Mul(xi, c.G) == Q {
			st.Elapsed = time.Since(start)
			return xi, st, nil
		}
		s.logf("candidate failed verification, restarting")
	}
	st.Elapsed = time.Since(start)
	return 0, st, fmt.Errorf("ecdl: no verified answer after %d restarts", maxRestarts)
}

// findRelation looks for A_i in F_i with A_1 + ... + A_m = R, returning nil
// if the contraction counts no solutions.
func (s *Solver) findRelation(fb *FactorBases, R Point) (*Relation, NetworkStats, error) {
	c := s.Curve
	var net *Network
	var A [][]int // edges of each A_i
	var decode func(vals []bool) (Point, bool)
	if s.Gadgets {
		if s.arith == nil {
			s.arith = NewArith(new(big.Int).SetUint64(c.P))
		}
		var pts []FieldPoint
		net, pts = GadgetRelationNetwork(c, s.arith, fb, R)
		for _, pt := range pts {
			A = append(A, concat(pt.X.Edges(), pt.Y.Edges()))
		}
		b := s.arith.Bits
		decode = func(vals []bool) (Point, bool) {
			pt := Point{X: bitsUint(vals[:b]).Uint64(), Y: bitsUint(vals[b:]).Uint64()}
			return pt, c.OnCurve(pt)
		}
	} else {
		net, A = RelationNetwork(c, fb, R)
		decode = c.DecodePoint
	}
	ns := net.Stats()
	var edges []int
	for _, blk := range A {
		edges = append(edges, blk...)
	}
	fixed, err := FindWitness(s.Counter, net, edges)
	if err != nil || fixed == nil {
		return nil, ns, err
	}
	rel := &Relation{}
	sum := Infinity
	for i, blk := range A {
		vals := make([]bool, len(blk))
		for j, e := range blk {
			vals[j] = fixed[e]
		}
		pt, ok := decode(vals)
		if !ok || !fb.Member(i, pt) {
			return nil, ns, errors.New("ecdl: counter returned an invalid witness")
		}
		rel.Points = append(rel.Points, pt)
		sum = c.Add(sum, pt)
	}
	if sum != R {
		return nil, ns, errors.New("ecdl: counter returned a witness that does not sum to the target")
	}
	return rel, ns, nil
}

func (s *Solver) logf(format string, args ...interface{}) {
	if s.Logf != nil {
		s.Logf(format, args...)
	}
}

func setSizes(fb *FactorBases) []int {
	out := make([]int, len(fb.Sets))
	for i, set := range fb.Sets {
		out[i] = len(set)
	}
	return out
}
