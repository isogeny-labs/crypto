"""Specs of the extended residual-sweep study (lead ICP-residual-partial-relations).

Arm families, all `ic.pipeline` on one prime-order curve per size unless noted:

  context      rho.negation (the matched reference), rho.negation_lanes (16 lanes),
               bsgs.negation, kangaroo.vow
  sweep        residual-sweep:m=2, smallest-abscissa base of B = 2..128
  variants     at B = 8: no sign fold, distinguished-point filters dp = 2/4/6 (limited
               memory), pair-table hybrid, and three base shapes (pseudo-random strided
               abscissae = the null control, known-log progression, known-log random)
  incumbent    the pair-table recipe at base ~ r^(1/3) (walk) and with 16 synchronized
               walks at half, equal and double that base

Sizes 20..36 carry every family (3 sessions); sizes 38..44 carry the context arms and
the small-base sweeps only (the pair-table recipe is out of reach there, and the table
of a large-base sweep would not fit in memory).
"""
import json, pathlib, sys

HERE = pathlib.Path(__file__).parent
SEEDS = {20: 59291, 22: 59292, 24: 59293, 26: 59294, 28: 59297, 30: 59298, 32: 59297, 34: 59299, 36: 59300,
         38: 59301, 40: 59302, 42: 59303, 44: 59304}
INCUMBENT_BASE = {20: 128, 22: 192, 24: 256, 26: 384, 28: 512, 30: 768, 32: 1024, 34: 1536, 36: 2048}
FULL = list(INCUMBENT_BASE)
BIG = [38, 40, 42, 44]


def stride_for(bits):
    """A fixed odd stride below every prime of `bits` bits: the golden fraction of 2^(bits-1)."""
    return int(0.6180339887 * 2 ** (bits - 1)) | 1



def ic(base, oracle, targets="walk", fb="prime-abscissa", extra=None):
    fbp = {"factor_base": f"{fb}:size={base}" + (f",{extra}" if extra else ""), "oracle": oracle, "targets": targets}
    return {"id": "ic.pipeline", "params": fbp}


def arms(bits):
    out = [("rho-neg", "reference", {"id": "rho.negation"}),
           ("rho-lanes16", "candidate", {"id": "rho.negation_lanes", "params": {"lanes": "16"}}),
           ("bsgs-neg", "candidate", {"id": "bsgs.negation"}),
           ("kangaroo", "candidate", {"id": "kangaroo.vow"})]
    big = bits in BIG
    if big:
        # Memory bound: a plain sweep keeps ~sqrt(2 r B) residuals, so past 40 bits only the
        # limited-memory (distinguished-point) variants run.
        if bits <= 40:
            out.append(("sweep-b2", "candidate", ic(2, "residual-sweep:m=2")))
        for b, k in ((2, 4), (2, 6), (8, 4), (8, 6)):
            out.append((f"b{b}-dp{k}", "candidate", ic(b, f"residual-sweep:m=2,dp={k}")))
        return out
    for b in [2, 4, 8, 16, 32, 64, 128]:
        out.append((f"sweep-b{b}", "candidate", ic(b, "residual-sweep:m=2")))
    dps = [2, 4] if bits >= 34 else [2, 4, 6]
    for k in dps:
        out.append((f"b8-dp{k}", "candidate", ic(8, f"residual-sweep:m=2,dp={k}")))
    out += [("b8-nofold", "candidate", ic(8, "residual-sweep:m=2,fold=0")),
            ("b8-hybrid", "candidate", ic(8, "residual-sweep:m=2,pairs=1")),
            ("b8-null-strided", "candidate", ic(8, "residual-sweep:m=2", extra=f"stride={stride_for(bits)}")),
            ("b8-klprog", "candidate", ic(8, "residual-sweep:m=2", fb="known-log")),
            ("b8-klrand", "candidate", ic(8, "residual-sweep:m=2", fb="known-log", extra="scalars=random"))]
    b0 = INCUMBENT_BASE[bits]
    out.append((f"pair-walk-b{b0}", "candidate", ic(b0, "mitm-prime-batch:m=2", targets="walk")))
    for b in ((b0 // 2, b0) if bits >= 34 else (b0 // 2, b0, 2 * b0)):
        out.append((f"pair-b16-b{b}", "candidate", ic(b, "mitm-prime-batch:m=2", targets="walk-prime-batch-16")))
    return out


CURVES = json.loads((HERE / "curves.json").read_text()) if (HERE / "curves.json").exists() else {}


def curve_spec(bits):
    """prime_search covers 8..32 bits; larger sizes use the prime-order curves of gen_curves.py."""
    return CURVES[str(bits)] if bits > 32 else {"kind": "prime_search", "bits": bits, "seed": SEEDS[bits]}


def spec(bits):
    big = bits in BIG
    return {"schema": "ecbench.spec/v1",
            "label": f"extended residual-sweep study, {bits}-bit prime curve",
            "description": "lead ICP-residual-partial-relations, exhaustive: sizes, variants, base shapes, generic comparators",
            "workloads": {"curves": [curve_spec(bits)],
                          "targets_per_curve": 2 if big else 4, "target_seed": 20261002},
            "arms": [{"name": n, "role": r, "method": m} for n, r, m in arms(bits)],
            "measurement": {"rounds": 2 if big else 3, "warmup": 0 if big else 1, "order": "alternate", "seed": 1,
                            "isolation_required": "L2", "timeout_seconds": 3600}}


if __name__ == "__main__":
    for bits in FULL + BIG:
        (HERE / "specs" / f"ext-{bits}.json").write_text(json.dumps(spec(bits), indent=1) + "\n")
    print("wrote", len(FULL + BIG), "specs;", {b: len(arms(b)) for b in FULL + BIG})
