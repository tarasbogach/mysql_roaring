use roaring::{RoaringBitmap as Map32, RoaringTreemap as Map64};
use num_traits::cast::ToPrimitive;
use udf::prelude::*;
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign};

use macros::{ints_create, maps_op, map_ints_op, map_contains, map_count, map_json};

ints_create!();
maps_op!();
map_ints_op!();
map_contains!();
map_count!();
map_json!();


#[derive(Default)]
struct Roaring32AggCreate {
    map: Map32,
    vec: Vec<u8>
}

#[register]
impl AggregateUdf for Roaring32AggCreate {
    fn clear(&mut self, _cfg: &UdfCfg<Process>, _error: Option<NonZeroU8>) -> Result<(), NonZeroU8> {
        self.vec.clear();
        self.map.clear();
        Ok(())
    }

    fn add(&mut self, _cfg: &UdfCfg<Process>, args: &ArgList<Process>, _error: Option<NonZeroU8>) -> Result<(), NonZeroU8> {
        let value = args.get(0).unwrap().value().as_int();
        if let Some(value) = value {
            if let Some(value) = value.to_u32() {
                self.map.insert(value);
                Ok(())
            } else {
                Err(NonZeroU8::new(1u8).unwrap())
            }
        } else {
            Ok(())
        }
    }

    fn remove(&mut self, _cfg: &UdfCfg<Process>, args: &ArgList<Process>, _error: Option<NonZeroU8>) -> Result<(), NonZeroU8> {
        let value = args.get(0).unwrap().value().as_int();
        if let Some(value) = value {
            if let Some(value) = value.to_u32() {
                self.map.remove(value);
                Ok(())
            } else {
                Err(NonZeroU8::new(1u8).unwrap())
            }
        } else {
            Ok(())
        }
    }
}
#[register]
impl BasicUdf for Roaring32AggCreate {
    type Returns<'a> = Option<&'a [u8]>;

    fn init(cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        cfg.set_maybe_null(true);
        if args.len() != 1 {
            return Err(format!("Expected one arguments; Got {} arguments.", args.len()));
        }
        if !args.get(0).unwrap().value().is_int() {
            return Err(format!("{} argument mast be INTEGER or NULL.", 0));
        }
        Ok(Self::default())
    }

    fn process<'a>(&'a mut self, _cfg: &UdfCfg<Process>, _args: &ArgList<Process>, _error: Option<NonZeroU8>) -> Result<Self::Returns<'a>, ProcessError> {
        if self.map.len() == 0 {
            return Ok(None);
        }
        let capacity = self.map.serialized_size();
        if self.vec.capacity() < capacity {
            self.vec.reserve(capacity - self.vec.capacity());
        }
        if let Ok(_) = self.map.serialize_into(&mut self.vec) {
            Ok(Some(&self.vec[0..self.vec.len()]))
        } else {
            Err(ProcessError)
        }
    }
}
