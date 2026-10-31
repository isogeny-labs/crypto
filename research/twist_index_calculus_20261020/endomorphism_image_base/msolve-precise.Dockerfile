FROM twist-image-msolve:0.11.0
COPY msolve-precision.patch /opt/msolve-precision.patch
WORKDIR /opt/msolve
RUN git apply --check /opt/msolve-precision.patch && git apply /opt/msolve-precision.patch && \
    make -j4 && make check && make install && ldconfig
WORKDIR /work
