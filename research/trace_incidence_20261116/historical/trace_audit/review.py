"""Regenerate the independent audit report from the recorded test artifact.

This report separates field-arithmetic complexity from native timings. It is
not a proof of the implementation, an independent timing run, a novelty
claim, or a claim about the complete ECDLP relation search.
"""

import hashlib
import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
audit = json.loads((HERE / "audit_results.json").read_text())
report = {
    "status": audit["status"],
    "scope": "Independent correctness and source/complexity review of complete split-divisor translation; no independent performance measurements.",
    "reproduce": "python /workspace/ecdlp_research/trace_audit/audit.py && python /workspace/ecdlp_research/trace_audit/review.py",
    "production_source_sha256": audit["source_sha256"],
    "native_source_sha256": audit["native_source_sha256"],
    "audit_source_sha256": audit["auditor_sha256"],
    "paper_extracted_text_sha256": hashlib.sha256((ROOT / "2610.32508.txt").read_bytes()).hexdigest(),
    "paper_pdf_sha256": hashlib.sha256(Path("/tmp/codex-remote-attachments/6afa5266-5260-8191-8ee1-0aa6e921e017/C9958719-E306-4915-BB29-FE8334E485FE/1-2610.32508").read_bytes()).hexdigest(),
    "checks": {
        "translation_fixtures": audit["case_count"],
        "complete_translations_both_methods": audit["translation_checks"],
        "internal_identity_cases": len(audit["internal_identity_checks"]),
        "invalid_input_rejection_cases": len(audit["invalid_input_checks"]),
        "detected_output_mutations": sum(m["detected"] for m in audit["mutations"]),
        "translation_field_primes": sorted({r["p"] for r in audit["rows"]}),
        "internal_field_primes": sorted({r["p"] for r in audit["internal_identity_checks"]}),
        "coverage": audit["by_tag"],
    },
    "independence": {
        "expected_group_law": "Construct line/cubic intersection polynomial, divide out known input abscissas including tangent multiplicity, negate third intersection. This does not call production point_add or rational translation maps.",
        "expected_divisors": "Python integer polynomial product and Lagrange interpolation; enumerate F_p to decode every output polynomial in complete-translation checks.",
        "internal_identities": "Direct scalar sums over known roots, independent products of distinct image factors, and direct partial-fraction numerator sums. No production polynomial arithmetic is used to form expected values.",
        "mutations": audit["mutations"],
        "limitation": "The geometric oracle shares the mathematical group law and curve model, but uses a different computational construction. This is extensive testing and source review, not a formal verification.",
    },
    "mathematical_review": {
        "image_multiplicity": "Translation is a bijection on E(F_p). A finite x coordinate has at most two points, so a simple input's translated x fibers have size one or two. A size-two fiber consists of opposite points and cannot be a 2-torsion point.",
        "trace_minimal_polynomial": "Unweighted projected moments are sum_j m_j lambda_j^k with distinct lambda_j and m_j in {1,2}. Since p>3, each m_j is nonzero. The corresponding Vandermonde/diagonal-weight Hankel form has full rank equal to the number of distinct images. Therefore 2d moments determine their squarefree image polynomial, including zero images and d>=p.",
        "multiplicity_and_ordinate_recovery": "For q(T)=product_j(T-lambda_j), N_a(lambda_j)=m_j q'(lambda_j). Thus N_a/q' separates multiplicities by gcd with 1 and 2. The weighted numerator N_b/q' equals the sum of ordinates in a fiber; on singleton fibers this is its unique ordinate. On double fibers both signs are recovered separately by split-algebra square root.",
        "small_characteristic": "The logarithmic-derivative trace series uses no division by moment index. No d<p restriction is needed under the stated image multiplicity promise. Tests include the complete d=p=5 simple input on y^2=x^3+3x.",
        "exceptions": "The x=x_Q support factor is removed before denominator inversion. It maps to 2Q or infinity by the affine group law. Input infinity maps to Q. Q=infinity returns the original divisor. All are included in complete timing.",
        "paper_preconditions": "Paper Algorithm 3 assumes p>3, a nonsquare beta, split simple components, and excludes both Q and -Q from their support. The implementation extends exceptional support handling and supplies a charged nonsquare search when beta is not provided.",
    },
    "complexity_review": {
        "trace_projection": "For m=ceil(sqrt(2d)) and q=ceil(2d/m), constructs an A matrix of shape (2q,d), a B matrix of shape (d,m), and computes A*B. Baby/giant powers and Hankel-to-convolution dual conversion require about sqrt(d) degree-d polynomial operations. Matrix/baby storage is O(d^(3/2)) field entries, with additional Python/native copies.",
        "matrix_cost": "Classical rectangular multiplication costs O(d^2) field operations. Blocking with fast square multiplication gives the familiar O(d^((omega+1)/2)) upper bound. This source review does not establish which multiplication algorithm FLINT selects at measured dimensions.",
        "remaining_quadratic_work": "The supplied native Berlekamp-Massey implementation is quadratic in sequence length and performs modular inversion by exponentiation at nonzero discrepancies. Its field-operation bound is O(d^2+d log p). Thus this implementation does not attain a subquadratic complete-translation bound or confirm the paper's proposed exponent as implemented.",
        "dense_route": "Forms a full d by d multiplication matrix and computes its characteristic polynomial; inverse composition forms and solves a dense singleton-degree square system. The paper's theoretical dense bound is O(d^omega) plus polynomial algebra and lifting. Actual native backend algorithms and constants need measured comparison.",
        "collision_lift": "Shared by both routes. For p=3 mod 4 it uses modular exponentiation. Otherwise it uses Sylow splitting by GCD/modular powers and pairwise CRT. There is no factor() or roots() call in the timed translation source. For supplied beta, a standard soft-O(M(d) log^2 p) allowance covers the split-algebra square-root route; this is not a new root-finding algorithm.",
        "nonsquare_search": "Default general-p calls deterministically try beta=2,3,... and charge that search inside translation. No unconditional polynomial-in-log(p) deterministic bound for this search is asserted. Supplying a validated beta changes that precomputation assumption and must be reported.",
        "novelty": "Paper section 8.1 explicitly proposes trace power projections and inverse modular composition. This implementation is an audited realization with a quadratic recurrence step, not a claim to invent the trace idea.",
    },
    "benchmark_review": {
        "timed_boundary": "benchmark.py times the complete translate call including field/poly conversion, checks, rational map, reconstruction, collision lifting, nonsquare search when required, and exceptional handling. Both methods receive the same (u,v) input. Input construction and C++ helper build/loading precede timing and are disclosed.",
        "comparison_scope": "The dense method reproduces the paper's dense-matrix reconstruction route using the same native polynomial/matrix backend. It shares the new implementation's collision lifting and exception handling with trace. This is not a literal runtime reproduction of the paper's original Magma program or an optimal-algorithm comparison.",
        "process_cost": "Fresh-child CPU/wall and peak RSS are retained separately from the algorithm call; fresh-child totals include process imports, validation and other harness overhead. Peak RSS includes interpreter and native libraries; it is not a measure of polynomial data alone.",
        "fixture_generation": "Timing fixture creation uses known roots and production affine point_add outside both methods. Independent audit supplies separate correctness evidence. Excluded fixture construction is legitimate for translation input (u,v), but cannot be excluded from a complete labelled-pool relation-generation cost.",
        "machine_scope": "CPU-pinned shared container, one FLINT thread; not a dedicated host. Measured speed ratios must be tied to exact fields, degrees, collision patterns, source hashes, and repetitions.",
    },
    "limitations": [
        "Production field API is a prime 3<p<2^63, short nonsingular Weierstrass model. It does not implement binary fields or direct 256-bit prime arithmetic.",
        "Primality and splitting of u are trusted preconditions. Runtime checks reject nonmonic, nonsquarefree and off-curve inputs, but do not promise rejection of arbitrary nonsplit or composite-modulus inputs.",
        "Supported input is a simple finite divisor with at most one point per x, plus at most one infinity. General repeated effective divisors require a separate representation and are outside this API.",
        "Large-prime tests check internal identities for d<=13 without enumerating a large field. Complete end-to-end translations are checked only over the listed small fields.",
        "The audit does not certify cryptographic constant-time behavior, malformed Python object handling, extension fields, threading, or every compiler/platform.",
        "No improved incidence-GCD bound, relation-generation exponent, or complete ECDLP complexity follows from this translation check. Structured factor bases and mixed labelled pools remain separate constructions with separate costs.",
    ],
}
(HERE / "review.json").write_text(json.dumps(report,indent=2)+"\n")
print(json.dumps({"status":report["status"],"report":str(HERE / "review.json"),"checks":{k:v for k,v in report["checks"].items() if k != "coverage"}},indent=2))
