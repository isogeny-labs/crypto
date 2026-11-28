import sqlite3,sys,re
c=sqlite3.connect('file:/tmp/claude-0/-home-user/9dbbf9e8-7c86-599b-a188-83c0330b1243/scratchpad/kb/cat/catalog.sqlite?mode=ro',uri=True)
def search(q,n=6,repo=None,snip=160):
    sql="""select d.id,d.title,l.repo,l.path,p.id,snippet(passage_fts,1,'[',']','…',24),bm25(passage_fts)
    from passage_fts join passages p on p.rowid=passage_fts.rowid join versions v on v.id=p.version_id
    join documents d on d.id=v.document_id join locations l on l.document_id=d.id and l.version_id=v.id and l.present=1
    where passage_fts match ? %s order by bm25(passage_fts) limit ?"""%("and l.repo=?" if repo else "")
    a=[q]+([repo] if repo else [])+[n]
    return c.execute(sql,a).fetchall()
if __name__=='__main__':
    q=sys.argv[1]; n=int(sys.argv[2]) if len(sys.argv)>2 else 6
    for r in search(q,n): print(r[0][:12],'|',r[2],r[3],'|',r[5].replace('\n',' '))
