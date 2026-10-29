package ecdl

import (
	"errors"
	"fmt"
)

// A Boolean tensor network: edges carry bits, vertices carry predicates on the
// bits of their incident edges. Contracting the network means counting the
// assignments of bits to edges that satisfy every vertex.
//
// Vertices come in three shapes, all of which are 0/1 tensors:
//   - SupportVertex lists the satisfying rows explicitly (a sparse tensor);
//   - FuncVertex holds when its output bits equal F(input bits);
//   - PredVertex holds when an arbitrary predicate accepts its bits.
// The shapes only tell a contractor how to evaluate a vertex cheaply; the
// brute-force counter treats all three as predicates.

// Vertex is a predicate on the bits of its incident edges.
type Vertex interface {
	Edges() []int
	// Holds reports whether vals (aligned with Edges()) satisfies the vertex.
	Holds(vals []bool) bool
}

// SupportVertex is satisfied exactly by the listed rows.
type SupportVertex struct {
	E    []int
	Rows [][]bool
	set  map[string]bool
}

func NewSupportVertex(edges []int, rows [][]bool) *SupportVertex {
	v := &SupportVertex{E: edges, Rows: rows, set: make(map[string]bool, len(rows))}
	for _, r := range rows {
		v.set[bitsKey(r)] = true
	}
	return v
}

func (v *SupportVertex) Edges() []int           { return v.E }
func (v *SupportVertex) Holds(vals []bool) bool { return v.set[bitsKey(vals)] }

// FuncVertex is satisfied when the Out bits equal F(In bits) and F accepts.
type FuncVertex struct {
	In, Out []int
	F       func(in []bool) (out []bool, ok bool)
}

func (v *FuncVertex) Edges() []int {
	return append(append([]int{}, v.In...), v.Out...)
}

func (v *FuncVertex) Holds(vals []bool) bool {
	out, ok := v.F(vals[:len(v.In)])
	if !ok {
		return false
	}
	for i, b := range out {
		if vals[len(v.In)+i] != b {
			return false
		}
	}
	return true
}

// PredVertex is satisfied when P accepts its bits.
type PredVertex struct {
	E []int
	P func(vals []bool) bool
}

func (v *PredVertex) Edges() []int           { return v.E }
func (v *PredVertex) Holds(vals []bool) bool { return v.P(vals) }

// Network is a Boolean tensor network. The vertex order is the contraction
// schedule used by SparseContractor.
type Network struct {
	NumEdges int
	Vertices []Vertex
	// Schedule, if set, is an evaluation order for DFSCounter that
	// interleaves every vertex with hints (see schedule.go). It does not
	// change what the network counts.
	Schedule []Step
}

// NetworkStats summarizes the shape of a network.
type NetworkStats struct {
	Edges, Vertices, MaxDegree int
	// IsGraph is true when every edge has at most two endpoints, as in the
	// tensor contraction problem (otherwise it is a hypergraph).
	IsGraph bool
}

func (n *Network) Stats() NetworkStats {
	endpoints := make([]int, n.NumEdges)
	st := NetworkStats{Edges: n.NumEdges, Vertices: len(n.Vertices), IsGraph: true}
	for _, v := range n.Vertices {
		if d := len(v.Edges()); d > st.MaxDegree {
			st.MaxDegree = d
		}
		for _, e := range v.Edges() {
			endpoints[e]++
		}
	}
	for _, k := range endpoints {
		if k > 2 {
			st.IsGraph = false
		}
	}
	return st
}

// Counter solves the Boolean tensor contraction problem: it counts satisfying
// assignments, optionally with some edge bits fixed. The [Ope26] algorithm
// would be one implementation of this interface.
type Counter interface {
	Count(n *Network, fixed map[int]bool) (uint64, error)
}

// BruteForceCounter enumerates all 2^NumEdges assignments. Reference only.
type BruteForceCounter struct{}

func (BruteForceCounter) Count(n *Network, fixed map[int]bool) (uint64, error) {
	if n.NumEdges > 24 {
		return 0, errors.New("ecdl: brute force limited to 24 edges")
	}
	var count uint64
	assign := make([]bool, n.NumEdges)
	for mask := uint64(0); mask < 1<<uint(n.NumEdges); mask++ {
		for e := range assign {
			assign[e] = mask>>uint(e)&1 == 1
		}
		if !agrees(assign, fixed) {
			continue
		}
		ok := true
		for _, v := range n.Vertices {
			es := v.Edges()
			vals := make([]bool, len(es))
			for i, e := range es {
				vals[i] = assign[e]
			}
			if !v.Holds(vals) {
				ok = false
				break
			}
		}
		if ok {
			count++
		}
	}
	return count, nil
}

func agrees(assign []bool, fixed map[int]bool) bool {
	for e, b := range fixed {
		if assign[e] != b {
			return false
		}
	}
	return true
}

// SparseContractor contracts the network vertex by vertex in network order,
// holding the partial contraction as a sparse table over the edges that are
// bound so far, and summing out each edge as soon as no later vertex touches
// it. Each SupportVertex is hash-joined in, each FuncVertex extends rows by
// computing its outputs, and each PredVertex filters rows. The schedule must
// bind all inputs of a FuncVertex or PredVertex before that vertex.
//
// This is an exact, ordinary contractor with no subexponential guarantee. It
// stands in for the [Ope26] solver behind the Counter interface.
type SparseContractor struct {
	// MaxRows aborts contraction when an intermediate table grows past it
	// (0 means unlimited).
	MaxRows int
	// PeakRows records the largest intermediate table seen by the last Count.
	PeakRows int
}

type table struct {
	edges []int
	pos   map[int]int
	rows  map[string]uint64 // key: one byte (0/1) per edge in edges
}

func (s *SparseContractor) Count(n *Network, fixed map[int]bool) (uint64, error) {
	s.PeakRows = 0
	for _, st := range n.Schedule {
		if st.Hint != nil {
			return 0, errors.New("ecdl: SparseContractor does not support hinted schedules; use DFSCounter")
		}
	}
	// Last vertex index touching each edge, for early summing-out.
	last := make([]int, n.NumEdges)
	for e := range last {
		last[e] = -1
	}
	for i, v := range n.Vertices {
		for _, e := range v.Edges() {
			last[e] = i
		}
	}
	t := &table{pos: map[int]int{}, rows: map[string]uint64{"": 1}}
	for i, v := range n.Vertices {
		var err error
		switch v := v.(type) {
		case *SupportVertex:
			t = joinSupport(t, v, fixed)
		case *FuncVertex:
			t, err = applyFunc(t, v, fixed)
		case *PredVertex:
			t, err = applyPred(t, v)
		default:
			err = fmt.Errorf("ecdl: unsupported vertex type %T", v)
		}
		if err != nil {
			return 0, err
		}
		if len(t.rows) > s.PeakRows {
			s.PeakRows = len(t.rows)
		}
		if s.MaxRows > 0 && len(t.rows) > s.MaxRows {
			return 0, fmt.Errorf("ecdl: intermediate table exceeded %d rows", s.MaxRows)
		}
		t = sumOut(t, func(e int) bool { return last[e] > i })
		if len(t.rows) == 0 {
			return 0, nil
		}
	}
	var total uint64
	for _, c := range t.rows {
		total += c
	}
	// Edges touched by no vertex are free unless fixed.
	for e := range last {
		if last[e] == -1 {
			if _, ok := fixed[e]; !ok {
				total *= 2
			}
		}
	}
	return total, nil
}

func newTable(edges []int) *table {
	t := &table{edges: edges, pos: make(map[int]int, len(edges)), rows: map[string]uint64{}}
	for i, e := range edges {
		t.pos[e] = i
	}
	return t
}

func joinSupport(t *table, v *SupportVertex, fixed map[int]bool) *table {
	var shared []int    // indices into v.E bound in t
	var sharedPos []int // their positions in t
	var fresh []int     // indices into v.E not yet bound
	for i, e := range v.E {
		if p, ok := t.pos[e]; ok {
			shared = append(shared, i)
			sharedPos = append(sharedPos, p)
		} else {
			fresh = append(fresh, i)
		}
	}
	// Group the vertex rows (restricted to fixed bits) by their shared part.
	groups := map[string][][]byte{}
	for _, r := range v.Rows {
		ok := true
		for i, e := range v.E {
			if b, isFixed := fixed[e]; isFixed && b != r[i] {
				ok = false
				break
			}
		}
		if !ok {
			continue
		}
		sk := make([]byte, len(shared))
		for j, i := range shared {
			sk[j] = bit(r[i])
		}
		fk := make([]byte, len(fresh))
		for j, i := range fresh {
			fk[j] = bit(r[i])
		}
		groups[string(sk)] = append(groups[string(sk)], fk)
	}
	edges := append([]int{}, t.edges...)
	for _, i := range fresh {
		edges = append(edges, v.E[i])
	}
	out := newTable(edges)
	sk := make([]byte, len(shared))
	for key, c := range t.rows {
		for j, p := range sharedPos {
			sk[j] = key[p]
		}
		for _, fk := range groups[string(sk)] {
			out.rows[key+string(fk)] += c
		}
	}
	return out
}

func applyFunc(t *table, v *FuncVertex, fixed map[int]bool) (*table, error) {
	inPos := make([]int, len(v.In))
	for i, e := range v.In {
		p, ok := t.pos[e]
		if !ok {
			return nil, fmt.Errorf("ecdl: schedule reaches a function vertex before input edge %d is bound", e)
		}
		inPos[i] = p
	}
	outPos := make([]int, len(v.Out)) // -1 when the output edge is new
	edges := append([]int{}, t.edges...)
	for i, e := range v.Out {
		if p, ok := t.pos[e]; ok {
			outPos[i] = p
		} else {
			outPos[i] = -1
			edges = append(edges, e)
		}
	}
	out := newTable(edges)
	in := make([]bool, len(v.In))
	for key, c := range t.rows {
		for i, p := range inPos {
			in[i] = key[p] == 1
		}
		res, ok := v.F(in)
		if !ok {
			continue
		}
		ext := make([]byte, 0, len(v.Out))
		for i, b := range res {
			e := v.Out[i]
			if fb, isFixed := fixed[e]; isFixed && fb != b {
				ok = false
				break
			}
			if outPos[i] >= 0 {
				if (key[outPos[i]] == 1) != b {
					ok = false
					break
				}
			} else {
				ext = append(ext, bit(b))
			}
		}
		if ok {
			out.rows[key+string(ext)] += c
		}
	}
	return out, nil
}

func applyPred(t *table, v *PredVertex) (*table, error) {
	pos := make([]int, len(v.E))
	for i, e := range v.E {
		p, ok := t.pos[e]
		if !ok {
			return nil, fmt.Errorf("ecdl: schedule reaches a predicate vertex before edge %d is bound", e)
		}
		pos[i] = p
	}
	out := newTable(t.edges)
	vals := make([]bool, len(v.E))
	for key, c := range t.rows {
		for i, p := range pos {
			vals[i] = key[p] == 1
		}
		if v.P(vals) {
			out.rows[key] += c
		}
	}
	return out, nil
}

// sumOut projects the table onto the edges for which keep is true.
func sumOut(t *table, keep func(e int) bool) *table {
	var keepIdx []int
	var edges []int
	for i, e := range t.edges {
		if keep(e) {
			keepIdx = append(keepIdx, i)
			edges = append(edges, e)
		}
	}
	if len(edges) == len(t.edges) {
		return t
	}
	out := newTable(edges)
	buf := make([]byte, len(keepIdx))
	for key, c := range t.rows {
		for j, i := range keepIdx {
			buf[j] = key[i]
		}
		out.rows[string(buf)] += c
	}
	return out
}

// FindWitness turns a counting solver into a search solver by
// self-reduction: it fixes the given edges one bit at a time, keeping the
// branch whose count stays positive. It returns nil if there is no solution.
func FindWitness(c Counter, n *Network, edges []int) (map[int]bool, error) {
	fixed := map[int]bool{}
	total, err := c.Count(n, fixed)
	if err != nil || total == 0 {
		return nil, err
	}
	for _, e := range edges {
		fixed[e] = false
		k, err := c.Count(n, fixed)
		if err != nil {
			return nil, err
		}
		if k == 0 {
			fixed[e] = true
		}
	}
	return fixed, nil
}

func bit(b bool) byte {
	if b {
		return 1
	}
	return 0
}

func bitsKey(vals []bool) string {
	k := make([]byte, len(vals))
	for i, b := range vals {
		k[i] = bit(b)
	}
	return string(k)
}
