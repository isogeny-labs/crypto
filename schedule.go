package ecdl

import (
	"errors"
	"fmt"
)

// Step is one entry of a hinted schedule: a vertex to evaluate, or a hint.
type Step struct {
	Vertex Vertex
	Hint   *EdgeHint
}

// EdgeHint proposes candidate values for the Writes edges given the values of
// the Reads edges. A hint is not a constraint: the count is defined by the
// vertices alone. Hints only steer enumeration, so they must be complete:
// every satisfying assignment must agree with one of the candidates. The
// gadgets in this package use hints for witnesses that the vertices pin down
// uniquely (quotients, inverses, field outputs), and their completeness is
// tested exhaustively at small sizes.
type EdgeHint struct {
	Reads, Writes []int
	Gen           func(read []bool) [][]bool
}

// DFSCounter counts satisfying assignments by depth-first enumeration along
// the network's schedule: support vertices and hints branch, function
// vertices extend the assignment, predicate vertices prune. It handles
// networks with or without a Schedule. Like SparseContractor it is exact and
// exponential; it stands in for a real contraction solver.
type DFSCounter struct{}

type dfs struct {
	sched  []Step
	assign []int8 // -1 unbound, 0, 1
	fixed  map[int]bool
	trail  []int
}

func (DFSCounter) Count(n *Network, fixed map[int]bool) (uint64, error) {
	sched := n.Schedule
	if sched == nil {
		for _, v := range n.Vertices {
			sched = append(sched, Step{Vertex: v})
		}
	}
	touched := make([]bool, n.NumEdges)
	for _, v := range n.Vertices {
		for _, e := range v.Edges() {
			touched[e] = true
		}
	}
	d := &dfs{sched: sched, assign: make([]int8, n.NumEdges), fixed: fixed}
	for e := range d.assign {
		d.assign[e] = -1
	}
	total, err := d.run(0)
	if err != nil {
		return 0, err
	}
	for e, t := range touched {
		if !t {
			if _, ok := fixed[e]; !ok {
				total *= 2
			}
		}
	}
	return total, nil
}

// bind assigns vals to edges, failing on a clash with an existing or fixed
// value. Newly bound edges are pushed on the trail.
func (d *dfs) bind(edges []int, vals []bool) bool {
	for i, e := range edges {
		b := vals[i]
		if fb, ok := d.fixed[e]; ok && fb != b {
			return false
		}
		switch d.assign[e] {
		case -1:
			d.assign[e] = int8(bit(b))
			d.trail = append(d.trail, e)
		case int8(bit(b)):
		default:
			return false
		}
	}
	return true
}

func (d *dfs) undo(mark int) {
	for _, e := range d.trail[mark:] {
		d.assign[e] = -1
	}
	d.trail = d.trail[:mark]
}

func (d *dfs) read(edges []int) ([]bool, error) {
	vals := make([]bool, len(edges))
	for i, e := range edges {
		if d.assign[e] < 0 {
			return nil, fmt.Errorf("ecdl: schedule reads edge %d before it is bound", e)
		}
		vals[i] = d.assign[e] == 1
	}
	return vals, nil
}

func (d *dfs) run(i int) (uint64, error) {
	if i == len(d.sched) {
		return 1, nil
	}
	st := d.sched[i]
	mark := len(d.trail)
	defer d.undo(mark)
	if st.Hint != nil {
		in, err := d.read(st.Hint.Reads)
		if err != nil {
			return 0, err
		}
		var total uint64
		for _, cand := range st.Hint.Gen(in) {
			if d.bind(st.Hint.Writes, cand) {
				k, err := d.run(i + 1)
				if err != nil {
					return 0, err
				}
				total += k
			}
			d.undo(mark)
		}
		return total, nil
	}
	switch v := st.Vertex.(type) {
	case *SupportVertex:
		var total uint64
		for _, row := range v.Rows {
			if d.bind(v.E, row) {
				k, err := d.run(i + 1)
				if err != nil {
					return 0, err
				}
				total += k
			}
			d.undo(mark)
		}
		return total, nil
	case *FuncVertex:
		in, err := d.read(v.In)
		if err != nil {
			return 0, err
		}
		out, ok := v.F(in)
		if !ok || !d.bind(v.Out, out) {
			return 0, nil
		}
		return d.run(i + 1)
	case *PredVertex:
		vals, err := d.read(v.E)
		if err != nil {
			return 0, err
		}
		if !v.P(vals) {
			return 0, nil
		}
		return d.run(i + 1)
	case nil:
		return 0, errors.New("ecdl: empty schedule step")
	default:
		return 0, fmt.Errorf("ecdl: unsupported vertex type %T", v)
	}
}
