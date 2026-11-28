from pathlib import Path
import symtable,builtins,json,sys

def undefined_globals(path):
    st=symtable.symtable(Path(path).read_text(),str(path),'exec')
    bound={s.get_name() for s in st.get_symbols() if s.is_assigned() or s.is_imported() or s.is_namespace()}
    allowed=set(dir(builtins))|{'__name__','__file__','__package__','__spec__','__builtins__','__doc__'}|bound
    errors=set()
    def visit(t):
        for s in t.get_symbols():
            if s.is_referenced() and s.is_global() and s.get_name() not in allowed:errors.add(s.get_name())
        for c in t.get_children():visit(c)
    visit(st)
    return sorted(errors)
if __name__=='__main__':
    print(json.dumps({p:undefined_globals(p)for p in sys.argv[1:]},indent=2))
