* 32 - for INT
* 64 - for BIGINT
* nullsafe - if result is NULL, then return an empty map BLOB
* count - return number of elements in bitmap result, not a bitmap itself


* [ ] roaring[32|64]_[nullsafe_]create ([value0 INT[,value1 INT[, ...]]]) -> BLOB
* [ ] roaring[32|64]_[nullsafe_]insert (map BLOB[, value0 INT[,value1 INT[, ...]]]) -> BLOB
* [ ] roaring[32|64]_[nullsafe_]remove (map BLOB[, value0 INT[,value1 INT[, ...]]]) -> BLOB
* [x] roaring[32|64]_[nullsafe_]contains (map BLOB, value INT) -> BOOL
* [ ] roaring[32|64]_[nullsafe_]count (map BLOB) -> INT
* [ ] roaring[32|64]_[nullsafe_]json (map BLOB) -> LONGTEXT -- JSON Array of Number
* [x] roaring[32|64]_[nullsafe_]and[_count] ([map0 BLOB[, map1 BLOB[, ...]]]) -> BLOB
* [x] roaring[32|64]_[nullsafe_]or[_count] ([map0 BLOB[, map1 BLOB[, ...]]]) -> BLOB
* [x] roaring[32|64]_[nullsafe_]xor[_count] ([map0 BLOB[, map1 BLOB[, ...]]]) -> BLOB
* [ ] roaring[32|64]_[nullsafe_]group_create[_count] (value INT) -> BLOB
* [ ] roaring[32|64]_[nullsafe_]group_and[_count] (map BLOB) -> BLOB
* [ ] roaring[32|64]_[nullsafe_]group_or[_count] (map BLOB) -> BLOB
* [ ] roaring[32|64]_[nullsafe_]group_xor[_count] (map BLOB) -> BLOB
