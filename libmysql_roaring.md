* roaring32_create(value INT, ...) BLOB|NULL
* roaring32_remove(map BLOB, value INT, ...) BLOB|NULL
* roaring32_insert(map BLOB, value INT, ...) BLOB|NULL
* roaring32_contains(map BLOB, value INT) BOOL|NULL
* roaring32_count(map BLOB) INT|NULL
* roaring32_json(map BLOB) LONGTEXT|NULL
* roaring32_and(map BLOB, map BLOB, ...) BLOB|NULL
* roaring32_or(map BLOB, map BLOB, ...) BLOB|NULL
* roaring32_xor(map BLOB, map BLOB, ...) BLOB|NULL
* roaring32_and_count(map BLOB, map BLOB, ...) INT|NULL
* roaring32_or_count(map BLOB, map BLOB, ...) INT|NULL
* roaring32_xor_count(map BLOB, map BLOB, ...) INT|NULL
* roaring32_group_create(value INT) BLOB|NULL
* roaring32_group_and(map BLOB) BLOB|NULL
* roaring32_group_or(map BLOB) BLOB|NULL
* roaring32_group_xor(map BLOB) BLOB|NULL
* roaring32_group_and_count(map BLOB) INT|NULL
* roaring32_group_or_count(map BLOB) INT|NULL
* roaring32_group_xor_count(map BLOB) INT|NULL
* roaring64_create(value BIGINT, ...) LONGBLOB|NULL
* roaring64_remove(map LONGBLOB, value BIGINT, ...) LONGBLOB|NULL
* roaring64_insert(map LONGBLOB, value BIGINT, ...) LONGBLOB|NULL
* roaring64_contains(map LONGBLOB, value BIGINT) BOOL|NULL
* roaring64_count(map LONGBLOB) BIGINT|NULL
* roaring64_json(map LONGBLOB) LONGTEXT|NULL
* roaring64_and(map LONGBLOB, map LONGBLOB, ...) LONGBLOB|NULL
* roaring64_or(map LONGBLOB, map LONGBLOB, ...) LONGBLOB|NULL
* roaring64_xor(map LONGBLOB, map LONGBLOB, ...) LONGBLOB|NULL
* roaring64_and_count(map LONGBLOB, map LONGBLOB, ...) BIGINT|NULL
* roaring64_or_count(map LONGBLOB, map LONGBLOB, ...) BIGINT|NULL
* roaring64_xor_count(map LONGBLOB, map LONGBLOB, ...) BIGINT|NULL
* roaring64_group_create(value BIGINT) LONGBLOB|NULL
* roaring64_group_and(map LONGBLOB) LONGBLOB|NULL
* roaring64_group_or(map LONGBLOB) LONGBLOB|NULL
* roaring64_group_xor(map LONGBLOB) LONGBLOB|NULL
* roaring64_group_and_count(map LONGBLOB) BIGINT|NULL
* roaring64_group_or_count(map LONGBLOB) BIGINT|NULL
* roaring64_group_xor_count(map LONGBLOB) BIGINT|NULL
* roaring32_nullsafe_create(value INT, ...) BLOB
* roaring32_nullsafe_remove(map BLOB, value INT, ...) BLOB
* roaring32_nullsafe_insert(map BLOB, value INT, ...) BLOB
* roaring32_nullsafe_contains(map BLOB, value INT) BOOL
* roaring32_nullsafe_count(map BLOB) INT
* roaring32_nullsafe_json(map BLOB) LONGTEXT
* roaring32_nullsafe_and(map BLOB, map BLOB, ...) BLOB
* roaring32_nullsafe_or(map BLOB, map BLOB, ...) BLOB
* roaring32_nullsafe_xor(map BLOB, map BLOB, ...) BLOB
* roaring32_nullsafe_and_count(map BLOB, map BLOB, ...) INT
* roaring32_nullsafe_or_count(map BLOB, map BLOB, ...) INT
* roaring32_nullsafe_xor_count(map BLOB, map BLOB, ...) INT
* roaring32_nullsafe_group_create(value INT) BLOB
* roaring32_nullsafe_group_and(map BLOB) BLOB
* roaring32_nullsafe_group_or(map BLOB) BLOB
* roaring32_nullsafe_group_xor(map BLOB) BLOB
* roaring32_nullsafe_group_and_count(map BLOB) INT
* roaring32_nullsafe_group_or_count(map BLOB) INT
* roaring32_nullsafe_group_xor_count(map BLOB) INT
* roaring64_nullsafe_create(value BIGINT, ...) LONGBLOB
* roaring64_nullsafe_remove(map LONGBLOB, value BIGINT, ...) LONGBLOB
* roaring64_nullsafe_insert(map LONGBLOB, value BIGINT, ...) LONGBLOB
* roaring64_nullsafe_contains(map LONGBLOB, value BIGINT) BOOL
* roaring64_nullsafe_count(map LONGBLOB) BIGINT
* roaring64_nullsafe_json(map LONGBLOB) LONGTEXT
* roaring64_nullsafe_and(map LONGBLOB, map LONGBLOB, ...) LONGBLOB
* roaring64_nullsafe_or(map LONGBLOB, map LONGBLOB, ...) LONGBLOB
* roaring64_nullsafe_xor(map LONGBLOB, map LONGBLOB, ...) LONGBLOB
* roaring64_nullsafe_and_count(map LONGBLOB, map LONGBLOB, ...) BIGINT
* roaring64_nullsafe_or_count(map LONGBLOB, map LONGBLOB, ...) BIGINT
* roaring64_nullsafe_xor_count(map LONGBLOB, map LONGBLOB, ...) BIGINT
* roaring64_nullsafe_group_create(value BIGINT) LONGBLOB
* roaring64_nullsafe_group_and(map LONGBLOB) LONGBLOB
* roaring64_nullsafe_group_or(map LONGBLOB) LONGBLOB
* roaring64_nullsafe_group_xor(map LONGBLOB) LONGBLOB
* roaring64_nullsafe_group_and_count(map LONGBLOB) BIGINT
* roaring64_nullsafe_group_or_count(map LONGBLOB) BIGINT
* roaring64_nullsafe_group_xor_count(map LONGBLOB) BIGINT