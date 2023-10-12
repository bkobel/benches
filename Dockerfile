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

WORKDIR /tools
RUN git clone https://chromium.googlesource.com/chromium/tools/depot_tools.git
ENV PATH "$PATH:/tools/depot_tools"

WORKDIR /src
RUN fetch --nohooks --nohistory chromium && gclient runhooks

RUN echo 'is_debug = false\n\
symbol_level = 0\n\
is_component_build = false' > out/Default/args.gn
