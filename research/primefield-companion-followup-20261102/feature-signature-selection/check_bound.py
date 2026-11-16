"""Independent theorem-record checker; prime certification remains external."""


def verify(record):
    r, q = record['r'], record['q']
    if type(r) is not int or type(q) is not int or r <= 3 or not 1 <= q < r:
        raise ValueError('prime/window premise')
    if q >= r.bit_length():
        cap = r
    else:
        cap = min(r, 2**q + q)
    status = 'analytic_excluded' if cap < r else 'inconclusive'
    expected = dict(schema='character-signature-bound/v1', r=r, q=q,
                    signature_cap=cap, agreement_cap=cap, status=status,
                    prime_certification='caller precondition',
                    domain='all functions of the q shifted quadratic characters')
    if record != expected:
        raise ValueError('altered theorem record')
    return {'verified_record': True, 'status': status,
            'scope': 'arithmetic premises of documented signature-count theorem'}
