#!/usr/bin/env python3
"""Render REPORT.typ to REPORT.pdf (document rendering only; needs the `typst` wheel)."""
import pathlib, typst
d = pathlib.Path(__file__).resolve().parent.parent
typst.compile(str(d / "REPORT.typ"), output=str(d / "REPORT.pdf"))
