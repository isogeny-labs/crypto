TEX := portfolio.tex
SECTIONS := $(wildcard sections/*.tex)
BIB := refs.bib
ASSETS := $(wildcard figures/*.pdf) figures/recorded-data.json evidence/sources.json
include ../paper.mk

.PHONY: figures review-check
figures:
	python3 figures/render.py
	python3 figures/render-expanded.py

review-check:
	python3 tools/check.py
