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


def singular_protocol_observation(path, requested):
    """Extract only the uniquely delimited, explicitly labelled std summary.

    Large protocol lines are skipped in bounded chunks; no unlabelled numeric
    token is interpreted. This is an observation, not a field-operation count.
    """
    import re
    from pathlib import Path

    empty={'status':'not_requested' if not requested else 'incomplete',
           'product_criterion':None,'chain_criterion':None,'overlong_lines':0}
    if not requested:
        return empty
    marker_begin=b'=== BEGIN GROEBNER BASIS PROTOCOL ==='
    marker_end=b'=== END GROEBNER BASIS PROTOCOL ==='
    pattern=re.compile(rb'product criterion:\s*(\d+)\s+chain criterion:\s*(\d+)')
    inside=False; began=ended=0; matches=[]; overlong=0
    with Path(path).open('rb') as stream:
        while True:
            line=stream.readline(4097)
            if not line: break
            if len(line)>4096 and not line.endswith(b'\n'):
                overlong+=1
                while line and not line.endswith(b'\n'):
                    line=stream.readline(4097)
                continue
            line=line.strip()
            if line==marker_begin:
                began+=1; inside=True; continue
            if line==marker_end:
                ended+=1; inside=False; continue
            if inside:
                match=pattern.fullmatch(line)
                if match:
                    matches.append((int(match.group(1)),int(match.group(2))))
    empty['overlong_lines']=overlong
    if began!=1 or ended!=1 or inside:
        empty['status']='incomplete' if began<=1 and ended<=1 else 'ambiguous'
    elif len(matches)==0:
        empty['status']='unrecognized'
    elif len(matches)!=1:
        empty['status']='ambiguous'
    else:
        empty.update({'status':'captured','product_criterion':matches[0][0], 'chain_criterion':matches[0][1]})
    return empty
