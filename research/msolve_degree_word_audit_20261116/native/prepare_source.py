"""Modify source text and record exact file provenance; no mathematical computation."""
from pathlib import Path
import difflib, hashlib, json, sys

root = Path(sys.argv[1]).resolve()
original = {}
for name in ['src/neogb/data.h', 'src/neogb/hash.c', 'src/neogb/symbol.c', 'src/neogb/la_ff_32.c']:
    original[name] = (root / name).read_text()
assert 'typedef uint16_t exp_t;' in original['src/neogb/data.h']

name = 'src/neogb/hash.c'
text = original[name]
assert text.count('res |= 1 << ctr;') == 1
text = text.replace('res |= 1 << ctr;', 'res |= 1U << ctr;')
start = text.index('static inline hi_t get_lcm(')
end = text.index('static inline hm_t *poly_to_matrix_row(', start)
part = text[start:end]
needle = '#if PARALLEL_HASHING\n'
assert part.count(needle) == 1
logging = r'''    /* Degree-word audit: observations only, no algorithmic branch changes. */
    if (evl == 3 && ebl == 0) {
        static unsigned audit_lcm_count = 0;
        if (audit_lcm_count < 1024) {
            fprintf(stderr, "WORD_LCM %u %u %u %u %u %u %u %llu\n",
                    (unsigned)ea[1], (unsigned)ea[2],
                    (unsigned)eb[1], (unsigned)eb[2],
                    (unsigned)etmp[1], (unsigned)etmp[2], (unsigned)etmp[0],
                    (unsigned long long)etmp[1] + (unsigned long long)etmp[2]);
        } else if (audit_lcm_count == 1024) {
            fprintf(stderr, "WORD_LOG_CAP lcm\n");
        }
        ++audit_lcm_count;
    }
'''
text = text[:start] + part.replace(needle, logging + needle) + text[end:]
start = text.index('static inline hm_t *multiplied_poly_to_matrix_row(')
part = text[start:]
needle = '  insert_multiplied_poly_in_hash_table(row, hm, em, poly, bht, sht);\n'
assert part.count(needle) == 1
logging = r'''  if (sht->evl == 3 && sht->ebl == 0) {
    static unsigned audit_row_count = 0;
    if (audit_row_count < 1024) {
      unsigned max_word = 0;
      unsigned long long max_degree = 0;
      for (len_t audit_j = OFFSET; audit_j < row[LENGTH] + OFFSET; ++audit_j) {
        const exp_t *audit_e = sht->ev[row[audit_j]];
        const unsigned long long audit_degree =
            (unsigned long long)audit_e[1] + (unsigned long long)audit_e[2];
        if ((unsigned)audit_e[0] > max_word) max_word = (unsigned)audit_e[0];
        if (audit_degree > max_degree) max_degree = audit_degree;
      }
      fprintf(stderr, "WORD_ROW %u %u %llu %u\n",
              (unsigned)sht->ev[row[OFFSET]][0], max_word, max_degree,
              (unsigned)row[LENGTH]);
    } else if (audit_row_count == 1024) {
      fprintf(stderr, "WORD_LOG_CAP row\n");
    }
    ++audit_row_count;
  }
'''
text = text[:start] + part.replace(needle, needle + logging)
(root / name).write_text(text)

name = 'src/neogb/symbol.c'
text = original[name]
start = text.index('static int32_t select_spairs_by_minimal_degree(')
end = text.index('static inline void find_multiplied_reducer(', start)
part = text[start:end]
needle = '    mdeg  = ps[0].deg;\n'
assert part.count(needle) == 1
logging = r'''    {
        static unsigned audit_select_count = 0;
        if (audit_select_count < 1024) {
            fprintf(stderr, "WORD_SELECT %u %u\n", (unsigned)mdeg,
                    (unsigned)md->max_gb_degree);
        } else if (audit_select_count == 1024) {
            fprintf(stderr, "WORD_LOG_CAP select\n");
        }
        ++audit_select_count;
    }
'''
text = text[:start] + part.replace(needle, needle + logging) + text[end:]
(root / name).write_text(text)

name = 'src/neogb/la_ff_32.c'
text = original[name]
start = text.index('static hm_t *reduce_dense_row_by_known_pivots_sparse_17_bit(')
end = text.index('static hm_t *trace_reduce_dense_row_by_known_pivots_sparse_17_bit(', start)
part = text[start:end]
assert part.count('bs->cf_32[dts[COEFFS]]') == 1
part = part.replace('bs->cf_32[dts[COEFFS]]', 'mat->cf_32[dts[COEFFS]]')
text = text[:start] + part + text[end:]
(root / name).write_text(text)

patch = []
files = []
for name, before in original.items():
    after = (root / name).read_text()
    patch.extend(difflib.unified_diff(before.splitlines(True), after.splitlines(True),
                                   fromfile='a/' + name, tofile='b/' + name))
    files.append(dict(path=name, original_sha256=hashlib.sha256(before.encode()).hexdigest(),
                      patched_sha256=hashlib.sha256(after.encode()).hexdigest(),
                      original_bytes=len(before.encode()), patched_bytes=len(after.encode())))
(root / 'degree-audit.patch').write_text(''.join(patch))
print(json.dumps(dict(source_commit='5e72b9d99eaea55fe87eb8ac945ec9e914a69327',
    files=files, variant='known matrix-table backport, unsigned mask, bounded read-only degree logging',
    exponent_type_unchanged=True, lcm_and_selection_logic_unchanged=True), indent=2))
