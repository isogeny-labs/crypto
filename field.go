// Package ecdl is a toy implementation of index calculus for the elliptic-curve
// discrete logarithm problem in which relations are found by Boolean tensor
// contraction. Every component runs at toy sizes (p below 2^24 or so); see the
// README for what this prototype does and does not demonstrate.
package ecdl

import "math/bits"

// mulMod returns a*b mod m for a, b < m.
func mulMod(a, b, m uint64) uint64 {
	hi, lo := bits.Mul64(a, b)
	_, rem := bits.Div64(hi, lo, m)
	return rem
}

func addMod(a, b, m uint64) uint64 {
	s := a + b
	if s >= m || s < a {
		s -= m
	}
	return s
}

func subMod(a, b, m uint64) uint64 {
	if a >= b {
		return a - b
	}
	return a + (m - b)
}

func powMod(a, e, m uint64) uint64 {
	r := uint64(1) % m
	a %= m
	for e > 0 {
		if e&1 == 1 {
			r = mulMod(r, a, m)
		}
		a = mulMod(a, a, m)
		e >>= 1
	}
	return r
}

// invMod returns a^-1 mod a prime m. a must be nonzero mod m.
func invMod(a, m uint64) uint64 {
	return powMod(a, m-2, m)
}

// IsPrime is a deterministic Miller-Rabin test for 64-bit integers.
func IsPrime(n uint64) bool {
	if n < 2 {
		return false
	}
	for _, q := range []uint64{2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37} {
		if n%q == 0 {
			return n == q
		}
	}
	d, s := n-1, 0
	for d%2 == 0 {
		d /= 2
		s++
	}
	for _, a := range []uint64{2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37} {
		x := powMod(a, d, n)
		if x == 1 || x == n-1 {
			continue
		}
		composite := true
		for i := 1; i < s; i++ {
			x = mulMod(x, x, n)
			if x == n-1 {
				composite = false
				break
			}
		}
		if composite {
			return false
		}
	}
	return true
}
