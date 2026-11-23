import sqlite3,sys
c=sqlite3.connect('file:/tmp/claude-0/-home-user/9dbbf9e8-7c86-599b-a188-83c0330b1243/scratchpad/kb/cat/catalog.sqlite?mode=ro',uri=True)
def body(repo,path):
    v=c.execute("select version_id,document_id from locations where repo=? and path=? and present=1",(repo,path)).fetchone()
    if not v: return None
    rows=c.execute("select text from passages where version_id=? and is_parent=0 order by chunk_index",(v[0],)).fetchall()
    if not rows: rows=c.execute("select text from passages where version_id=? order by chunk_index",(v[0],)).fetchall()
    return v[1],"\n".join(r[0] for r in rows)
def atlas(i):
    r=c.execute("select path from locations where repo='crypto-autoresearcher' and path like ? and present=1",('knowledge/leads/ecdlp/%/'+i+'.yaml',)).fetchone()
    return (r[0],)+body('crypto-autoresearcher',r[0]) if r else None
if __name__=='__main__':
    for i in sys.argv[1:]:
        a=atlas(i)
        if a: print('=====',a[0],a[1]);print(a[2][:int(__import__('os').environ.get('N','1800'))])
        else: print('=====',i,'NOT IN CORPUS')
