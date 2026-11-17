# Constructed-kernel binary adapter: native development check

Direct rustc compilation links the existing verified974f2e9cd46b library rlib;
its exact SHA256, library source tree and component sourcec0d2b94f1 are recorded.
The optimized native test executable passes14tests:13core plus the independent
seven-bit binary-curve/reference fixture. That fixture enumerates its full
ambient domain, constructs a kernel from supplied points, compares the census,
and checks all target-offset/kernel pairs and signed relation recovery against
independent binary arithmetic. The ordinary Cargo integration gate remains
separate and pending; this is not a full library or hosted-CI pass.

This is software API correctness on the existing small fixture. No actual E0
kernel/order, scaling result, random sampler or IC speed/cold-price claim is
made. The source notes preserve the standard finite-group assumptions and
missing full-pipeline obligations. The frozen scientific run is unchanged.
