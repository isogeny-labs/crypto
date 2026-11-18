from research.prime_orbit_roots_20261116_v71.solver import solve as reference
from .solver import solve as candidate
from .kernel import eligible
def engine(p):return candidate if not(p&(p+1))or eligible(p)else reference
def solve(inst):return engine(inst['p'])(inst)
