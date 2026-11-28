from research.prime_orbit_roots_20261116_v71.solver import solve
THRESHOLDS={192:16,224:16,256:16,384:17,521:18}
def method(inst):
 return 'factor' if inst['shift_high']-inst['shift_low']+1 >= 1<<THRESHOLDS.get(inst['p'].bit_length(),16) else 'difference'
def portfolio(inst):return solve(inst,method(inst))
