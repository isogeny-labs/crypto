# Source data

`../tian_data.json` contains numerical mathematical data from Section 3.4 of
Song Tian's arXiv v1 preprint, https://arxiv.org/abs/2012.07173 . It records the
SHA-256 digest of the source TeX used to extract the coefficient arrays.

The normal experiment runner uses this JSON offline. To regenerate it:

```sh
curl -L https://arxiv.org/src/2012.07173 -o sources/tian_source.tar
tar -xOf sources/tian_source.tar Cover_attack.tex > sources/tian.tex
sage -python extract_tian_data.py
```

The remaining sources are linked in the main README and the generated report.
Downloaded papers and temporary research files are omitted from the experiment
archive. No researcher-provided executable implementation is included.
