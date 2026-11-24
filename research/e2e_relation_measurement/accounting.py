"""Exact outcome counts for signed relation witnesses."""


def count_witnesses(witnesses):
    """Classify all witnesses; a failed scalar check is never a usable result."""
    result = {'total': 0, 'zero_b': 0, 'nonzero_b': 0,
              'nonzero_b_verified': 0, 'nonzero_b_verification_failed': 0}
    for witness in witnesses:
        b = witness['B']
        verified = witness['verified']
        if type(b) is not int or type(verified) is not bool:
            raise ValueError('invalid witness B or verification status')
        result['total'] += 1
        if b == 0:
            if verified:
                raise ValueError('zero-B witness cannot have a verified scalar')
            result['zero_b'] += 1
        else:
            result['nonzero_b'] += 1
            key = 'nonzero_b_verified' if verified else 'nonzero_b_verification_failed'
            result[key] += 1
    return result
