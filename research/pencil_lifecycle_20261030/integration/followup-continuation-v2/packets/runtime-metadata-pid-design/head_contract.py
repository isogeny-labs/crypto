"""Pure prospective HEAD header decoder; no socket, subprocess or Docker action."""
import base64,json
DIR=1<<31

def directory_header(status,headers,expected_name):
    if status!=200:raise ValueError('HEAD status')
    values=[v for k,v in headers if k.lower()=='x-docker-container-path-stat']
    if len(values)!=1 or len(values[0])>65536:raise ValueError('PathStat count/cap')
    raw=base64.b64decode(values[0],validate=True)
    if len(raw)>16384:raise ValueError('decoded cap')
    def unique(pairs):
        d={}
        for k,v in pairs:
            if k in d:raise ValueError('duplicate key')
            d[k]=v
        return d
    x=json.loads(raw,object_pairs_hook=unique)
    if set(x)!={'name','size','mode','mtime','linkTarget'}:raise ValueError('schema')
    if x['name']!=expected_name or type(x['size']) is not int or x['size']<0:raise ValueError('name/size')
    if type(x['mode']) is not int or x['mode']<0 or not isinstance(x['mtime'],str):raise ValueError('mode/mtime')
    if x['linkTarget']!='':raise ValueError('symlink target')
    if x['mode']&~(DIR|0o777) or x['mode']&DIR==0:raise ValueError('not plain directory')
    if x['mode']&0o111!=0o111:raise ValueError('traversal mode bits')
    return {'name':x['name'],'go_mode':x['mode'],'permission_bits':oct(x['mode']&0o777),'uid_gid_observed':False,'acl_observed':False}
