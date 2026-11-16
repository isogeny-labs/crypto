"""Offline conversion only; no collector, subgroup, or primality validation."""
def diagnostic_row(modulus, legacy_row):
    """[A,b,d] for A*x+b*k=d -> [A,b,-d] for A*x+b*k+c=0."""
    if type(modulus) is not int or modulus < 2:
        raise ValueError("modulus must be an integer >=2")
    if not isinstance(legacy_row, (list, tuple)) or len(legacy_row) < 2:
        raise ValueError("row requires target and constant columns")
    if any(type(v) is not int or not 0 <= v < modulus for v in legacy_row):
        raise ValueError("canonical integer residues required")
    return list(legacy_row[:-1]) + [(-legacy_row[-1]) % modulus]
