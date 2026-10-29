package ecdl

import (
	"fmt"
	"math/big"
	"math/bits"
)

// Low-degree gadgets for modular arithmetic.
//
// A field element of b bits is split into n blocks of t = ceil(sqrt b) bits.
// Every gadget is a set of vertices that each read O(b^(3/4)) bits, and the
// gadgets use O(b) edges in total. Field multiplication checks the integer
// identity a*b - q*p - c = 0 (with witness q). The two convolutions are
// computed with a number-theoretic transform of length L >= 2n over a prime
// Q, factored four-step style as L = L1*L2. That gives two stages of about
// sqrt(L) vertices each, and each vertex sees about sqrt(L) blocks. A carry
// chain then checks that the coefficients sum to zero.
//
// This is our own construction, guided by the paper's one-line description
// ("a two-stage Fourier-transform decomposition"); the paper's gadget is not
// reproduced here.

// Arith holds the parameters for arithmetic modulo P.
type Arith struct {
	P      *big.Int
	Bits   int // b
	T, N   int // block width t and number of blocks n
	L      int // transform length, a power of two >= 2n
	L1, L2 int
	Q      *big.Int // NTT prime, Q = 1 mod L and Q > 4 n 4^t
	WQ, WE int      // bit widths of transform values and of signed carries

	omega, omegaInv []*big.Int // omega^e and omega^-e for e in [0, L)
	lInv            *big.Int
	pHat            []*big.Int // transform of p's blocks
	widths          []int      // block widths of a field element
}

// NewArith chooses the block size, transform and NTT prime for modulus p.
func NewArith(p *big.Int) *Arith {
	ar := &Arith{P: new(big.Int).Set(p), Bits: p.BitLen()}
	b := ar.Bits
	t := 1
	for t*t < b {
		t++
	}
	ar.T = t
	ar.N = (b + t - 1) / t
	ar.L = 1
	for ar.L < 2*ar.N {
		ar.L *= 2
	}
	lg := bits.TrailingZeros(uint(ar.L))
	ar.L1 = 1 << uint(lg/2)
	ar.L2 = ar.L / ar.L1
	for i := 0; i < ar.N; i++ {
		w := t
		if i == ar.N-1 {
			w = b - t*(ar.N-1)
		}
		ar.widths = append(ar.widths, w)
	}

	// Q = k L + 1 > 4 n 2^(2t), prime.
	bound := new(big.Int).Lsh(big.NewInt(int64(4*ar.N)), uint(2*t))
	L := big.NewInt(int64(ar.L))
	k := new(big.Int).Div(bound, L)
	q := new(big.Int)
	for {
		k.Add(k, big.NewInt(1))
		q.Mul(k, L).Add(q, big.NewInt(1))
		if q.ProbablyPrime(30) {
			break
		}
	}
	ar.Q = q
	ar.WQ = q.BitLen()
	// Carries satisfy |e| < n 2^t + 2; store them in two's complement.
	ar.WE = new(big.Int).Lsh(big.NewInt(int64(ar.N+2)), uint(t)).BitLen() + 1

	// omega of order exactly L: g^((Q-1)/L) with omega^(L/2) = -1.
	e := new(big.Int).Div(new(big.Int).Sub(q, big.NewInt(1)), L)
	qm1 := new(big.Int).Sub(q, big.NewInt(1))
	var omega *big.Int
	for g := int64(2); ; g++ {
		omega = new(big.Int).Exp(big.NewInt(g), e, q)
		if new(big.Int).Exp(omega, big.NewInt(int64(ar.L/2)), q).Cmp(qm1) == 0 {
			break
		}
	}
	oi := new(big.Int).ModInverse(omega, q)
	ar.omega = make([]*big.Int, ar.L)
	ar.omegaInv = make([]*big.Int, ar.L)
	ar.omega[0], ar.omegaInv[0] = big.NewInt(1), big.NewInt(1)
	for i := 1; i < ar.L; i++ {
		ar.omega[i] = new(big.Int).Mod(new(big.Int).Mul(ar.omega[i-1], omega), q)
		ar.omegaInv[i] = new(big.Int).Mod(new(big.Int).Mul(ar.omegaInv[i-1], oi), q)
	}
	ar.lInv = new(big.Int).ModInverse(L, q)
	ar.pHat = ar.ntt(ar.blocksOf(p))
	return ar
}

// blocksOf splits x >= 0 into L blocks of t bits (zero beyond n).
func (ar *Arith) blocksOf(x *big.Int) []*big.Int {
	out := make([]*big.Int, ar.L)
	mask := new(big.Int).Sub(new(big.Int).Lsh(big.NewInt(1), uint(ar.T)), big.NewInt(1))
	for i := range out {
		out[i] = new(big.Int).And(new(big.Int).Rsh(x, uint(i*ar.T)), mask)
	}
	return out
}

// ntt is the direct O(L^2) transform, used for constants only.
func (ar *Arith) ntt(x []*big.Int) []*big.Int {
	out := make([]*big.Int, ar.L)
	for k := range out {
		s := new(big.Int)
		for i, xi := range x {
			s.Add(s, new(big.Int).Mul(xi, ar.omega[(i*k)%ar.L]))
		}
		out[k] = s.Mod(s, ar.Q)
	}
	return out
}

func (ar *Arith) w(e int) *big.Int    { return ar.omega[((e%ar.L)+ar.L)%ar.L] }
func (ar *Arith) wInv(e int) *big.Int { return ar.omegaInv[((e%ar.L)+ar.L)%ar.L] }

// Var is a multi-bit value carried by edges, split into blocks.
type Var struct {
	name     string
	widths   []int
	src      [][]int   // edges written by the defining step
	uses     [][][]int // fresh edges per consumer, joined to src at Finalize
	producer bool      // src already has a vertex endpoint
	defined  bool
	copyStep int // schedule slot reserved for fan-out vertices
}

// Width is the total number of bits.
func (v *Var) Width() int {
	w := 0
	for _, x := range v.widths {
		w += x
	}
	return w
}

// Edges returns the defining edges, LSB first (valid after Finalize).
func (v *Var) Edges() []int {
	var out []int
	for _, blk := range v.src {
		out = append(out, blk...)
	}
	return out
}

// Operand is a field-element input to a gadget: a Var or a constant.
type Operand struct {
	V *Var
	C *big.Int
}

type varHint struct {
	reads, writes []*Var
	gen           func(in []*big.Int) [][]*big.Int
}

type bstep struct {
	v    Vertex
	hint *varHint
	fan  *Var // fan-out vertices for this Var go here
}

// Builder assembles a gadget network and its schedule. Each Var is defined
// once (by a hint or a support vertex) before it is used; Finalize joins the
// consumers of each Var to its defining edges, with per-block copy vertices
// when there are too many to share an edge.
type Builder struct {
	next  int
	steps []bstep
	// Exhaustive makes every hint enumerate all values of the Vars it writes
	// instead of the computed witness. Only for testing at tiny sizes: it
	// checks that the vertices alone pin every witness down.
	Exhaustive bool
}

func (b *Builder) edges(k int) []int {
	e := make([]int, k)
	for i := range e {
		e[i] = b.next
		b.next++
	}
	return e
}

func (b *Builder) NewVar(name string, widths []int) *Var {
	v := &Var{name: name, widths: widths}
	for _, w := range widths {
		v.src = append(v.src, b.edges(w))
	}
	return v
}

// FieldVar allocates a field element of ar's block layout.
func (b *Builder) FieldVar(ar *Arith, name string) *Var { return b.NewVar(name, ar.widths) }

func (b *Builder) add(v Vertex) { b.steps = append(b.steps, bstep{v: v}) }

func (b *Builder) markDefined(v *Var) {
	if v.defined {
		panic("ecdl: Var " + v.name + " defined twice")
	}
	v.defined = true
	v.copyStep = len(b.steps)
	b.steps = append(b.steps, bstep{fan: v})
}

// Hint defines the writes Vars from the reads Vars.
func (b *Builder) Hint(reads, writes []*Var, gen func(in []*big.Int) [][]*big.Int) {
	for _, r := range reads {
		if !r.defined {
			panic("ecdl: hint reads undefined Var " + r.name)
		}
	}
	b.steps = append(b.steps, bstep{hint: &varHint{reads: reads, writes: writes, gen: gen}})
	for _, w := range writes {
		b.markDefined(w)
	}
}

// Support defines vars by a support vertex listing their joint values.
func (b *Builder) Support(vars []*Var, rows [][]*big.Int) {
	var edges []int
	for _, v := range vars {
		edges = append(edges, v.Edges()...)
	}
	brows := make([][]bool, len(rows))
	for i, row := range rows {
		for j, v := range vars {
			bs, ok := uintBits(row[j], v.Width())
			if !ok {
				panic("ecdl: support value does not fit")
			}
			brows[i] = append(brows[i], bs...)
		}
	}
	b.add(NewSupportVertex(edges, brows))
	for _, v := range vars {
		v.producer = true
		b.markDefined(v)
	}
}

// use returns fresh edges, block by block, for one consumer of v.
func (b *Builder) use(v *Var) [][]int {
	if !v.defined {
		panic("ecdl: use of undefined Var " + v.name)
	}
	blk := make([][]int, len(v.widths))
	for i, w := range v.widths {
		blk[i] = b.edges(w)
	}
	v.uses = append(v.uses, blk)
	return blk
}

// Finalize produces the network. Vars stay valid: their Edges() refer to the
// final numbering.
func (b *Builder) Finalize() *Network {
	alias := map[int]int{}
	fanVerts := map[int][]Vertex{}
	seen := map[*Var]bool{}
	for _, st := range b.steps {
		v := st.fan
		if v == nil || seen[v] {
			continue
		}
		seen[v] = true
		k := len(v.uses)
		ends := k
		if v.producer {
			ends++
		}
		if k == 0 && !v.producer {
			panic("ecdl: Var " + v.name + " is defined by a hint but never constrained")
		}
		if ends <= 2 {
			for _, u := range v.uses {
				for j := range u {
					for i, e := range u[j] {
						alias[e] = v.src[j][i]
					}
				}
			}
			continue
		}
		for j := range v.widths {
			var out []int
			for _, u := range v.uses {
				out = append(out, u[j]...)
			}
			w := v.widths[j]
			fanVerts[v.copyStep] = append(fanVerts[v.copyStep], &FuncVertex{
				In:  v.src[j],
				Out: out,
				F: func(in []bool) ([]bool, bool) {
					o := make([]bool, 0, k*w)
					for i := 0; i < k; i++ {
						o = append(o, in...)
					}
					return o, true
				},
			})
		}
	}

	// Renumber edges densely, applying aliases.
	renum := map[int]int{}
	id := func(e int) int {
		if a, ok := alias[e]; ok {
			e = a
		}
		if r, ok := renum[e]; ok {
			return r
		}
		r := len(renum)
		renum[e] = r
		return r
	}
	mapAll := func(es []int) []int {
		out := make([]int, len(es))
		for i, e := range es {
			out[i] = id(e)
		}
		return out
	}
	varEdges := func(vs []*Var) []int {
		var out []int
		for _, v := range vs {
			out = append(out, v.Edges()...)
		}
		return out
	}

	n := &Network{}
	push := func(v Vertex) {
		var nv Vertex
		switch v := v.(type) {
		case *SupportVertex:
			nv = NewSupportVertex(mapAll(v.E), v.Rows)
		case *FuncVertex:
			nv = &FuncVertex{In: mapAll(v.In), Out: mapAll(v.Out), F: v.F}
		case *PredVertex:
			nv = &PredVertex{E: mapAll(v.E), P: v.P}
		default:
			panic(fmt.Sprintf("ecdl: unexpected vertex %T", v))
		}
		n.Vertices = append(n.Vertices, nv)
		n.Schedule = append(n.Schedule, Step{Vertex: nv})
	}
	for i, st := range b.steps {
		switch {
		case st.v != nil:
			push(st.v)
		case st.hint != nil:
			h := st.hint
			n.Schedule = append(n.Schedule, Step{Hint: b.edgeHint(h, mapAll(varEdges(h.reads)), mapAll(varEdges(h.writes)))})
		case st.fan != nil:
			for _, fv := range fanVerts[i] {
				push(fv)
			}
		}
	}
	// Vars keep the final numbering for their defining edges.
	for v := range seen {
		for j := range v.src {
			v.src[j] = mapAll(v.src[j])
		}
	}
	n.NumEdges = len(renum)
	return n
}

func (b *Builder) edgeHint(h *varHint, reads, writes []int) *EdgeHint {
	exhaustive := b.Exhaustive
	return &EdgeHint{Reads: reads, Writes: writes, Gen: func(in []bool) [][]bool {
		if exhaustive {
			total := 0
			for _, w := range h.writes {
				total += w.Width()
			}
			out := make([][]bool, 0, 1<<uint(total))
			for mask := 0; mask < 1<<uint(total); mask++ {
				row := make([]bool, total)
				for i := range row {
					row[i] = mask>>uint(i)&1 == 1
				}
				out = append(out, row)
			}
			return out
		}
		vals := make([]*big.Int, len(h.reads))
		off := 0
		for i, r := range h.reads {
			vals[i] = bitsUint(in[off : off+r.Width()])
			off += r.Width()
		}
		var out [][]bool
		for _, cand := range h.gen(vals) {
			var row []bool
			ok := true
			for i, w := range h.writes {
				bs, fits := uintBits(cand[i], w.Width())
				if !fits {
					ok = false
					break
				}
				row = append(row, bs...)
			}
			if ok {
				out = append(out, row)
			}
		}
		return out
	}}
}

// bitsUint reads an unsigned LSB-first integer.
func bitsUint(vals []bool) *big.Int {
	x := new(big.Int)
	for i := len(vals) - 1; i >= 0; i-- {
		x.Lsh(x, 1)
		if vals[i] {
			x.SetBit(x, 0, 1)
		}
	}
	return x
}

// uintBits writes 0 <= x < 2^w as w bits, LSB first.
func uintBits(x *big.Int, w int) ([]bool, bool) {
	if x.Sign() < 0 || x.BitLen() > w {
		return nil, false
	}
	out := make([]bool, w)
	for i := range out {
		out[i] = x.Bit(i) == 1
	}
	return out, true
}

// bitsSigned reads a two's-complement LSB-first integer.
func bitsSigned(vals []bool) *big.Int {
	x := bitsUint(vals)
	if len(vals) > 0 && vals[len(vals)-1] {
		x.Sub(x, new(big.Int).Lsh(big.NewInt(1), uint(len(vals))))
	}
	return x
}

// signedBits writes -2^(w-1) <= x < 2^(w-1) in two's complement.
func signedBits(x *big.Int, w int) ([]bool, bool) {
	lim := new(big.Int).Lsh(big.NewInt(1), uint(w-1))
	if x.Cmp(lim) >= 0 || x.Cmp(new(big.Int).Neg(lim)) < 0 {
		return nil, false
	}
	y := new(big.Int).Set(x)
	if y.Sign() < 0 {
		y.Add(y, new(big.Int).Lsh(big.NewInt(1), uint(w)))
	}
	return uintBits(y, w)
}

// splitBits cuts vals into consecutive pieces of the given widths.
func splitBits(vals []bool, widths ...int) [][]bool {
	out := make([][]bool, len(widths))
	off := 0
	for i, w := range widths {
		out[i] = vals[off : off+w]
		off += w
	}
	return out
}

func concat(es ...[]int) []int {
	var out []int
	for _, e := range es {
		out = append(out, e...)
	}
	return out
}

func operandValue(o Operand, vals map[*Var]*big.Int) *big.Int {
	if o.V != nil {
		return vals[o.V]
	}
	return o.C
}

// MulCheck adds the constraint x*y = c (mod p) with 0 <= c < p. If defineC,
// c is defined here (and range-checked); otherwise c must already be defined
// and range-checked by the caller.
func (b *Builder) MulCheck(ar *Arith, x, y Operand, c *Var, defineC bool) {
	q := b.NewVar("q", ar.widths)
	var reads []*Var
	for _, o := range []Operand{x, y} {
		if o.V != nil {
			reads = append(reads, o.V)
		}
	}
	if !defineC {
		reads = append(reads, c)
	}
	writes := []*Var{q}
	if defineC {
		writes = append(writes, c)
	}
	p := ar.P
	b.Hint(reads, writes, func(in []*big.Int) [][]*big.Int {
		vals := map[*Var]*big.Int{}
		for i, r := range reads {
			vals[r] = in[i]
		}
		prod := new(big.Int).Mul(operandValue(x, vals), operandValue(y, vals))
		if defineC {
			qq, cc := new(big.Int).DivMod(prod, p, new(big.Int))
			return [][]*big.Int{{qq, cc}}
		}
		diff := prod.Sub(prod, vals[c])
		qq, rem := new(big.Int).DivMod(diff, p, new(big.Int))
		if rem.Sign() != 0 || qq.Sign() < 0 {
			return nil
		}
		return [][]*big.Int{{qq}}
	})
	if defineC {
		b.RangeCheck(ar, c, ar.P, false)
	}

	// Stage 1 for each variable operand: vertex n2 reads blocks L2*n1 + n2
	// and outputs Y[n2][k1] = w^(n2 k1) sum_n1 x[L2 n1 + n2] w^(L2 n1 k1).
	type stage1 struct {
		y [][][]int // y[n2][k1]: WQ edges, nil when identically zero
	}
	var ops []stage1
	var constHat [][]*big.Int // transforms of constant operands, by position
	isConst := []bool{}
	for _, o := range []Operand{x, y, {V: q}} {
		if o.V == nil {
			constHat = append(constHat, ar.ntt(ar.blocksOf(new(big.Int).Mod(o.C, ar.P))))
			isConst = append(isConst, true)
			ops = append(ops, stage1{})
			continue
		}
		constHat = append(constHat, nil)
		isConst = append(isConst, false)
		blk := b.use(o.V)
		s := stage1{y: make([][][]int, ar.L2)}
		for n2 := 0; n2 < ar.L2; n2++ {
			var in []int
			var idx, wid []int
			for n1 := 0; n1 < ar.L1; n1++ {
				if i := ar.L2*n1 + n2; i < ar.N {
					in = append(in, blk[i]...)
					idx = append(idx, n1)
					wid = append(wid, ar.widths[i])
				}
			}
			if len(in) == 0 {
				continue
			}
			s.y[n2] = make([][]int, ar.L1)
			var out []int
			for k1 := range s.y[n2] {
				s.y[n2][k1] = b.edges(ar.WQ)
				out = append(out, s.y[n2][k1]...)
			}
			n2 := n2
			b.add(&FuncVertex{In: in, Out: out, F: func(bitsIn []bool) ([]bool, bool) {
				parts := splitBits(bitsIn, wid...)
				res := make([]bool, 0, ar.L1*ar.WQ)
				for k1 := 0; k1 < ar.L1; k1++ {
					s := new(big.Int)
					for j, n1 := range idx {
						s.Add(s, new(big.Int).Mul(bitsUint(parts[j]), ar.w(ar.L2*n1*k1)))
					}
					s.Mul(s.Mod(s, ar.Q), ar.w(n2*k1)).Mod(s, ar.Q)
					bs, _ := uintBits(s, ar.WQ)
					res = append(res, bs...)
				}
				return res, true
			}})
		}
		ops = append(ops, s)
	}

	// Stage 2 + pointwise product + inverse stage 1: vertex k1 finishes the
	// transforms X[k1 + L1 k2], forms D = X_x X_y - X_q pHat, and outputs
	// Z[k1][r] = w^(-r k1) sum_k2 D[k1 + L1 k2] w^(-L1 r k2).
	z := make([][][]int, ar.L1) // z[k1][r]
	for k1 := 0; k1 < ar.L1; k1++ {
		var in []int
		type src struct{ op, n2 int }
		var srcs []src
		for oi, s := range ops {
			if isConst[oi] {
				continue
			}
			for n2 := 0; n2 < ar.L2; n2++ {
				if s.y[n2] != nil {
					in = append(in, s.y[n2][k1]...)
					srcs = append(srcs, src{oi, n2})
				}
			}
		}
		z[k1] = make([][]int, ar.L2)
		var out []int
		for r := range z[k1] {
			z[k1][r] = b.edges(ar.WQ)
			out = append(out, z[k1][r]...)
		}
		k1 := k1
		b.add(&FuncVertex{In: in, Out: out, F: func(bitsIn []bool) ([]bool, bool) {
			Y := make([][]*big.Int, 3) // Y[op][n2], nil means zero
			for i := range Y {
				Y[i] = make([]*big.Int, ar.L2)
			}
			for i, s := range srcs {
				v := bitsUint(bitsIn[i*ar.WQ : (i+1)*ar.WQ])
				if v.Cmp(ar.Q) >= 0 {
					return nil, false
				}
				Y[s.op][s.n2] = v
			}
			X := func(op, k2 int) *big.Int {
				if isConst[op] {
					return constHat[op][k1+ar.L1*k2]
				}
				s := new(big.Int)
				for n2, yv := range Y[op] {
					if yv != nil {
						s.Add(s, new(big.Int).Mul(yv, ar.w(ar.L1*n2*k2)))
					}
				}
				return s.Mod(s, ar.Q)
			}
			D := make([]*big.Int, ar.L2)
			for k2 := range D {
				d := new(big.Int).Mul(X(0, k2), X(1, k2))
				d.Sub(d, new(big.Int).Mul(X(2, k2), ar.pHat[k1+ar.L1*k2]))
				D[k2] = d.Mod(d, ar.Q)
			}
			res := make([]bool, 0, ar.L2*ar.WQ)
			for r := 0; r < ar.L2; r++ {
				s := new(big.Int)
				for k2, d := range D {
					s.Add(s, new(big.Int).Mul(d, ar.wInv(ar.L1*r*k2)))
				}
				s.Mul(s.Mod(s, ar.Q), ar.wInv(r*k1)).Mod(s, ar.Q)
				bs, _ := uintBits(s, ar.WQ)
				res = append(res, bs...)
			}
			return res, true
		}})
	}

	// Inverse stage 2: vertex r outputs d[r + L2 s] = L^-1 sum_k1 w^(-L2 s k1) Z[k1][r].
	d := make([][]int, ar.L)
	for r := 0; r < ar.L2; r++ {
		var in, out []int
		for k1 := 0; k1 < ar.L1; k1++ {
			in = append(in, z[k1][r]...)
		}
		for s := 0; s < ar.L1; s++ {
			d[r+ar.L2*s] = b.edges(ar.WQ)
			out = append(out, d[r+ar.L2*s]...)
		}
		b.add(&FuncVertex{In: in, Out: out, F: func(bitsIn []bool) ([]bool, bool) {
			Z := make([]*big.Int, ar.L1)
			for k1 := range Z {
				Z[k1] = bitsUint(bitsIn[k1*ar.WQ : (k1+1)*ar.WQ])
				if Z[k1].Cmp(ar.Q) >= 0 {
					return nil, false
				}
			}
			res := make([]bool, 0, ar.L1*ar.WQ)
			for s := 0; s < ar.L1; s++ {
				v := new(big.Int)
				for k1, zv := range Z {
					v.Add(v, new(big.Int).Mul(zv, ar.wInv(ar.L2*s*k1)))
				}
				v.Mul(v.Mod(v, ar.Q), ar.lInv).Mod(v, ar.Q)
				bs, _ := uintBits(v, ar.WQ)
				res = append(res, bs...)
			}
			return res, true
		}})
	}

	// Carry chain: lift d_j to (-Q/2, Q/2), subtract c_j, add the carry, and
	// require divisibility by 2^t; the last position must leave zero.
	cblk := b.use(c)
	halfQ := new(big.Int).Rsh(ar.Q, 1)
	var carry []int
	for j := 0; j < ar.L; j++ {
		in := append([]int{}, d[j]...)
		cw := 0
		if j < ar.N {
			in = append(in, cblk[j]...)
			cw = ar.widths[j]
		}
		hasCarry := carry != nil
		in = append(in, carry...)
		value := func(bitsIn []bool) *big.Int {
			v := bitsUint(bitsIn[:ar.WQ])
			if v.Cmp(halfQ) > 0 {
				v.Sub(v, ar.Q)
			}
			v.Sub(v, bitsUint(bitsIn[ar.WQ:ar.WQ+cw]))
			if hasCarry {
				v.Add(v, bitsSigned(bitsIn[ar.WQ+cw:]))
			}
			return v
		}
		if j == ar.L-1 {
			b.add(&PredVertex{E: in, P: func(bitsIn []bool) bool { return value(bitsIn).Sign() == 0 }})
			break
		}
		carry = b.edges(ar.WE)
		mask := new(big.Int).Sub(new(big.Int).Lsh(big.NewInt(1), uint(ar.T)), big.NewInt(1))
		b.add(&FuncVertex{In: in, Out: carry, F: func(bitsIn []bool) ([]bool, bool) {
			v := value(bitsIn)
			if new(big.Int).And(v, mask).Sign() != 0 {
				return nil, false
			}
			return signedBits(v.Rsh(v, uint(ar.T)), ar.WE)
		}})
	}
}

// Term is coef * V in a linear check.
type Term struct {
	Coef int
	V    *Var
}

// LinCheck adds the constraint out + sum coef_i v_i + konst = 0 (mod p), where
// out may be nil. If defineOut, out is defined here and range-checked to
// [0, p). All term Vars must already be range-checked by their definers. The
// witness k = (out + sum + konst)/p travels along the block chain.
func (b *Builder) LinCheck(ar *Arith, out *Var, defineOut bool, terms []Term, konst *big.Int) {
	kc := new(big.Int).Mod(konst, ar.P)
	sumAbs := 2
	for _, t := range terms {
		if t.Coef < 0 {
			sumAbs -= t.Coef
		} else {
			sumAbs += t.Coef
		}
	}
	wk := big.NewInt(int64(sumAbs)).BitLen() + 1
	wc := big.NewInt(int64(2*sumAbs+4)).BitLen() + 1
	k := b.NewVar("k", []int{wk})

	var reads, writes []*Var
	for _, t := range terms {
		reads = append(reads, t.V)
	}
	if out != nil && !defineOut {
		reads = append(reads, out)
	}
	if defineOut {
		writes = append(writes, out)
	}
	writes = append(writes, k)
	p := ar.P
	b.Hint(reads, writes, func(in []*big.Int) [][]*big.Int {
		s := new(big.Int).Set(kc)
		for i, t := range terms {
			s.Add(s, new(big.Int).Mul(big.NewInt(int64(t.Coef)), in[i]))
		}
		var o *big.Int
		switch {
		case defineOut:
			o = new(big.Int).Mod(new(big.Int).Neg(s), p)
		case out != nil:
			o = in[len(terms)]
		default:
			o = new(big.Int)
		}
		s.Add(s, o)
		kk, rem := new(big.Int).DivMod(s, p, new(big.Int))
		if rem.Sign() != 0 {
			return nil
		}
		// k is stored in two's complement but the hint layer is unsigned.
		kb, ok := signedBits(kk, wk)
		if !ok {
			return nil
		}
		if defineOut {
			return [][]*big.Int{{o, bitsUint(kb)}}
		}
		return [][]*big.Int{{bitsUint(kb)}}
	})
	if defineOut {
		b.RangeCheck(ar, out, ar.P, false)
	}

	var outBlk [][]int
	if out != nil {
		outBlk = b.use(out)
	}
	termBlk := make([][][]int, len(terms))
	for i, t := range terms {
		termBlk[i] = b.use(t.V)
	}
	kEdges := b.use(k)[0]
	pb := ar.blocksOf(ar.P)
	cb := ar.blocksOf(kc)
	var carry []int
	for j := 0; j < ar.N; j++ {
		w := ar.widths[j]
		var in []int
		var widths []int
		if out != nil {
			in = append(in, outBlk[j]...)
			widths = append(widths, w)
		}
		for i := range terms {
			in = append(in, termBlk[i][j]...)
			widths = append(widths, w)
		}
		in = append(in, kEdges...)
		widths = append(widths, wk)
		hasCarry := carry != nil
		if hasCarry {
			in = append(in, carry...)
			widths = append(widths, wc)
		}
		j := j
		value := func(bitsIn []bool) (*big.Int, []bool) {
			parts := splitBits(bitsIn, widths...)
			v := new(big.Int).Set(cb[j])
			pi := 0
			if out != nil {
				v.Add(v, bitsUint(parts[0]))
				pi = 1
			}
			for i, t := range terms {
				v.Add(v, new(big.Int).Mul(big.NewInt(int64(t.Coef)), bitsUint(parts[pi+i])))
			}
			kbits := parts[pi+len(terms)]
			v.Sub(v, new(big.Int).Mul(bitsSigned(kbits), pb[j]))
			if hasCarry {
				v.Add(v, bitsSigned(parts[pi+len(terms)+1]))
			}
			return v, kbits
		}
		if j == ar.N-1 {
			b.add(&PredVertex{E: in, P: func(bitsIn []bool) bool {
				v, _ := value(bitsIn)
				return v.Sign() == 0
			}})
			break
		}
		carry = b.edges(wc)
		kNext := b.edges(wk)
		mask := new(big.Int).Sub(new(big.Int).Lsh(big.NewInt(1), uint(ar.T)), big.NewInt(1))
		b.add(&FuncVertex{In: in, Out: concat(carry, kNext), F: func(bitsIn []bool) ([]bool, bool) {
			v, kbits := value(bitsIn)
			if new(big.Int).And(v, mask).Sign() != 0 {
				return nil, false
			}
			cbits, ok := signedBits(v.Rsh(v, uint(ar.T)), wc)
			if !ok {
				return nil, false
			}
			return append(cbits, kbits...), true
		}})
		kEdges = kNext
	}
}

// RangeCheck adds the constraint v < U (and v != 0 if nonzero), comparing
// block by block from the least significant end.
func (b *Builder) RangeCheck(ar *Arith, v *Var, U *big.Int, nonzero bool) {
	blk := b.use(v)
	ub := ar.blocksOf(U)
	if U.BitLen() > ar.N*ar.T {
		panic("ecdl: range bound too large")
	}
	var state []int // [lt, nz]
	for j := 0; j < len(v.widths); j++ {
		in := append(append([]int{}, blk[j]...), state...)
		w := v.widths[j]
		has := state != nil
		j := j
		step := func(bitsIn []bool) (lt, nz bool) {
			x := bitsUint(bitsIn[:w])
			if has {
				lt, nz = bitsIn[w], bitsIn[w+1]
			}
			cmp := x.Cmp(ub[j])
			return cmp < 0 || (cmp == 0 && lt), nz || x.Sign() != 0
		}
		if j == len(v.widths)-1 {
			b.add(&PredVertex{E: in, P: func(bitsIn []bool) bool {
				lt, nz := step(bitsIn)
				return lt && (!nonzero || nz)
			}})
			break
		}
		state = b.edges(2)
		b.add(&FuncVertex{In: in, Out: state, F: func(bitsIn []bool) ([]bool, bool) {
			lt, nz := step(bitsIn)
			return []bool{lt, nz}, true
		}})
	}
}

// NewBit allocates a 1-bit Var.
func (b *Builder) NewBit(name string) *Var { return b.NewVar(name, []int{1}) }

func boolInt(x bool) *big.Int {
	if x {
		return big.NewInt(1)
	}
	return new(big.Int)
}

// IsZero defines the bit z = [v == 0], checked by an OR chain over v's blocks.
func (b *Builder) IsZero(v *Var) *Var {
	z := b.NewBit("iszero")
	b.Hint([]*Var{v}, []*Var{z}, func(in []*big.Int) [][]*big.Int {
		return [][]*big.Int{{boolInt(in[0].Sign() == 0)}}
	})
	blk := b.use(v)
	ze := b.use(z)[0]
	var nz []int
	for j := range blk {
		in := concat(blk[j], nz)
		w, has := v.widths[j], nz != nil
		anySet := func(bitsIn []bool) bool {
			x := false
			for _, bb := range bitsIn[:w] {
				x = x || bb
			}
			return x || (has && bitsIn[w])
		}
		if j == len(blk)-1 {
			b.add(&PredVertex{E: append(in, ze...), P: func(bitsIn []bool) bool {
				return anySet(bitsIn) != bitsIn[len(bitsIn)-1]
			}})
			break
		}
		nz = b.edges(1)
		b.add(&FuncVertex{In: in, Out: nz, F: func(bitsIn []bool) ([]bool, bool) {
			return []bool{anySet(bitsIn)}, true
		}})
	}
	return z
}

// flagChain checks ok(j, f, bits of edgesAt(j)) for every block j, passing
// the flag bit f from vertex to vertex so each vertex reads it once.
func (b *Builder) flagChain(f *Var, n int, edgesAt func(j int) []int, ok func(j int, flag bool, vals []bool) bool) {
	fe := b.use(f)[0]
	for j := 0; j < n; j++ {
		in := concat(fe, edgesAt(j))
		j := j
		check := func(bitsIn []bool) bool { return ok(j, bitsIn[0], bitsIn[1:]) }
		if j == n-1 {
			b.add(&PredVertex{E: in, P: check})
			break
		}
		next := b.edges(1)
		b.add(&FuncVertex{In: in, Out: next, F: func(bitsIn []bool) ([]bool, bool) {
			if !check(bitsIn) {
				return nil, false
			}
			return []bool{bitsIn[0]}, true
		}})
		fe = next
	}
}

// Select defines the field element w = f ? u : v.
func (b *Builder) Select(ar *Arith, f *Var, u, v Operand) *Var {
	w := b.FieldVar(ar, "select")
	reads := []*Var{f}
	for _, o := range []Operand{u, v} {
		if o.V != nil {
			reads = append(reads, o.V)
		}
	}
	b.Hint(reads, []*Var{w}, func(in []*big.Int) [][]*big.Int {
		vals := map[*Var]*big.Int{}
		for i, r := range reads {
			vals[r] = in[i]
		}
		if in[0].Sign() != 0 {
			return [][]*big.Int{{operandValue(u, vals)}}
		}
		return [][]*big.Int{{operandValue(v, vals)}}
	})
	blocks := func(o Operand) ([][]int, []*big.Int) {
		if o.V != nil {
			return b.use(o.V), nil
		}
		return nil, ar.blocksOf(new(big.Int).Mod(o.C, ar.P))
	}
	ue, uc := blocks(u)
	ve, vc := blocks(v)
	we := b.use(w)
	b.flagChain(f, ar.N, func(j int) []int {
		var e []int
		if ue != nil {
			e = append(e, ue[j]...)
		}
		if ve != nil {
			e = append(e, ve[j]...)
		}
		return append(e, we[j]...)
	}, func(j int, flag bool, vals []bool) bool {
		wd := ar.widths[j]
		off := 0
		take := func(edges [][]int, c []*big.Int) *big.Int {
			if edges == nil {
				return c[j]
			}
			x := bitsUint(vals[off : off+wd])
			off += wd
			return x
		}
		uj, vj := take(ue, uc), take(ve, vc)
		wj := bitsUint(vals[off : off+wd])
		if flag {
			return wj.Cmp(uj) == 0
		}
		return wj.Cmp(vj) == 0
	})
	return w
}

// ZeroIf adds the constraint f => v = 0.
func (b *Builder) ZeroIf(ar *Arith, f, v *Var) {
	ve := b.use(v)
	b.flagChain(f, len(ve), func(j int) []int { return ve[j] }, func(j int, flag bool, vals []bool) bool {
		if !flag {
			return true
		}
		for _, bb := range vals {
			if bb {
				return false
			}
		}
		return true
	})
}
