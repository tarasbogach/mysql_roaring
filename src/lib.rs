use roaring::{RoaringBitmap as Map32, RoaringTreemap as Map64};
use num_traits::cast::ToPrimitive;
use udf::prelude::*;
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign};

use macros::{maps_op, map_contains, map_count, map_json};

maps_op!();
map_contains!();
map_count!();
map_json!();


//
// #[derive(Default)]
// struct Roaring32Json {
//     vec: Vec<u8>
// }
//
// #[register]
// impl BasicUdf for Roaring32Json {
//     type Returns<'a> = Option<&'a [u8]>;
//
//     fn init(cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
//         cfg.set_maybe_null(true);
//         if args.len() != 1 {
//             return Err(format!("Expected one arguments; Got {} arguments.", args.len()));
//         }
//         if !args.get(0).unwrap().value().is_string() {
//             return Err(format!("First argument mast be CLOB (bitmap) or NULL."));
//         }
//         Ok(Self::default())
//     }
//
//     fn process<'a>(&'a mut self, _cfg: &UdfCfg<Process>, args: &ArgList<Process>, _error: Option<NonZeroU8>) -> Result<Self::Returns<'a>, ProcessError> {
//         let arg = args.get(0).unwrap().value();
//         let bytes = arg.as_bytes();
//         if let Some(bytes) = bytes {
//             if let Ok(map) = Map32::deserialize_from(bytes) {
//                 let mut count = map.len() as usize;
//                 let capacity = ((10usize+1)*count)+2; // "[" + (log10(2^32) + ",") * map.len() + "]";
//                 let mut vec: Vec<u8> = Vec::with_capacity(capacity);
//                 const COMA: &[u8] = b",";
//                 vec.extend(b"[");
//                 for item in map.iter() {
//                     vec.extend(item.to_string().as_bytes());
//                     count-=1;
//                     if count != 0 {
//                         vec.extend(COMA);
//                     }
//                 }
//                 vec.extend(b"]");
//                 self.vec = vec;
//                 Ok(Some(&self.vec[0..self.vec.len()]))
//             } else {
//                 Err(ProcessError)
//             }
//         } else {
//             Ok(None)
//         }
//     }
// }