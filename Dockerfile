FROM alpine:latest as certs
RUN apk add --no-cache ca-certificates

FROM scratch
COPY --from=certs /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

ENV GPT_POST=out
COPY target/x86_64-unknown-linux-musl/release/gpt-cli /gpt-cli
ENTRYPOINT ["/gpt-cli"]
