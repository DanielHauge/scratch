from debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends netcat-traditional bash pv && rm -rf /var/lib/apt/lists/*

COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
