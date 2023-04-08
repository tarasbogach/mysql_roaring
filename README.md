# Roaring bitmaps as MySQL/MariaDB User Defined Functions

| This work is complete, but not tested yet!

## Storage
* You can use `LONGBLOB` data type to store bitmap in MySQL/MariaDB column.
* Please remember that if you're using database replication,
you should probably exclude tables with bitmaps from it or switch to statement-based replication.
Otherwise, the size of your binary log will significantly increase,
and the performance of the database will be slightly slower.

## UDF functions

### prefixes

* `roaring32` - for INT (maybe unsigned)
* `roaring64` - for BIGINT (maybe unsigned)
* `nullsafe` - SQL functions return NULL if at least one argument is NULL.
With the 'nullsafe' prefix, the function will return an empty result of the corresponding type, instead of NULL.
  * empty bitmap BLOB
  * `0` for integer
  * `'[]'` for json

### functions

* `roaring[32|64]_[nullsafe_]create(value0 INT,[value1 INT[, ...]]) -> BLOB` - Create bitmap from multiple integer arguments.
* `roaring[32|64]_[nullsafe_][remove|insert](map BLOB, value0 INT[,value1 INT[, ...]]) -> BLOB` - Add or remove multiple integers from bitmap.
* `roaring[32|64]_[nullsafe_]contains (map BLOB, value INT) -> BOOL` - Check whether integer is in a bitmap.
* `roaring[32|64]_[nullsafe_]count (map BLOB) -> INT` - How many integers are in a bitmap.
* `roaring[32|64]_[nullsafe_]json (map BLOB) -> TEXT` - Get all integers from bitmap as JSON array.
* `roaring[32|64]_[nullsafe_][and|or|xor] ([map0 BLOB[, map1 BLOB[, ...]]]) -> BLOB` - Perform one of bitmap operations (and, or, xor) on multiple bitmap arguments and return resulting bitmap.
* `roaring[32|64]_[nullsafe_][and|or|xor]_count ([map0 BLOB[, map1 BLOB[, ...]]]) -> BLOB` - Perform one of bitmap operations (and, or, xor) on multiple bitmap arguments and return number of integers in a resulting bitmap.
* `roaring[32|64]_[nullsafe_]group_create (value INT) -> BLOB` - Aggregation. Create bitmap from integers is group.
* `roaring[32|64]_[nullsafe_]group_[and|or|xor] (map BLOB) -> BLOB` - Aggregation. Perform one of bitmap operations (and, or, xor) on bitmaps in group and return resulting bitmap.
* `roaring[32|64]_[nullsafe_]group_[and|or|xor]_count (map BLOB) -> BLOB` - Aggregation. Perform one of bitmap operations (and, or, xor) on bitmaps in group and return number of integers in a resulting bitmap.

### full list of functions
[libmysql_roaring.md](libmysql_roaring.md)

### SQL queries for all functions loading
[libmysql_roaring.sql](libmysql_roaring.sql)

## Build and installation

### Standalone MySQL/MariaDB

* Get Rust programing language tools https://www.rust-lang.org/tools/install
* Get this git repository `git clone --depth=1 https://github.com/tarasbogach/mysql_roaring.git`
* Go to project folder `cd mysql_roaring`
* Build this project `cargo build --release`
* Now you should have compiled shared library `./target/release/libmysql_roaring.so`
* Find your MySQL/MariaDB plugin folder `echo "SHOW VARIABLES LIKE 'plugin_dir';" | mysql`,
and copy shared library to it `sudo cp ./target/release/libmysql_roaring.so /usr/lib/mysql/plugin/`
* Run queries from [libmysql_roaring.sql](libmysql_roaring.sql) `cat libmysql_roaring.sql | mysql`

### Docker

You will need to add `build` stage to your Dockerfile.
And copy `.so` and `.sql` files from it to your MySQL/MariaDB image.

```Dockerfile
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
```