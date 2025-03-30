# Start with the official Rust image
FROM rust:latest

# Install necessary tools for building GLIBC
RUN apt update && apt install -y build-essential manpages-dev wget

# Download and compile GLIBC 2.38
RUN wget http://ftp.gnu.org/gnu/libc/glibc-2.38.tar.gz && \
    tar -xvzf glibc-2.38.tar.gz && \
    cd glibc-2.38 && \
    mkdir build && \
    cd build && \
    ../configure --prefix=/opt/glibc-2.38 && \
    make -j$(nproc) && \
    make install && \
    cd / && \
    rm -rf glibc-2.38*

# Set the environment variable to use the new GLIBC
ENV LD_LIBRARY_PATH=/opt/glibc-2.38/lib:$LD_LIBRARY_PATH
