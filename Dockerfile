FROM rust:1.82.0-alpine as build
WORKDIR /app
COPY ./ /app/

RUN sed -i s/dl-cdn.alpinelinux.org/mirror.tuna.tsinghua.edu.cn/g  /etc/apk/repositories  && \
    apk add openssl-dev openssl-libs-static musl-dev upx

RUN echo [source.crates-io] > cargo.config
RUN echo 'replace-with = "ustc"' >> cargo.config
RUN echo [source.ustc] >> cargo.config
RUN echo 'registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"' >> cargo.config

RUN cargo --config cargo.config install  --path api --root /tmp/app
RUN cargo --config cargo.config install  --path migration --root /tmp/app

RUN strip -s /tmp/app/bin/storage_api 
RUN strip  --strip-debug /tmp/app/bin/storage_api 
RUN upx /tmp/app/bin/storage_api

RUN strip -s /tmp/app/bin/migration
RUN strip  --strip-debug  /tmp/app/bin/migration 
RUN upx /tmp/app/bin/migration

FROM alpine:3.20.3

WORKDIR /app

COPY --from=build /tmp/app/bin/storage_api /app/bin/storage_api
COPY --from=build /tmp/app/bin/migration /app/bin/migration

# RUN sed -i s/dl-cdn.alpinelinux.org/mirror.tuna.tsinghua.edu.cn/g  /etc/apk/repositories  && \
# apk add --no-cache tzdata

RUN apk add --no-cache tzdata

EXPOSE 8080

# ENTRYPOINT /app/note
CMD /app/bin/storage_api
