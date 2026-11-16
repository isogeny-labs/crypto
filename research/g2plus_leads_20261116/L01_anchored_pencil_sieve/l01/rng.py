M64 = (1 << 64) - 1
GOLD = 0x9E3779B97F4A7C15


def mix64(z):
    z = (z + GOLD) & M64
    z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & M64
    z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & M64
    return z ^ (z >> 31)


class SM64:
    def __init__(self, state):
        self.state = state & M64

    def next(self):
        self.state = (self.state + GOLD) & M64
        return mix64(self.state)

    def below(self, n):
        return self.next() % n

    def bit(self):
        return self.next() & 1


def stream_seed(tag, g, p, cs, ss):
    a = mix64((tag ^ (g << 56)) & M64)
    b = mix64((a ^ p) & M64)
    return mix64((b ^ (cs << 20) ^ ss) & M64)


def stream(tag, g, p, cs, ss):
    return SM64(stream_seed(tag, g, p, cs, ss))
