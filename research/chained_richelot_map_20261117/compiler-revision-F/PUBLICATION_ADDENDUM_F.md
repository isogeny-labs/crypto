# F: unchanged E scientific source, packed raw publication diagnostics

E's source layout and all scientific input bytes are retained exactly in F.
The original E lock and prospective record retain their original paths and
the whole E source/diff/format-negative packet is archived byte-for-byte.
Raw diff context blank lines and the original rejected layout have trailing
whitespace, so those bytes are packaged unchanged rather than changed to pass
Git's whitespace gate. F excludes only that metadata diff and a Python bytecode
cache from its execution closure; the caller selects F's lock. Source methods,
test bodies, input fixtures, sign formula and every acceptance/limit remain
the same. No mathematical run occurred in E or F preparation. [pcc:unverified]
