// Command ecdlscale prints the shape of the low-degree gadget networks for
// field multiplication, complete point addition, and the per-point validity
// and membership checks as the field size grows:
// edges per bit should level off (O(b) edges) and max degree / b^(3/4)
// should stay bounded. Networks are built but never evaluated.
package main

import (
	"flag"
	"fmt"
	"math"
	"math/big"
	"strconv"
	"strings"

	"ecdltensor"
)

func main() {
	sizes := flag.String("bits", "64,256,1024,4096,16384", "comma-separated field sizes")
	flag.Parse()
	fmt.Printf("%7s %5s %4s %4s | %8s %8s %8s | %7s %9s\n",
		"b", "t", "n", "L", "mul e/b", "add e/b", "point e/b", "maxdeg", "deg/b^.75")
	for _, s := range strings.Split(*sizes, ",") {
		b, err := strconv.Atoi(strings.TrimSpace(s))
		if err != nil || b < 4 {
			fmt.Println("bad size:", s)
			continue
		}
		p := new(big.Int).Lsh(big.NewInt(1), uint(b-1))
		p.Add(p, big.NewInt(1))
		ar := ecdl.NewArith(p)
		mul, add, point := ecdl.GadgetShapes(ar)
		maxdeg := 0
		for _, st := range []ecdl.NetworkStats{mul, add, point} {
			if st.MaxDegree > maxdeg {
				maxdeg = st.MaxDegree
			}
			if !st.IsGraph {
				fmt.Println("warning: not a graph at b =", b)
			}
		}
		per := func(st ecdl.NetworkStats) float64 { return float64(st.Edges) / float64(b) }
		fmt.Printf("%7d %5d %4d %4d | %8.1f %8.1f %8.1f | %7d %9.1f\n",
			b, ar.T, ar.N, ar.L, per(mul), per(add), per(point), maxdeg, float64(maxdeg)/math.Pow(float64(b), 0.75))
	}
}
