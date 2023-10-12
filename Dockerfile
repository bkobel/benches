FROM ubuntu:latest

# Prevents dpkg errors
ENV TERM=xterm-256color

# Install necessary dependencies for building Chromium
RUN apt-get update && apt-get upgrade -y && apt-get install -y \
    git \
    python3 \
    curl \
    clang \
    lld \
    ninja-build \
    g++ \
    cmake \
    python2

WORKDIR /src

RUN cd /src && git clone https://chromium.googlesource.com/chromium/tools/depot_tools.git
ENV PATH "$PATH:/src/depot_tools"
RUN fetch --nohooks chromium && gclient runhooks

RUN echo 'is_debug = false\n\
symbol_level = 0\n\
is_component_build = false' > out/Default/args.gn
