"""Independent coefficient certificate check; no producer imports, unexecuted."""
def verify(row, column, encoding, meter):
    if not isinstance(encoding,dict):return False
    q=len(column)
    if q == 0 or len(row)!=q:
        return False
    p=meter.r
    if any(type(x) is not int or not 0<=x<p for v in (row,column) for x in v):
        return False
    mode=encoding.get('mode')
    stored=encoding.get('column')
    if not isinstance(stored,(list,tuple)) or tuple(stored)!=tuple(column):
        return False
    if mode=='plain':
        stored_row=encoding.get('row')
        return (encoding.get('scale') is None and isinstance(stored_row,(list,tuple))
                and tuple(stored_row)==tuple(row))
    if encoding.get('row') is not None:
        return False
    if mode in ('reverse','zero_row'):
        if encoding.get('scale') is not None:return False
        scale=1 if mode=='reverse' else 0
    elif mode=='scaled_reverse':
        scale=encoding.get('scale')
        if type(scale) is not int or not 1<scale<p:return False
    else:return False
    for j in range(q):
        meter.add('border_certificate_entry_tests')
        if row[q-1-j] != meter.mul(scale,column[j]):return False
    return True
