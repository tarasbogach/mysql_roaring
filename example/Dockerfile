FROM rust:latest AS build

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

WORKDIR /build

COPY ./mysql_roaring /build

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/target \
    cargo build --release \
    && mkdir /output \
    && cp target/release/libmysql_roaring.so /output \
    && cp libmysql_roaring.sql /output

FROM mariadb

COPY --from=build /output/libmysql_roaring.so /usr/lib/mysql/plugin/
COPY --from=build /output/libmysql_roaring.sql /docker-entrypoint-initdb.d/