use roaring::{RoaringBitmap as Map32, RoaringTreemap as Map64};
use num_traits::cast::ToPrimitive;
use udf::prelude::*;
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign};

use macros::{ints_create, maps_op, map_ints_op, map_contains, map_count, map_json, ints_group_create, maps_group_op};

ints_create!();
maps_op!();
map_ints_op!();
map_contains!();
map_count!();
map_json!();
ints_group_create!();
maps_group_op!();



