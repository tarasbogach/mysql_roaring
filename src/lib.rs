use roaring::{RoaringBitmap as Map32, RoaringTreemap as Map64};
use num_traits::cast::ToPrimitive;
use udf::prelude::*;
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign};

use macros::{create, map_op, map_int_op, contains, count, json, group_create, group_map_op};

create!();
map_op!();
map_int_op!();
contains!();
count!();
json!();
group_create!();
group_map_op!();