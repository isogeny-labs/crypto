CXX ?= c++
NVCC ?= nvcc
CXXFLAGS ?= -O2 -std=c++17 -Wall -Wextra -Werror
NVCCFLAGS ?= -O2 -std=c++17 -arch=sm_120

.PHONY: all host-test cuda-test clean

all: host-test

test_host: test_host.cpp certicom131.cuh verify.cuh fixtures.hpp params131.h vectors131.h
	$(CXX) $(CXXFLAGS) -o $@ test_host.cpp

host-test: test_host
	./test_host

test_cuda: test_cuda.cu certicom131.cuh verify.cuh fixtures.hpp params131.h vectors131.h
	$(NVCC) $(NVCCFLAGS) -o $@ test_cuda.cu

cuda-test: test_cuda
	./test_cuda

clean:
	rm -f test_host test_cuda
