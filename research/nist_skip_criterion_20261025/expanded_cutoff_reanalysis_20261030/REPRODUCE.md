# Reproduce the exact local reanalysis

Use RUNPLAN.md for classification, fixed definitions and expected outcomes.
No AWS access, new point count or random-curve sample is used by this loop.
The initial source/input bindings remain in SOURCES-SHA256SUMS.

From the repository root, with C++17,GMP and PARI headers/libraries installed:

    sh research/nist_skip_criterion_20261025/expanded_cutoff_reanalysis_20261030/build.sh /absolute/fresh-build
    sh research/nist_skip_criterion_20261025/expanded_cutoff_reanalysis_20261030/run.sh /absolute/fresh-build /absolute/fresh-run

PARI_ROOT may select the installation prefix. The recorded build used the
versioned library libpari-gmp-tls.so.9. Rebuilding may produce different
binary identities; retain them rather than claiming the recorded hashes.
The five mathematical table/summary files must match the verified originals.

For native descriptive tables and figure data:

    g++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic research/nist_skip_criterion_20261025/expanded_cutoff_reanalysis_20261030/summarize.cpp -o /absolute/summary
    /absolute/summary /absolute/fresh-run research/nist_skip_criterion_20261025/expanded_cutoff_reanalysis_20261030/inputs.tsv /absolute/fresh-derived

The reducer requires the independent verification and complete phase record.
It recomputes all coverage and label totals from the raw tables. The
supplementary2--5000strict count is the union of the recorded earlier and
requested family flags. No additional character computation is needed.

For document layout only, Python with PyMuPDF is used:

    python3 research/nist_skip_criterion_20261025/expanded_cutoff_reanalysis_20261030/render.py /absolute/fresh-derived papers/nist-expanded-cutoff-20261030
    make -C papers/nist-expanded-cutoff-20261030 check

Python only copies native tables and converts SVG to PDF; it computes no
curve feature, calibration, coverage or percentile. The native code supplies
all graph data and coordinates. No sampling-error bars are applicable to
the exact finite cohort. The graph selection rule is in figure-data.json;
all4901integer positions are preserved in the complete raw archive.

The archive's MEMBERS.json binds every included file. Verify it after a
fresh extraction before using restored data. DELIVERY.json distinguishes
the local download from unavailable remote custody; no fabricated S3 URL
or receipt is provided.
