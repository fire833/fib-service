
# Copyright (C) 2026 Kendall Tauser
#
# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 2 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License along
# with this program; if not, write to the Free Software Foundation, Inc.,
# 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

FROM docker.io/rust@sha256:e259494b14672950a909b4c54527e19d29cae6ebf980672114c8bb85e5594e4a AS builder

ADD src /usr/src/fib/src
ADD Cargo.lock /usr/src/fib
ADD Cargo.toml /usr/src/fib
WORKDIR /usr/src/fib

RUN wget -O /usr/local/bin/dumb-init https://static.tauser.us/init/dumb-init-x86 && chmod +x /usr/local/bin/dumb-init

RUN cargo build --release

FROM docker.io/ubuntu@sha256:93e29beff5257b2cdf1c024d831f6d2997e53655a653ff76a811fc9e6564a719

COPY --from=builder /usr/local/bin/dumb-init /usr/local/bin/dumb-init
COPY --from=builder /usr/src/fib/target/release/fib-service /usr/local/bin/fib-service
USER 1000:1000

LABEL org.opencontainers.image.author="Kendall Tauser"
LABEL org.opencontainers.image.title="Fib-service - A toy API for serving the Fibonacci numbers."
LABEL org.opencontainers.image.source="https://github.com/fire833/fib-service"

CMD [ "dumb-init", "fib-service" ]
