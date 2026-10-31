TEX      := paper.tex
SECTIONS := tables/all-candidates.tex tables/p192-sensitivity.tex
BIB      := refs.bib
EXTRA    := abstract.txt README.md Makefile prepare_assets.py inspection.json validation/local-records.json
ASSETS   := $(wildcard figures/*.pdf figures/*.svg data/* reproduction/*)
PAPER_MK := $(firstword $(wildcard ../paper.mk paper.mk))
include $(PAPER_MK)
