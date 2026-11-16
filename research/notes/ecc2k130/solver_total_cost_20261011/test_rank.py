"""Unit tests for the Z/r elimination and orbit lookup (run: python3 -m unittest test_rank)."""
import random
import unittest

from tcommon import rank_add, Stages


class RankTests(unittest.TestCase):
    def test_small_prime_matches_bruteforce(self):
        p = 7
        rng = random.Random(1)
        for _ in range(200):
            rows = [[rng.randrange(p) for _ in range(4)] for _ in range(rng.randrange(1, 7))]
            piv, got = {}, 0
            for r in rows:
                got += rank_add(piv, r, p)
            self.assertEqual(got, len(piv))
            self.assertEqual(got, brute_rank(rows, p))

    def test_dependent_row_changes_nothing(self):
        p = 11
        piv = {}
        self.assertTrue(rank_add(piv, [1, 2, 3], p))
        self.assertTrue(rank_add(piv, [0, 1, 5], p))
        before = {k: list(v) for k, v in piv.items()}
        # 3*[1,2,3] + 2*[0,1,5] = [3, 8, 19] = [3, 8, 8] mod 11
        self.assertFalse(rank_add(piv, [3, 8, 8], p))
        self.assertEqual(piv, before)

    def test_big_prime(self):
        p = 2417851639230796216685689
        piv = {}
        self.assertTrue(rank_add(piv, [1, 0], p))
        self.assertFalse(rank_add(piv, [p, 0], p))
        self.assertTrue(rank_add(piv, [5, p - 1], p))
        self.assertFalse(rank_add(piv, [10, p - 2], p))

    def test_stage_charged_on_exception(self):
        s = Stages()
        with self.assertRaises(ValueError):
            with s.stage("solve"):
                raise ValueError
        self.assertEqual(s.dump()["solve"]["calls"], 1)


def brute_rank(rows, p):
    m = [list(r) for r in rows]
    rank = 0
    for c in range(len(m[0])):
        pivot = next((i for i in range(rank, len(m)) if m[i][c] % p), None)
        if pivot is None:
            continue
        m[rank], m[pivot] = m[pivot], m[rank]
        inv = pow(m[rank][c], -1, p)
        m[rank] = [x * inv % p for x in m[rank]]
        for i in range(len(m)):
            if i != rank and m[i][c] % p:
                f = m[i][c]
                m[i] = [(a - f * b) % p for a, b in zip(m[i], m[rank])]
        rank += 1
    return rank


if __name__ == "__main__":
    unittest.main()
