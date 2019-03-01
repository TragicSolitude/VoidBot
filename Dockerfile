FROM debian:stretch-slim
WORKDIR /opt/voidbot
RUN apt update && \
    apt install --no-install-recommends -y libssl-dev ca-certificates && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*
COPY target/release/voidbot ./
CMD ["/opt/voidbot/voidbot"]