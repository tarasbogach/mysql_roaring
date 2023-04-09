# Roaring bitmaps as MySQL/MariaDB User Defined Functions

> This development is completed, but only partially tested. Please use it at your own risk.

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

## Examples

### 32bit integer

```sql
CREATE DATABASE IF NOT EXISTS example;

CREATE TABLE IF NOT EXISTS example.bitmaps (id INT8 UNSIGNED PRIMARY KEY, map LONGBLOB);

TRUNCATE TABLE example.bitmaps;

INSERT INTO  example.bitmaps
SELECT seq % 10 as id, roaring32_group_create(CAST(RAND() * POW(2, 31) AS INTEGER)) as map
FROM mysql.seq_1_to_1000000 GROUP BY seq % 10;

SELECT roaring32_count(map) FROM example.bitmaps;

SELECT roaring32_group_or_count(map) FROM example.bitmaps;
```

### 64bit integer

> Please note that the 64-bit version may be significantly (6.5x in some my cases) slower than the 32-bit version.

```sql
CREATE DATABASE IF NOT EXISTS example;

CREATE TABLE IF NOT EXISTS example.bitmaps (id INT8 UNSIGNED PRIMARY KEY, map LONGBLOB);

TRUNCATE TABLE example.bitmaps;

INSERT INTO  example.bitmaps
SELECT seq % 10 as id, roaring64_group_create(CAST(RAND() * POW(2, 63) AS INTEGER)) as map
FROM mysql.seq_1_to_1000000 GROUP BY seq % 10;

SELECT roaring64_count(map) FROM example.bitmaps;

SELECT roaring64_group_or_count(map) FROM example.bitmaps;
```

### Funding

In case you want to thank the author:

* USDT TRC-20 TPWUrwAdktRziAFjb7ARqhs98S1kYa3ank
* USDT ERC-20 0xdE1a4c9024b55F93D7FD34FbA488a172DF8e4cA3
* BTC bc1qyjxf2enwygtsptel0qd9zkn50fh25r0m0qsm8s
* ETH 0x95Bd0C1438A9f6300b40Fc5f1B61cEaBC89F0dA5
* DOGE DGS1fLGKFekVWW1v2QDPvpT3i2EoxLqf7
