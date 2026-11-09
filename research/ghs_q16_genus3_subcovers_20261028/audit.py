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


def gf_pow(x, exponent):
    out = 1
    while exponent:
        if exponent & 1:
            out = gf_mul(out, x)
        x = gf_mul(x, x)
        exponent >>= 1
    return out


def relative_trace(x):
    return x ^ gf_pow(x, 16) ^ gf_pow(x, 256)


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
               r["published_family_match"] and
               r["relative_trace_b_mask"] == relative_trace(r["b_mask"]) == 0 and
               r["verified_eligible_transfer"] and
               not r["measured_full_cost_below_rho"] for r in prior["rows"])
    assert all(r["published_trace_zero_genus3_cover"] ==
               "https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf"
               for r in prior["rows"])

    catalog_path = HERE/"catalog_models.json"
    catalog = json.loads(catalog_path.read_text())
    assert sum(len(row["curves"]) for row in catalog["rows"]) == 60
    degree3 = json.loads((HERE/"degree3_branch_supported_receipt.json").read_text())
    assert degree3["script_sha256"] == sha(HERE/"degree3_branch_supported.py")
    assert degree3["catalog_sha256"] == sha(catalog_path)
    assert len(degree3["models"]) == 60
    assert ({row["class"] for row in degree3["models"]} ==
            {row["label"] for row in catalog["rows"]})
    for model in degree3["models"]:
        assert len(model["branch_roots_masks"]) == len(model["reduced_residue_masks"]) == 4
        assert all(z != 0 for z in model["reduced_residue_masks"])
        assert len(model["cases"]) == 24
        rhs = model["rhs_bitmask"]
        for case in model["cases"]:
            columns = case["matrix_column_bitmasks"]
            assert len(columns) == 24
            assert rank(columns) == case["rank"] == 24
            assert rank(columns + [rhs]) == case["augmented_rank"] == 25

    frontier_path = HERE.parent/"ghs_q16_genus4_frontier_20261028"/"summary.json"
    frontier = json.loads(frontier_path.read_text())
    isogeny = json.loads((HERE/"isogeny_feasibility_receipt.json").read_text())
    assert isogeny["script_sha256"] == sha(HERE/"isogeny_feasibility.py")
    assert isogeny["frontier_sha256"] == sha(frontier_path)
    assert isogeny["catalog_sha256"] == sha(catalog_path)
    assert len(isogeny["classes"]) == 3
    assert sum(len(row["models"]) for row in isogeny["classes"]) == 66
    for row, source_class in zip(isogeny["classes"], frontier["classes"]):
        n, r, t = row["source_order"], row["subgroup_prime"], row["source_trace_t"]
        assert n == source_class["source_order"] and r == source_class["subgroup_prime"]
        assert t == 4097-n
        assert row["frobenius_polynomial_coefficients"] == [1,0,0,-t,0,0,4096]
        assert (row["discriminant_mod_r"] ==
                (3**6*16**6*(t*t-4*16**3)**3) % r != 0)
        assert row["f_squarefree_mod_r"] and row["prime_to_r_isogeny_exists_by_tate"]
        assert not row["explicit_isogeny_evaluated"]
        assert ({x["b_mask"] for x in row["models"]} ==
                {x["b_mask"] for x in source_class["models"]})
        assert all(x["relative_trace_b_mask"] == relative_trace(x["b_mask"]) != 0
                   for x in row["models"])
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

    all_catalog_path = HERE/"catalog_all_q16_weil_classes.jsonl"
    all_catalog = [json.loads(s) for s in all_catalog_path.read_text().splitlines()]
    assert len(all_catalog) == 126
    assert sum(len(row["curves"]) for row in all_catalog) == 10664
    full_scan_path = HERE/"degree3_all_q16_scan_receipt.json"
    full_scan = json.loads(full_scan_path.read_text())
    assert full_scan["catalog_sha256"] == sha(all_catalog_path)
    assert full_scan["script_sha256"] == sha(HERE/"degree3_all_q16_scan.py")
    assert full_scan["catalog_classes"] == len(all_catalog)
    assert full_scan["models_scanned"] == 10664
    assert full_scan["branch_polynomials"] == 7
    assert full_scan["patterns_per_model"] == 24
    assert full_scan["counts"] == {"models":10664,"exact_hits":60}
    assert len(full_scan["hits"]) == 60
    labels = {row["label"]:row for row in all_catalog}
    assert {hit["class"] for hit in full_scan["hits"]} == {
        "3.16.a_a_abv","3.16.a_a_adt","3.16.a_a_ah",
        "3.16.a_a_bv","3.16.a_a_dt","3.16.a_a_h"}
    for hit in full_scan["hits"]:
        assert labels[hit["class"]]["curves"][hit["model_index"]] == hit["equation"]
        assert sum(labels[hit["class"]]["poly"]) == hit["expected_order"]
        assert relative_trace(hit["b_mask"]) == hit["trace_b_mask"]
        assert sorted(hit["exponents"]) == [-2,-1,1,2]
    prior3 = json.loads((HERE/"prior_work_degree3_receipt.json").read_text())
    assert prior3["scan_sha256"] == sha(full_scan_path)
    assert prior3["script_sha256"] == sha(HERE/"prior_work_degree3.py")
    assert len(prior3["rows"]) == 60
    assert sum(hit["trace_b_mask"] != 0 for hit in full_scan["hits"]) == 36
    assert len({(hit["u_mask"],hit["b_mask"]) for hit in full_scan["hits"]}) == 30
    for index in (5,12):
        suffix = "" if index == 5 else "_12"
        cover_path = HERE/f"degree3_nonzero_trace_receipt{suffix}.json"
        cover = json.loads(cover_path.read_text())
        assert cover["scan_receipt_sha256"] == sha(full_scan_path)
        assert cover["script_sha256"] == sha(HERE/"degree3_nonzero_trace.py")
        assert cover["catalog_model_index"] == index
        assert cover["source_order"] == 4090 and cover["subgroup_prime"] == 409
        assert cover["relative_trace_b_mask"] == relative_trace(cover["b_mask"]) != 0
        assert cover["verified_exact_cover"] and cover["verified_subgroup_transfer"]
        attack = json.loads((HERE/f"attack_degree3_receipt{suffix}.json").read_text())
        assert attack["map_receipt_sha256"] == sha(cover_path)
        assert attack["search_sha256"] == sha(HERE/"attack_degree3.py")
        row = attack["rows"][0]
        assert row["b_mask"] == cover["b_mask"]
        assert row["status"] == "SOLVED" and row["recovered_log"] == 7
        assert row["full_transfer_over_mean_rho_wall_ratio"] > 1
        assert row["verified_on_source_and_jacobian"]
        audit_attack(row)
    print("PASS: old 15 maps and attacks; 10,664-model degree-three scan; 60 prior gates; two nonzero-trace maps and full attacks; hashes")


if __name__ == "__main__":
    main()
