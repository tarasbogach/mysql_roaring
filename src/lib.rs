use roaring::RoaringTreemap as Map64;
use roaring::RoaringBitmap as Map32;
use num_traits::cast::ToPrimitive;
use udf::prelude::*;
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign};

macro_rules! maps_op_nullsafe {
    ($map: ty, $strict_name: ident, $op_name: ident)=> {

        #[derive(Default)]
        struct $strict_name {
            vec: Vec<u8>,
        }

        #[register]
        impl BasicUdf for $strict_name {
            type Returns<'a> = &'a [u8];

            fn init(_cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
                if args.len() < 2 {
                    return Err(format!("Expected two or more CLOB (bitmap) or NULL arguments; Got {} arguments.", args.len()));
                }
                for arg in args.iter() {
                    if let SqlResult::String(_) = arg.value() {} else {
                        return Err(format!("Only CLOB (bitmap) or NULL arguments are allowed."));
                    }
                }
                Ok(Self::default())
            }

            fn process<'a>(&'a mut self, _cfg: &UdfCfg<Process>, args: &ArgList<Process>, _error: Option<NonZeroU8>) -> Result<Self::Returns<'a>, ProcessError> {
                let mut maybe_agg: Option<$map> = None;
                for arg in args.iter() {
                    match arg.value().as_bytes() {
                        None => (),
                        Some(bytes) => {
                            if let Ok(map) = <$map>::deserialize_from(bytes) {
                                match maybe_agg {
                                    None => maybe_agg = Some(map),
                                    Some(ref mut agg) => agg.$op_name(map),
                                }
                            } else {
                                return Err(ProcessError);
                            }
                        }
                    }
                }
                let agg = match maybe_agg {
                    Some(agg) => agg,
                    None => <$map>::new(),
                };
                let mut bytes = Vec::with_capacity(agg.serialized_size());
                if let Ok(_) = agg.serialize_into(&mut bytes) {
                    self.vec = bytes;
                    Ok(&self.vec[..])
                } else {
                    Err(ProcessError)
                }
            }
        }
    }
}
maps_op_nullsafe!(Map64, Roaring64NullsafeOr,  bitor_assign);
maps_op_nullsafe!(Map64, Roaring64NullsafeXor, bitxor_assign);
maps_op_nullsafe!(Map64, Roaring64NullsafeAnd, bitand_assign);
maps_op_nullsafe!(Map32, Roaring32NullsafeOr,  bitor_assign);
maps_op_nullsafe!(Map32, Roaring32NullsafeXor, bitxor_assign);
maps_op_nullsafe!(Map32, Roaring32NullsafeAnd, bitand_assign);


macro_rules! maps_op_nullsafe_count {
    ($map: ty, $strict_name: ident, $op_name: ident)=> {

        #[derive(Default)]
        struct $strict_name {
        }

        #[register]
        impl BasicUdf for $strict_name {
            type Returns<'a> = i64;

            fn init(_cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
                if args.len() < 2 {
                    return Err(format!("Expected two or more CLOB (bitmap) or NULL arguments; Got {} arguments.", args.len()));
                }
                for arg in args.iter() {
                    if let SqlResult::String(_) = arg.value() {} else {
                        return Err(format!("Only CLOB (bitmap) or NULL arguments are allowed."));
                    }
                }
                Ok(Self::default())
            }

            fn process<'a>(&'a mut self, _cfg: &UdfCfg<Process>, args: &ArgList<Process>, _error: Option<NonZeroU8>) -> Result<Self::Returns<'a>, ProcessError> {
                let mut maybe_agg: Option<$map> = None;
                for arg in args.iter() {
                    match arg.value().as_bytes() {
                        None => (),
                        Some(bytes) => {
                            if let Ok(map) = <$map>::deserialize_from(bytes) {
                                match maybe_agg {
                                    None => maybe_agg = Some(map),
                                    Some(ref mut agg) => agg.$op_name(map),
                                }
                            } else {
                                return Err(ProcessError);
                            }
                        }
                    }
                }
                match maybe_agg {
                    Some(agg) => Ok(agg.len() as i64),
                    None => Ok(0)
                }
            }
        }
    }
}
maps_op_nullsafe_count!(Map64, Roaring64NullsafeOrCount,  bitor_assign);
maps_op_nullsafe_count!(Map64, Roaring64NullsafeXorCount, bitxor_assign);
maps_op_nullsafe_count!(Map64, Roaring64NullsafeAndCount, bitand_assign);
maps_op_nullsafe_count!(Map32, Roaring32NullsafeOrCount,  bitor_assign);
maps_op_nullsafe_count!(Map32, Roaring32NullsafeXorCount, bitxor_assign);
maps_op_nullsafe_count!(Map32, Roaring32NullsafeAndCount, bitand_assign);

macro_rules! maps_op {
    ($map: ty, $strict_name: ident, $op_name: ident)=> {
        #[derive(Default)]
        struct $strict_name {
            vec: Vec<u8>,
        }

        #[register]
        impl BasicUdf for $strict_name {
            type Returns<'a> = Option<&'a [u8]>;

            fn init(cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
                cfg.set_maybe_null(true);
                if args.len() < 2 {
                    return Err(format!("Expected two or more CLOB (bitmap) or NULL arguments; Got {} arguments.", args.len()));
                }
                for arg in args.iter() {
                    if let SqlResult::String(_) = arg.value() {} else {
                        return Err(format!("Only CLOB (bitmap) or NULL arguments are allowed."));
                    }
                }
                Ok(Self::default())
            }

            fn process<'a>(&'a mut self, _cfg: &UdfCfg<Process>, args: &ArgList<Process>, _error: Option<NonZeroU8>) -> Result<Self::Returns<'a>, ProcessError> {
                let mut maybe_agg: Option<$map> = None;
                for arg in args.iter() {
                    match arg.value().as_bytes() {
                        None => return Ok(None),
                        Some(bytes) => {
                            if let Ok(map) = <$map>::deserialize_from(bytes) {
                                match maybe_agg {
                                    None => maybe_agg = Some(map),
                                    Some(ref mut agg) => agg.$op_name(map),
                                }
                            } else {
                                return Err(ProcessError);
                            }
                        }
                    }
                }
                let agg = match maybe_agg {
                    Some(agg) => agg,
                    None => return Ok(None),
                };
                let mut bytes = Vec::with_capacity(agg.serialized_size());
                if let Ok(_) = agg.serialize_into(&mut bytes) {
                    self.vec = bytes;
                    Ok(Some(&self.vec[..]))
                } else {
                    Err(ProcessError)
                }
            }
        }
    }
}
maps_op!(Map64, Roaring64Or,  bitor_assign);
maps_op!(Map64, Roaring64Xor, bitxor_assign);
maps_op!(Map64, Roaring64And, bitand_assign);
maps_op!(Map32, Roaring32Or,  bitor_assign);
maps_op!(Map32, Roaring32Xor, bitxor_assign);
maps_op!(Map32, Roaring32And, bitand_assign);


macro_rules! maps_op_count {
    ($map: ty, $strict_name: ident, $op_name: ident)=> {

        #[derive(Default)]
        struct $strict_name {
        }

        #[register]
        impl BasicUdf for $strict_name {
            type Returns<'a> = Option<i64>;

            fn init(cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
                cfg.set_maybe_null(true);
                if args.len() < 2 {
                    return Err(format!("Expected two or more CLOB (bitmap) or NULL arguments; Got {} arguments.", args.len()));
                }
                for arg in args.iter() {
                    if let SqlResult::String(_) = arg.value() {} else {
                        return Err(format!("Only CLOB (bitmap) or NULL arguments are allowed."));
                    }
                }
                Ok(Self::default())
            }

            fn process<'a>(&'a mut self, _cfg: &UdfCfg<Process>, args: &ArgList<Process>, _error: Option<NonZeroU8>) -> Result<Self::Returns<'a>, ProcessError> {
                let mut maybe_agg: Option<$map> = None;
                for arg in args.iter() {
                    match arg.value().as_bytes() {
                        None => return Ok(None),
                        Some(bytes) => {
                            if let Ok(map) = <$map>::deserialize_from(bytes) {
                                match maybe_agg {
                                    None => maybe_agg = Some(map),
                                    Some(ref mut agg) => agg.$op_name(map),
                                }
                            } else {
                                return Err(ProcessError);
                            }
                        }
                    }
                }
                match maybe_agg {
                    Some(agg) => Ok(Some(agg.len() as i64)),
                    None => Ok(None),
                }
            }
        }
    }
}
maps_op_count!(Map64, Roaring64OrCount,  bitor_assign);
maps_op_count!(Map64, Roaring64XorCount, bitxor_assign);
maps_op_count!(Map64, Roaring64AndCount, bitand_assign);
maps_op_count!(Map32, Roaring32OrCount,  bitor_assign);
maps_op_count!(Map32, Roaring32XorCount, bitxor_assign);
maps_op_count!(Map32, Roaring32AndCount, bitand_assign);


#[derive(Default)]
struct Roaring32Contains;

#[register]
impl BasicUdf for Roaring32Contains {
    type Returns<'a> = Option<i64>;

    fn init(cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        cfg.set_maybe_null(true);
        if args.len() < 2 {
            return Err(format!("Expected two arguments; Got {} arguments.", args.len()));
        }
        if !args.get(0).unwrap().value().is_string() {
            return Err(format!("First argument mast be CLOB (bitmap) or NULL."));
        }
        if !args.get(1).unwrap().value().is_int() {} else {
            return Err(format!("Second argument mast be INT or NULL."));
        }
        Ok(Self::default())
    }

    fn process<'a>(&'a mut self, _cfg: &UdfCfg<Process>, args: &ArgList<Process>, _error: Option<NonZeroU8>) -> Result<Self::Returns<'a>, ProcessError> {
        let arg0 = args.get(0).unwrap().value();
        let arg1 = args.get(1).unwrap().value();
        let bytes = arg0.as_bytes();
        let value = arg1.as_int();
        if let (Some(bytes), Some(value)) = (bytes, value) {
            if let (Ok(map), Some(value)) = (Map32::deserialize_from(bytes), value.to_u32()) {
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
struct Roaring32NullsafeContains;

#[register]
impl BasicUdf for Roaring32NullsafeContains {
    type Returns<'a> = i64;

    fn init(_cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        if args.len() < 2 {
            return Err(format!("Expected two arguments; Got {} arguments.", args.len()));
        }
        if !args.get(0).unwrap().value().is_string() {
            return Err(format!("First argument mast be CLOB (bitmap) or NULL."));
        }
        if !args.get(1).unwrap().value().is_int() {} else {
            return Err(format!("Second argument mast be INT or NULL."));
        }
        Ok(Self::default())
    }

    fn process<'a>(&'a mut self, _cfg: &UdfCfg<Process>, args: &ArgList<Process>, _error: Option<NonZeroU8>) -> Result<Self::Returns<'a>, ProcessError> {
        let arg0 = args.get(0).unwrap().value();
        let arg1 = args.get(1).unwrap().value();
        let bytes = arg0.as_bytes();
        let value = arg1.as_int();
        if let (Some(bytes), Some(value)) = (bytes, value) {
            if let (Ok(map), Some(value)) = (Map32::deserialize_from(bytes), value.to_u32()) {
                Ok(map.contains(value).into())
            } else {
                Err(ProcessError)
            }
        } else {
            Ok(0i64)
        }
    }
}