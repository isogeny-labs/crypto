"""Low-degree affine charts for the Semaev-only Frobenius sector split.

This is NOT a globally chart-complete polynomial ideal. Identity, inverse,
vertical-line and two-torsion matches need explicit disjoint charts.
For unspecialized Fp inputs, finite-field equations must also be charged.
"""
import sympy as sp


def leaf_constraints(x, y, s, a, b, d):
    """Boolean twist selector: s=0 base, s=1 nonsquare twist. Degree <=3."""
    scale = 1 + (d - 1) * s
    return [s * (s - 1),
            sp.expand(scale * y*y - x*x*x - a*x - b)]


def generic_add_chart(x1, y1, x2, y2, x3, y3, lam, scale):
    """Guard x1 != x2; all equations total degree <=2."""
    return [sp.expand(lam*(x2-x1) - (y2-y1)),
            sp.expand(x3 - scale*lam*lam + x1+x2),
            sp.expand(y3 - lam*(x1-x3) + y1)]


def generic_double_chart(x, y, x3, y3, lam, scale, a):
    """Guard y !=0, same point; degree <=2."""
    return [sp.expand(lam*2*scale*y - (3*x*x+a)),
            sp.expand(x3 - scale*lam*lam + 2*x),
            sp.expand(y3 - lam*(x-x3) + y)]


def max_generator_degree(equations):
    variables = sorted(set().union(*(eq.free_symbols for eq in equations)), key=str)
    return max((sp.Poly(eq, *variables).total_degree()
                for eq in equations), default=0)
