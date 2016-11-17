FROM ubuntu:16.04

RUN apt-get update && apt-get install -y --no-install-recommends \
  g++ \
  make \
  file \
  curl \
  ca-certificates \
  python2.7 \
  git \
  cmake \
  ccache \
  sudo \
  gdb \
  antlr4 \
  bison \
  flex

ENV RUST_CONFIGURE_ARGS --build=x86_64-unknown-linux-gnu
ENV RUST_CHECK_TARGET check-grammar
RUN mkdir /tmp/obj
RUN chmod 777 /tmp/obj
