"""Dedicated public-constructor entry point; numerical execution pending."""
from root_binding import build_bound
from driver_root import solve_or_witness


def solve_public(r,q,gamma,weights,meter,events):
    return solve_or_witness(**build_bound(r,q,gamma,weights,meter,events))
