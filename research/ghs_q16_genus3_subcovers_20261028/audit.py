#!/usr/bin/env python3
"""Independent GF(2) matrix replay and receipt linkage checks."""

import hashlib
import itertools
import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
MODULUS = (1 << 12) | (1 << 7) | (1 << 6) | (1 << 5) | (1 << 3) | (1 << 1) | 1


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def gf_mul(x, y):
    result = 0
    for _ in range(12):
        if y & 1:
            result ^= x
        y >>= 1
        x <<= 1
        if x & (1 << 12):
            x ^= MODULUS
    assert y == 0
    return result


def rank(vectors):
    pivots = {}
    for vector in vectors:
        while vector:
            pivot = vector.bit_length()-1
            if pivot in pivots:
                vector ^= pivots[pivot]
            else:
                pivots[pivot] = vector
                break
    return len(pivots)


def rank_mod(rows, p):
    if not rows:
        return 0
    A = [[int(x) % p for x in row] for row in rows]
    width = len(A[0])
    assert all(len(row) == width for row in A)
    pivot_row = 0
    for col in range(width):
        pivot = next((j for j in range(pivot_row, len(A)) if A[j][col]), None)
        if pivot is None:
            continue
        A[pivot_row], A[pivot] = A[pivot], A[pivot_row]
        inv = pow(A[pivot_row][col], -1, p)
        A[pivot_row] = [(z*inv) % p for z in A[pivot_row]]
        for j in range(pivot_row+1, len(A)):
            factor = A[j][col]
            if factor:
                A[j] = [(x-factor*y) % p for x, y in zip(A[j], A[pivot_row])]
        pivot_row += 1
        if pivot_row == len(A):
            break
    return pivot_row


def audit_attack(row):
    p = row["subgroup_prime"]
    M, rhs = row["relation_matrix_rows"], row["relation_rhs"]
    assert len(M) == len(rhs) == row["relation_rows"]
    assert all(len(coeffs) == row["factor_base_size"]+1 for coeffs in M)
    assert rank_mod(M,p) == row["relation_rank"]
    factor_base = [coeffs[:-1] for coeffs in M]
    assert rank_mod(M,p) == rank_mod(factor_base,p)+1
    shifted_rhs = [(b-coeffs[-1]*row["recovered_log"]) % p
                   for coeffs,b in zip(M,rhs)]
    assert rank_mod([coeffs+[b] for coeffs,b in zip(factor_base,shifted_rhs)],p) == rank_mod(factor_base,p)
    rho_steps = row["rho_steps_per_success"]
    assert len(rho_steps) == row["rho_successful_trials"] == 1000
    assert abs(sum(rho_steps)/1000-row["rho_mean_steps"]) < 1e-9


def main():
    search = json.loads((HERE/"receipt_3974.json").read_text())
    assert search["search_sha256"] == sha(HERE/"search.py")
    assert search["tested"] == 1474200 and not search["matches"]
    assert len(search["unavoidable_cubic_pole_patterns"]) == 24
    assert len(search["square_patterns"]) == 6

    automorphisms = json.loads((HERE/"automorphism_receipt.json").read_text())
    assert automorphisms["search_sha256"] == sha(HERE/"automorphisms.py")
    assert all(row["mobius_count"] == 12 and
               row["lifted_branch_permutations"] == [[0,1,2,3]]
               for row in automorphisms["results"])

    descent_path = HERE/"descent_receipt.json"
    descent = json.loads(descent_path.read_text())
    assert descent["search_sha256"] == sha(HERE/"descent.py")
    assert descent["intersection"] == [] and len(descent["rows"]) == 6
    for row in descent["rows"]:
        columns = row["matrix_column_bitmasks"]
        basis = row["kernel_basis_bitmasks"]
        assert len(columns) == 24 and len(basis) == 4
        assert rank(columns) == row["rank"] == 20
        assert rank(basis) == row["nullity"] == 4
        for vec in basis:
            assert 0 <= vec < (1 << 24)
            image = 0
            for j, col in enumerate(columns):
                if (vec >> j) & 1:
                    image ^= col
            assert image == 0
        found = set()
        for bits in itertools.product((0,1),repeat=4):
            vec = 0
            for bit, basis_vec in zip(bits,basis):
                if bit:
                    vec ^= basis_vec
            lam, mu = vec & 4095, (vec >> 12) & 4095
            if lam and mu:
                b = gf_mul(gf_mul(lam,mu),gf_mul(lam,mu))
                found.add((lam,mu,b))
        expected = {(p["lambda_mask"],p["mu_mask"],p["b_mask"])
                    for p in row["pairs"]}
        assert found == expected and len(found) == 15
    assert set(descent["eligible_b_masks"]) == {p["b_mask"]
        for p in descent["rows"][0]["pairs"]}

    map_path = HERE/"map15_receipt.json"
    mapped = json.loads(map_path.read_text())
    assert mapped["search_sha256"] == sha(HERE/"map15.py")
    assert mapped["descent_receipt_sha256"] == sha(descent_path)
    assert len(mapped["rows"]) == 15
    assert {r["b_mask"] for r in mapped["rows"]} == set(descent["eligible_b_masks"])
    assert all(r["verified_nonzero_base_field_subgroup_map"] for r in mapped["rows"])

    attacked = json.loads((HERE/"attack15_receipt.json").read_text())
    assert attacked["search_sha256"] == sha(HERE/"attack15.py")
    assert attacked["map_receipt_sha256"] == sha(map_path)
    assert len(attacked["rows"]) == 15
    assert {r["b_mask"] for r in attacked["rows"]} == set(descent["eligible_b_masks"])
    assert all(r["status"] == "SOLVED" and r["recovered_log"] == 7
               and r["rho_successful_trials"] == 1000
               and r["full_transfer_over_mean_rho_wall_ratio"] > 1
               and r["verified_on_source_and_jacobian"]
               for r in attacked["rows"])
    for row in attacked["rows"]:
        audit_attack(row)
    prior = json.loads((HERE/"prior_work_receipt.json").read_text())
    assert prior["search_sha256"] == sha(HERE/"prior_work.py")
    assert prior["map_receipt_sha256"] == sha(map_path)
    assert prior["attack_receipt_sha256"] == sha(HERE/"attack15_receipt.json")
    assert {r["b_mask"] for r in prior["rows"]} == set(descent["eligible_b_masks"])
    assert all(r["prior_lmfdb_isogeny_class"] and
               r["exact_equation_publication_status"] == "unverified" and
               r["verified_eligible_transfer"] and
               not r["measured_full_cost_below_rho"] for r in prior["rows"])
    family = json.loads((HERE/"family_formula_receipt.json").read_text())
    assert family["search_sha256"] == sha(HERE/"family_formula.py")
    assert family["map_receipt_sha256"] == sha(map_path)
    assert len(family["rows"]) == 15
    assert {r["delta_base_mask"] for r in family["rows"]} == set(range(1,16))
    representative_path = HERE/"map4162_receipt.json"
    representative = json.loads(representative_path.read_text())
    assert representative["search_sha256"] == sha(HERE/"map4162.py")
    assert representative["source_b_mask"] == 1384
    assert all(representative["checks"].values())
    representative_attack = json.loads((HERE/"attack4162_receipt.json").read_text())
    assert representative_attack["search_sha256"] == sha(HERE/"attack4162.py")
    assert representative_attack["map_receipt_sha256"] == sha(representative_path)
    assert representative_attack["source_target_log_recovered"] == 7
    assert representative_attack["verified_on_source_and_jacobian"]
    audit_attack({**representative_attack,
                  "recovered_log": representative_attack["source_target_log_recovered"]})
    print("PASS: six binary ranks; 16 modular relation systems; 15 maps, attacks, prior gates; hashes")


if __name__ == "__main__":
    main()
