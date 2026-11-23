"""Independent integer-arithmetic oracle for the F101 complete-row pilot."""
from itertools import product

P, A, B = 101, 7, 4
O = None


def add(left, right):
    if left is None:
        return right
    if right is None:
        return left
    x1, y1 = left
    x2, y2 = right
    if x1 == x2 and (y1 + y2) % P == 0:
        return O
    if left == right:
        slope = (3 * x1 * x1 + A) * pow(2 * y1, -1, P) % P
    else:
        slope = (y2 - y1) * pow(x2 - x1, -1, P) % P
    x3 = (slope * slope - x1 - x2) % P
    return x3, (slope * (x1 - x3) - y1) % P


def points():
    yield O
    for x in range(P):
        for y in range(P):
            if (y * y - x * x * x - A * x - B) % P == 0:
                yield x, y


def panel(size, length):
    full = list(points())
    base = []
    for x in range(P):
        lifts = [point for point in full if point is not None and point[0] == x]
        if lifts:
            base.append(min(lifts))
        if len(base) == size:
            break
    signed = [(index, sign, (point[0], sign * point[1] % P))
              for index, point in enumerate(base) for sign in (1, -1)]
    rows = {target: set() for target in full}
    for terms in product(signed, repeat=length):
        total = O
        row = [0] * size
        for index, sign, point in terms:
            total = add(total, point)
            row[index] += sign
        rows[total].add(tuple(row))
    return len(full), sum(bool(value) for value in rows.values()), sum(map(len, rows.values()))


if __name__ == '__main__':
    assert (4 * A**3 + 27 * B**2) % P != 0
    for length in (3, 4):
        for size in (4, 8):
            print(length, size, panel(size, length))
