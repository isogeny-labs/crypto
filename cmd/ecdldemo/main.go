// Command ecdldemo generates a random toy prime-order curve and recovers a
// random discrete logarithm by index calculus with tensor-contraction
// relation finding.
package main

import (
	"flag"
	"fmt"
	"log"
	"math/rand"
	"os"
	"time"

	"ecdltensor"
)

func main() {
	bits := flag.Int("bits", 14, "field size b in bits (4..26)")
	m := flag.Int("m", 2, "number of factor bases")
	seed := flag.Int64("seed", time.Now().UnixNano(), "random seed")
	quiet := flag.Bool("q", false, "suppress progress output")
	gadgets := flag.Bool("gadgets", false, "use low-degree arithmetic gadgets (slow; keep -bits small)")
	window := flag.Int("window", 0, "with -gadgets: constrain x < p>>k in every factor base (0 = off)")
	flag.Parse()

	rng := rand.New(rand.NewSource(*seed))
	c, err := ecdl.RandomPrimeOrderCurve(rng, *bits)
	if err != nil {
		log.Fatal(err)
	}
	xi := uint64(rng.Int63n(int64(c.Order)))
	Q := c.Mul(xi, c.G)
	fmt.Printf("curve  y^2 = x^3 + %d x + %d over F_%d, prime order r = %d\n", c.A, c.B, c.P, c.Order)
	fmt.Printf("P = (%d, %d), Q = (%d, %d)\n", c.G.X, c.G.Y, Q.X, Q.Y)

	s := &ecdl.Solver{Curve: c, M: *m, Counter: &ecdl.SparseContractor{}, Rng: rng}
	if *gadgets {
		s.Gadgets, s.Counter = true, ecdl.DFSCounter{}
		s.WindowBits = *window
	}
	if !*quiet {
		s.Logf = func(f string, a ...interface{}) { fmt.Fprintf(os.Stderr, f+"\n", a...) }
	}
	got, st, err := s.Solve(Q)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("network: %d edges, %d vertices, max degree %d, graph=%v\n",
		st.Network.Edges, st.Network.Vertices, st.Network.MaxDegree, st.Network.IsGraph)
	fmt.Printf("N = %d, relations = %d, attempts = %d, restarts = %d, time = %v\n",
		st.FactorBaseSize, st.Relations, st.Attempts, st.Restarts, st.Elapsed.Round(time.Millisecond))
	fmt.Printf("recovered xi = %d (secret %d) — verified [xi]P == Q: %v\n", got, xi, c.Mul(got, c.G) == Q)
}
