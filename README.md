# Roaring bitmaps as MySQL/MariaDB User Defined Functions

**This is work in progress!**

## Prefixes and suffixes:

* roaring32 - for INT
* roaring64 - for BIGINT
* nullsafe_ - return an empty bitmap BLOB or 0 or "[]" insted of NULL
* _count    - return number of elements in bitmap result, not a bitmap itself

## Checklist:

* [ ]  4 roaring[32|64]_[nullsafe_?]create ([value0 INT[,value1 INT[, ...]]]) -> BLOB
* [ ]  4 roaring[32|64]_[nullsafe_?]insert (map BLOB[, value0 INT[,value1 INT[, ...]]]) -> BLOB
* [ ]  4 roaring[32|64]_[nullsafe_?]remove (map BLOB[, value0 INT[,value1 INT[, ...]]]) -> BLOB
* [x]  4 roaring[32|64]_[nullsafe_?]contains (map BLOB, value INT) -> BOOL
* [x]  4 roaring[32|64]_[nullsafe_?]count (map BLOB) -> INT
* [x]  4 roaring[32|64]_[nullsafe_?]json (map BLOB) -> LONGTEXT -- JSON Array of Number
* [x] 24 roaring[32|64]_[nullsafe_?][and|or|xor][_count?] ([map0 BLOB[, map1 BLOB[, ...]]]) -> BLOB
* [ ]  8 roaring[32|64]_[nullsafe_?]group_create[_count?] (value INT) -> BLOB
* [ ]  8 roaring[32|64]_[nullsafe_?]group_[and|or|xor][_count?] (map BLOB) -> BLOB
