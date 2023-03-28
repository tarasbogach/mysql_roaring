```
roaring32_ prefix - for INT
roaring64_ prefix - for BIGINT
nullsafe_ prefix - if result is NULL, then return an empty map BLOB
_count suffix - return number of elements in bitmap result, not a bitmap itself

roaring[32|64][_nullsafe]_create[_count]([value0 INT[,value1 INT[, ...]]]) -> BLOB
roaring[32|64][_nullsafe]_insert[_count](map BLOB[, value0 INT[,value1 INT[, ...]]]) -> BLOB
roaring[32|64][_nullsafe]_remove[_count](map BLOB[, value0 INT[,value1 INT[, ...]]]) -> BLOB
roaring[32|64][_nullsafe]_and[_count]([map0 BLOB[, map1 BLOB[, ...]]]) -> BLOB
roaring[32|64][_nullsafe]_or[_count]([map0 BLOB[, map1 BLOB[, ...]]]) -> BLOB
roaring[32|64][_nullsafe]_xor[_count]([map0 BLOB[, map1 BLOB[, ...]]]) -> BLOB
roaring[32|64][_nullsafe]_group_create[_count](value INT) -> BLOB
roaring[32|64][_nullsafe]_group_and[_count](map BLOB) -> BLOB
roaring[32|64][_nullsafe]_group_or[_count](map BLOB) -> BLOB
roaring[32|64][_nullsafe]_group_xor[_count](map BLOB) -> BLOB

roaring[32|64][_nullsafe]_contains(map BLOB, value INT) -> BOOL
roaring[32|64][_nullsafe]_count(map BLOB) -> INT
roaring[32|64][_nullsafe]_json(map BLOB) -> LONGTEXT -- JSON Array of Number

```