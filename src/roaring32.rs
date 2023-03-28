use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign};

use num_traits::cast::ToPrimitive;
use roaring::RoaringBitmap as Map;
use udf::prelude::*;

enum Operation {
    Or,
    XOr,
    And,
}


#[inline]
fn init_maps<T: Default>(_cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<T, String> {
    if args.len() < 2 {
        return Err(format!("Expected two or more CLOB (bitmap) or NULL arguments; Got {} arguments.", args.len()));
    }
    for arg in args.iter() {
        if let SqlResult::String(_) = arg.value() {} else {
            return Err(format!("Only CLOB (bitmap) or NULL arguments are allowed."));
        }
    }
    Ok(T::default())
}

#[inline]
fn process_maps_nullsafe(args: &ArgList<Process>, operation: &Operation) -> Result<Vec<u8>, ProcessError> {
    let mut maybe_agg: Option<Map> = None;
    for arg in args.iter() {
        match arg.value().as_bytes() {
            None => (),
            Some(bytes) => {
                if let Ok(map) = Map::deserialize_from(bytes) {
                    match maybe_agg {
                        None => maybe_agg = Some(map),
                        Some(ref mut agg) => {
                            match operation {
                                Operation::Or => agg.bitor_assign(map),
                                Operation::XOr => agg.bitxor_assign(map),
                                Operation::And => agg.bitand_assign(map),
                            }
                        }
                    }
                } else {
                    return Err(ProcessError);
                }
            }
        }
    }
    let agg = match maybe_agg {
        Some(agg) => agg,
        None => Map::new(),
    };
    let mut bytes = Vec::with_capacity(agg.serialized_size());
    if let Ok(_) = agg.serialize_into(&mut bytes) {
        Ok(bytes)
    } else {
        Err(ProcessError)
    }
}


#[derive(Default)]
struct Roaring32OrNullsafe {
    vec: Vec<u8>,
}

#[register]
impl BasicUdf for Roaring32OrNullsafe {
    type Returns<'a> = &'a [u8];

    fn init(cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        init_maps::<Self>(cfg, args)
    }

    fn process<'a>(&'a mut self, _cfg: &UdfCfg<Process>, args: &ArgList<Process>, _error: Option<NonZeroU8>)
                   -> Result<Self::Returns<'a>, ProcessError> {
        match process_maps_nullsafe(args, &Operation::Or) {
            Ok(vec) => {
                self.vec = vec;
                Ok(&self.vec[..])
            }
            Err(err) => Err(err),
        }
    }
}

#[derive(Default)]
struct Roaring32AndNullsafe {
    vec: Vec<u8>,
}

#[register]
impl BasicUdf for Roaring32AndNullsafe {
    type Returns<'a> = &'a [u8];

    fn init(cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        init_maps::<Self>(cfg, args)
    }

    fn process<'a>(&'a mut self, _cfg: &UdfCfg<Process>, args: &ArgList<Process>, _error: Option<NonZeroU8>)
                   -> Result<Self::Returns<'a>, ProcessError> {
        match process_maps_nullsafe(args, &Operation::And) {
            Ok(vec) => {
                self.vec = vec;
                Ok(&self.vec[..])
            }
            Err(err) => Err(err),
        }
    }
}

#[derive(Default)]
struct Roaring32XorNullsafe {
    vec: Vec<u8>,
}

#[register]
impl BasicUdf for Roaring32XorNullsafe {
    type Returns<'a> = &'a [u8];

    fn init(cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        init_maps::<Self>(cfg, args)
    }

    fn process<'a>(&'a mut self, _cfg: &UdfCfg<Process>, args: &ArgList<Process>, _error: Option<NonZeroU8>)
                   -> Result<Self::Returns<'a>, ProcessError> {
        match process_maps_nullsafe(args, &Operation::XOr) {
            Ok(vec) => {
                self.vec = vec;
                Ok(&self.vec[..])
            }
            Err(err) => Err(err),
        }
    }
}

#[inline]
fn init_map_int<T: Default>(_cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<T, String> {
    if args.len() < 2 {
        return Err(format!("Expected two arguments; Got {} arguments.", args.len()));
    }
    if !args.get(0).unwrap().value().is_string() {
        return Err(format!("First argument mast be CLOB (bitmap) or NULL."));
    }
    if !args.get(1).unwrap().value().is_int() {} else {
        return Err(format!("Second argument mast be INT or NULL."));
    }
    Ok(T::default())
}

#[derive(Default)]
struct Roaring32Contains;

#[register]
impl BasicUdf for Roaring32Contains {
    type Returns<'a> = Option<i64>;

    fn init(cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        cfg.set_maybe_null(true);
        init_map_int::<Self>(cfg, args)
    }

    fn process<'a>(&'a mut self, _cfg: &UdfCfg<Process>, args: &ArgList<Process>, _error: Option<NonZeroU8>)
                   -> Result<Self::Returns<'a>, ProcessError> {
        let arg0 = args.get(0).unwrap().value();
        let arg1 = args.get(1).unwrap().value();
        let bytes = arg0.as_bytes();
        let value = arg1.as_int();
        if let (Some(bytes), Some(value)) = (bytes, value) {
            if let (Ok(map), Some(value)) = (Map::deserialize_from(bytes), value.to_u32()) {
                Ok(Some(map.contains(value).into()))
            } else {
                Err(ProcessError)
            }
        } else {
            Ok(None)
        }
    }
}

#[derive(Default)]
struct Roaring32ContainsNullsafe;

#[register]
impl BasicUdf for Roaring32ContainsNullsafe {
    type Returns<'a> = i64;

    fn init(cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        init_map_int::<Self>(cfg, args)
    }

    fn process<'a>(&'a mut self, _cfg: &UdfCfg<Process>, args: &ArgList<Process>, _error: Option<NonZeroU8>)
                   -> Result<Self::Returns<'a>, ProcessError> {
        let arg0 = args.get(0).unwrap().value();
        let arg1 = args.get(1).unwrap().value();
        let bytes = arg0.as_bytes();
        let value = arg1.as_int();
        if let (Some(bytes), Some(value)) = (bytes, value) {
            if let (Ok(map), Some(value)) = (Map::deserialize_from(bytes), value.to_u32()) {
                Ok(map.contains(value).into())
            } else {
                Err(ProcessError)
            }
        } else {
            Ok(0i64)
        }
    }
}