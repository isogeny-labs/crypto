"""FreeLIP zrandom (Lenstra): state <- state * 3^121 mod (2^107 - 1); zrandom(2^26) = low 26 bits."""
Q = 2**107 - 1
AL = pow(3, 121, Q)

def base_point_x(seed, p, b, skip=7):
    st = seed
    def limb():
        nonlocal st
        st = st * AL % Q
        return st & (2**26 - 1)
    for _ in range(skip): limb()
    nl = -(-p.bit_length() // 26) + 1
    tries = 0
    while True:
        x = 0
        for _ in range(nl): x = x << 26 | limb()
        x %= p; tries += 1
        if pow((x**3 - 3*x + b) % p, (p - 1) // 2, p) == 1:
            return x, tries
