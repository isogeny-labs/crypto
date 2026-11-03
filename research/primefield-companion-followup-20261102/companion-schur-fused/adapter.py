"""Unexecuted integration adapter; frozen historical Schur source is unchanged.

Caller places companion-schur, companion-blocks, companion-fused and
structured-block-kernel on the import path, as specified in the manifest.
"""
from schur import Schur,LeadingBlockObstruction
from multiply import apply as fused_apply

class FusedSchur(Schur):
    """Use the deferred-reduction kernel for each charged source application."""
    def _source(self,x,transpose=False):
        self.m.add('source_transpose_calls' if transpose else 'source_forward_calls')
        return fused_apply(self.G,self.K,self.a,self.b,x,self.m,transpose)
