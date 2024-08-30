# HyperBench

HyperBench is a benchmarking tool designed to provide a reasonably objective
performance comparison between different types of container runtime, such as
hypervisor-based runtimes (like Edera Protect), interposer-based runtimes
such as gVisor and namespace-based runtimes like containerd and CRI-o.

To this end, there are two types of benchmark that are included in HyperBench:
raw computational benchmarks which are intended to assess the overhead of the
container runtime on raw computations and benchmarks that are designed to assess
the overhead of the container runtime when processing system calls.

In the future, additional benchmarks such as those exercising CUDA and other
computational acceleration frameworks are planned.

## Running HyperBench

In general you want to use the OCI image:

```
% docker run --rm -it ghcr.io/edera-dev/hyperbench:latest
...
```

However you can also build from source like any other Cargo application.
We do not recommend running debug builds for performance testing, however,
because the performance scores will be highly inaccurate.  Only use
release builds for benchmarking.

