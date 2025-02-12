//! Implementation of various cast operations.

#![allow(non_snake_case)]

use std::cmp::Ordering;

use crate::{binary::ByteArray, geopoint::*, interval::*, timestamp::*, variant::*, Weight};
use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use dbsp::algebra::{HasOne, HasZero, F32, F64};
use num::{FromPrimitive, One, ToPrimitive, Zero};
use num_traits::cast::NumCast;
use rust_decimal::{Decimal, RoundingStrategy};
use std::collections::BTreeMap;
use std::error::Error;
use std::string::String;

const FLOAT_DISPLAY_PRECISION: usize = 6;
const DOUBLE_DISPLAY_PRECISION: usize = 15;

// Creates three cast functions based on an existing one
macro_rules! cast_function {
    ($result_name: ident, $result_type: ty, $type_name: ident, $arg_type: ty) => {
        ::paste::paste! {
            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_ $result_name N_ $type_name>]( value: $arg_type ) -> Option<$result_type> {
                Some([<cast_to_ $result_name _ $type_name>](value))
            }

            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_ $result_name _ $type_name N >]( value: Option<$arg_type> ) -> $result_type {
                [<cast_to_ $result_name _ $type_name>](value.unwrap())
            }

            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_ $result_name N_ $type_name N >]( value: Option<$arg_type> ) -> Option<$result_type> {
                let value = value?;
                Some([<cast_to_ $result_name _ $type_name >](value))
            }
        }
    };
}

/////////// cast to b

macro_rules! cast_to_b {
    ($type_name: ident, $arg_type: ty) => {
        ::paste::paste! {
            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_b_ $type_name>]( value: $arg_type ) -> bool {
                value != <$arg_type as num::Zero>::zero()
            }

            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_b_ $type_name N >]( value: Option<$arg_type> ) -> bool {
                [<cast_to_b_ $type_name>](value.unwrap())
            }

            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_bN_ $type_name >]( value: $arg_type ) -> Option<bool> {
                Some([< cast_to_b_ $type_name >](value))
            }

            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_bN_ $type_name N >]( value: Option<$arg_type> ) -> Option<bool> {
                let value = value?;
                [<cast_to_bN_ $type_name >](value)
            }
        }
    };
}

macro_rules! cast_to_b_fp {
    ($type_name: ident, $arg_type: ty) => {
        ::paste::paste! {
            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_b_ $type_name>]( value: $arg_type ) -> bool {
                value != $arg_type::zero()
            }

            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_b_ $type_name N >]( value: Option<$arg_type> ) -> bool {
                [<cast_to_b_ $type_name>](value.unwrap())
            }

            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_bN_ $type_name >]( value: $arg_type ) -> Option<bool> {
                Some([< cast_to_b_ $type_name >](value))
            }

            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_bN_ $type_name N >]( value: Option<$arg_type> ) -> Option<bool> {
                let value = value?;
                [<cast_to_bN_ $type_name >](value)
            }
        }
    };
}

#[doc(hidden)]
#[inline]
pub fn cast_to_b_b(value: bool) -> bool {
    value
}

#[doc(hidden)]
#[inline]
pub fn cast_to_b_bN(value: Option<bool>) -> bool {
    value.unwrap()
}

cast_to_b!(decimal, Decimal);
cast_to_b_fp!(d, F64);
cast_to_b_fp!(f, F32);
cast_to_b!(i8, i8);
cast_to_b!(i16, i16);
cast_to_b!(i32, i32);
cast_to_b!(i64, i64);
cast_to_b!(i, isize);
cast_to_b!(u, usize);

#[doc(hidden)]
#[inline]
pub fn cast_to_b_s(value: String) -> bool {
    value.trim().parse().unwrap_or(false)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_b_sN(value: Option<String>) -> bool {
    value.unwrap().trim().parse().unwrap_or(false)
}

/////////// cast to bN

#[doc(hidden)]
#[inline]
pub fn cast_to_bN_nullN(_value: Option<()>) -> Option<bool> {
    None
}

#[doc(hidden)]
#[inline]
pub fn cast_to_bN_b(value: bool) -> Option<bool> {
    Some(value)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_bN_bN(value: Option<bool>) -> Option<bool> {
    value
}

/////////// cast to date

#[doc(hidden)]
#[inline]
pub fn cast_to_Date_s(value: String) -> Date {
    let dt = NaiveDate::parse_from_str(&value, "%Y-%m-%d").ok();
    match dt {
        Some(value) => {
            Date::new((value.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp() / 86400) as i32)
        }
        None => panic!("Could not parse string '{value}' as a Date"),
    }
}

cast_function!(Date, Date, s, String);

#[doc(hidden)]
pub fn cast_to_Date_Timestamp(value: Timestamp) -> Date {
    value.get_date()
}

cast_function!(Date, Date, Timestamp, Timestamp);

#[doc(hidden)]
#[inline]
pub fn cast_to_DateN_nullN(_value: Option<()>) -> Option<Date> {
    None
}

#[doc(hidden)]
#[inline]
pub fn cast_to_Date_Date(value: Date) -> Date {
    value
}

cast_function!(Date, Date, Date, Date);

/////////// cast to Time

#[doc(hidden)]
#[inline]
pub fn cast_to_Time_s(value: String) -> Time {
    match NaiveTime::parse_from_str(&value, "%H:%M:%S%.f").ok() {
        None => panic!("Could not parse string '{value}' as a Time"),
        Some(value) => Time::from_time(value),
    }
}

cast_function!(Time, Time, s, String);

#[doc(hidden)]
#[inline]
pub fn cast_to_TimeN_nullN(_value: Option<()>) -> Option<Time> {
    None
}

#[doc(hidden)]
#[inline]
pub fn cast_to_Time_Time(value: Time) -> Time {
    value
}

cast_function!(Time, Time, Time, Time);

#[doc(hidden)]
#[inline]
pub fn cast_to_Time_Timestamp(value: Timestamp) -> Time {
    Time::from_time(value.to_dateTime().time())
}

cast_function!(Time, Time, Timestamp, Timestamp);

/////////// cast to decimal

#[doc(hidden)]
#[inline]
pub fn cast_to_decimal_b(value: bool, precision: u32, scale: u32) -> Decimal {
    let result = if value {
        <rust_decimal::Decimal as One>::one()
    } else {
        <rust_decimal::Decimal as Zero>::zero()
    };
    cast_to_decimal_decimal(result, precision, scale)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_decimal_bN(value: Option<bool>, precision: u32, scale: u32) -> Decimal {
    cast_to_decimal_b(value.unwrap(), precision, scale)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_decimal_decimal(value: Decimal, precision: u32, scale: u32) -> Decimal {
    // make sure we can fit the left half of the number in the new wanted precision

    // '1234.5678' -> DECIMAL(6, 2) is fine as the integer part fits in 4 digits
    // but to DECIMAL(6, 3) would error as we can't fit '1234' in 3 digits
    // This is the rounding strategy used in Calcite
    let result = value.round_dp_with_strategy(scale, RoundingStrategy::ToZero);

    let int_part_precision = result
        .trunc()
        .mantissa()
        .checked_abs()
        .unwrap_or(i128::MAX) // i128::MIN and i128::MAX have the same number of digits
        .checked_ilog10()
        .map(|v| v + 1)
        .unwrap_or(0);
    let to_int_part_precision = precision - scale;

    if to_int_part_precision < int_part_precision {
        panic!("cannot represent {value} as DECIMAL({precision}, {scale}): precision of DECIMAL type too small to represent value")
    }

    result
}

#[doc(hidden)]
#[inline]
pub fn cast_to_decimal_decimalN(value: Option<Decimal>, precision: u32, scale: u32) -> Decimal {
    let result = value.unwrap();
    cast_to_decimal_decimal(result, precision, scale)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_decimal_d(value: F64, precision: u32, scale: u32) -> Decimal {
    let result = Decimal::from_f64(value.into_inner()).unwrap();
    cast_to_decimal_decimal(result, precision, scale)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_decimal_dN(value: Option<F64>, precision: u32, scale: u32) -> Decimal {
    let result = Decimal::from_f64(value.unwrap().into_inner()).unwrap();
    cast_to_decimal_decimal(result, precision, scale)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_decimal_f(value: F32, precision: u32, scale: u32) -> Decimal {
    let result = Decimal::from_f32(value.into_inner()).unwrap();
    cast_to_decimal_decimal(result, precision, scale)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_decimal_fN(value: Option<F32>, precision: u32, scale: u32) -> Decimal {
    let result = Decimal::from_f32(value.unwrap().into_inner()).unwrap();
    cast_to_decimal_decimal(result, precision, scale)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_decimal_s(value: String, precision: u32, scale: u32) -> Decimal {
    let result = value.trim().parse().unwrap();
    cast_to_decimal_decimal(result, precision, scale)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_decimal_sN(value: Option<String>, precision: u32, scale: u32) -> Decimal {
    let result = match value {
        None => <rust_decimal::Decimal as Zero>::zero(),
        Some(x) => x.trim().parse().unwrap(),
    };
    cast_to_decimal_decimal(result, precision, scale)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_decimalN_V(value: Variant, precision: u32, scale: u32) -> Option<Decimal> {
    match value {
        Variant::TinyInt(i) => Some(cast_to_decimal_i8(i, precision, scale)),
        Variant::SmallInt(i) => Some(cast_to_decimal_i16(i, precision, scale)),
        Variant::Int(i) => Some(cast_to_decimal_i32(i, precision, scale)),
        Variant::BigInt(i) => Some(cast_to_decimal_i64(i, precision, scale)),
        Variant::Real(f) => Some(cast_to_decimal_f(f, precision, scale)),
        Variant::Double(f) => Some(cast_to_decimal_d(f, precision, scale)),
        Variant::Decimal(d) => Some(cast_to_decimal_decimal(d, precision, scale)),
        _ => None,
    }
}

#[doc(hidden)]
#[inline]
pub fn cast_to_decimalN_VN(value: Option<Variant>, precision: u32, scale: u32) -> Option<Decimal> {
    let value = value?;
    cast_to_decimalN_V(value, precision, scale)
}

macro_rules! cast_to_decimal {
    ($type_name: ident, $arg_type: ty) => {
        ::paste::paste! {
            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_decimal_ $type_name> ]( value: $arg_type, precision: u32, scale: u32 ) -> Decimal {
                let result = Decimal::[<from_ $arg_type>](value).unwrap();
                cast_to_decimal_decimal(result, precision, scale)
            }

            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_decimal_ $type_name N> ]( value: Option<$arg_type>, precision: u32, scale: u32 ) -> Decimal {
                let result = Decimal::[<from_ $arg_type>](value.unwrap()).unwrap();
                cast_to_decimal_decimal(result, precision, scale)
            }

            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_decimalN_ $type_name> ]( value: $arg_type, precision: u32, scale: u32 ) -> Option<Decimal> {
                let result = Some(Decimal::[<from_ $arg_type>](value).unwrap());
                set_ps(result, precision, scale)
            }

            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_decimalN_ $type_name N> ]( value: Option<$arg_type>, precision: u32, scale: u32 ) -> Option<Decimal> {
                let value = value?;
                [<cast_to_decimalN_ $type_name >](value, precision, scale)
            }
        }
    }
}

cast_to_decimal!(i, isize);
cast_to_decimal!(i8, i8);
cast_to_decimal!(i16, i16);
cast_to_decimal!(i32, i32);
cast_to_decimal!(i64, i64);
cast_to_decimal!(u, usize);

/////////// cast to decimalN

#[doc(hidden)]
#[inline]
fn set_ps(value: Option<Decimal>, precision: u32, scale: u32) -> Option<Decimal> {
    value.map(|v| cast_to_decimal_decimal(v, precision, scale))
}

#[doc(hidden)]
#[inline]
pub fn cast_to_decimalN_nullN(_value: Option<()>, _precision: u32, _scale: i32) -> Option<Decimal> {
    None
}

#[doc(hidden)]
#[inline]
pub fn cast_to_decimalN_b(value: bool, precision: u32, scale: u32) -> Option<Decimal> {
    let result = if value {
        Some(<rust_decimal::Decimal as One>::one())
    } else {
        Some(<rust_decimal::Decimal as Zero>::zero())
    };
    set_ps(result, precision, scale)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_decimalN_bN(value: Option<bool>, precision: u32, scale: u32) -> Option<Decimal> {
    let value = value?;
    cast_to_decimalN_b(value, precision, scale)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_decimalN_decimal(value: Decimal, precision: u32, scale: u32) -> Option<Decimal> {
    let result = Some(value);
    set_ps(result, precision, scale)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_decimalN_decimalN(
    value: Option<Decimal>,
    precision: u32,
    scale: u32,
) -> Option<Decimal> {
    set_ps(value, precision, scale)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_decimalN_d(value: F64, precision: u32, scale: u32) -> Option<Decimal> {
    let result = Decimal::from_f64(value.into_inner());
    set_ps(result, precision, scale)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_decimalN_dN(value: Option<F64>, precision: u32, scale: u32) -> Option<Decimal> {
    let result = match value {
        None => None,
        Some(x) => Decimal::from_f64(x.into_inner()),
    };
    set_ps(result, precision, scale)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_decimalN_f(value: F32, precision: u32, scale: u32) -> Option<Decimal> {
    let result = Decimal::from_f32(value.into_inner());
    set_ps(result, precision, scale)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_decimalN_fN(value: Option<F32>, precision: u32, scale: u32) -> Option<Decimal> {
    let result = match value {
        None => None,
        Some(x) => Decimal::from_f32(x.into_inner()),
    };
    set_ps(result, precision, scale)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_decimalN_s(value: String, precision: u32, scale: u32) -> Option<Decimal> {
    let result = Some(value.trim().parse().unwrap());
    set_ps(result, precision, scale)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_decimalN_sN(value: Option<String>, precision: u32, scale: u32) -> Option<Decimal> {
    let value = value?;
    cast_to_decimalN_s(value, precision, scale)
}

/////////// cast to double

macro_rules! cast_to_fp {
    ($type_name: ident, $arg_type: ty,
     $result_type_name: ident, $result_type: ty, $result_base_type: ty) => {
        ::paste::paste! {
            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_ $result_type_name _ $type_name >]( value: $arg_type ) -> $result_type {
                $result_type ::from(value as $result_base_type)
            }

            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_ $result_type_name _ $type_name N >]( value: Option<$arg_type> ) -> $result_type {
                $result_type ::from(value.unwrap() as $result_base_type)
            }

            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_ $result_type_name N_ $type_name >]( value: $arg_type ) -> Option<$result_type> {
                Some([<cast_to_ $result_type_name _ $type_name >](value))
            }

            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_ $result_type_name N_ $type_name N >]( value: Option<$arg_type> ) -> Option<$result_type> {
                let value = value?;
                [<cast_to_ $result_type_name N_ $type_name >](value)
            }
        }
    }
}

macro_rules! cast_to_fps {
    ($type_name: ident, $arg_type: ty) => {
        cast_to_fp!($type_name, $arg_type, d, F64, f64);
        cast_to_fp!($type_name, $arg_type, f, F32, f32);
    };
}

#[doc(hidden)]
#[inline]
pub fn cast_to_d_b(value: bool) -> F64 {
    if value {
        F64::one()
    } else {
        F64::zero()
    }
}

#[doc(hidden)]
#[inline]
pub fn cast_to_d_bN(value: Option<bool>) -> F64 {
    if value.unwrap() {
        F64::one()
    } else {
        F64::zero()
    }
}

#[doc(hidden)]
#[inline]
pub fn cast_to_d_decimal(value: Decimal) -> F64 {
    F64::from(value.to_f64().unwrap())
}

#[doc(hidden)]
#[inline]
pub fn cast_to_d_decimalN(value: Option<Decimal>) -> F64 {
    F64::from(value.unwrap().to_f64().unwrap())
}

#[doc(hidden)]
#[inline]
pub fn cast_to_d_d(value: F64) -> F64 {
    value
}

#[doc(hidden)]
#[inline]
pub fn cast_to_d_dN(value: Option<F64>) -> F64 {
    value.unwrap()
}

#[doc(hidden)]
#[inline]
pub fn cast_to_d_f(value: F32) -> F64 {
    F64::from(value.into_inner())
}

#[doc(hidden)]
#[inline]
pub fn cast_to_d_fN(value: Option<F32>) -> F64 {
    F64::from(value.unwrap().into_inner())
}

#[doc(hidden)]
#[inline]
pub fn cast_to_d_s(value: String) -> F64 {
    match value.trim().parse() {
        Err(_) => F64::zero(),
        Ok(x) => x,
    }
}

#[doc(hidden)]
#[inline]
pub fn cast_to_d_sN(value: Option<String>) -> F64 {
    match value.unwrap().trim().parse() {
        Err(_) => F64::zero(),
        Ok(x) => x,
    }
}

/////////// cast to doubleN

#[doc(hidden)]
#[inline]
pub fn cast_to_dN_nullN(_value: Option<()>) -> Option<F64> {
    None
}

#[doc(hidden)]
#[inline]
pub fn cast_to_dN_b(value: bool) -> Option<F64> {
    if value {
        Some(F64::one())
    } else {
        Some(F64::zero())
    }
}

#[doc(hidden)]
#[inline]
pub fn cast_to_dN_bN(value: Option<bool>) -> Option<F64> {
    value.map(|x| if x { F64::one() } else { F64::zero() })
}

#[doc(hidden)]
#[inline]
pub fn cast_to_dN_decimal(value: Decimal) -> Option<F64> {
    value.to_f64().map(F64::from)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_dN_decimalN(value: Option<Decimal>) -> Option<F64> {
    match value {
        None => None,
        Some(x) => x.to_f64().map(F64::from),
    }
}

#[doc(hidden)]
#[inline]
pub fn cast_to_dN_d(value: F64) -> Option<F64> {
    Some(value)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_dN_dN(value: Option<F64>) -> Option<F64> {
    value
}

#[doc(hidden)]
#[inline]
pub fn cast_to_dN_f(value: F32) -> Option<F64> {
    Some(F64::from(value.into_inner()))
}

#[doc(hidden)]
#[inline]
pub fn cast_to_dN_fN(value: Option<F32>) -> Option<F64> {
    value.map(|x| F64::from(x.into_inner()))
}

#[doc(hidden)]
#[inline]
pub fn cast_to_dN_s(value: String) -> Option<F64> {
    match value.trim().parse::<f64>() {
        Err(_) => Some(F64::zero()),
        Ok(x) => Some(F64::new(x)),
    }
}

#[doc(hidden)]
#[inline]
pub fn cast_to_dN_sN(value: Option<String>) -> Option<F64> {
    match value {
        None => None,
        Some(x) => match x.trim().parse::<f64>() {
            Err(_) => Some(F64::zero()),
            Ok(x) => Some(F64::new(x)),
        },
    }
}

cast_to_fps!(i, isize);
cast_to_fps!(i8, i8);
cast_to_fps!(i16, i16);
cast_to_fps!(i32, i32);
cast_to_fps!(i64, i64);
cast_to_fps!(u, usize);

/////////// Cast to float

#[doc(hidden)]
#[inline]
pub fn cast_to_f_b(value: bool) -> F32 {
    if value {
        F32::one()
    } else {
        F32::zero()
    }
}

#[doc(hidden)]
#[inline]
pub fn cast_to_f_bN(value: Option<bool>) -> F32 {
    if value.unwrap() {
        F32::one()
    } else {
        F32::zero()
    }
}

#[doc(hidden)]
#[inline]
pub fn cast_to_f_decimal(value: Decimal) -> F32 {
    F32::from(value.to_f32().unwrap())
}

#[doc(hidden)]
#[inline]
pub fn cast_to_f_decimalN(value: Option<Decimal>) -> F32 {
    F32::from(value.unwrap().to_f32().unwrap())
}

#[doc(hidden)]
#[inline]
pub fn cast_to_f_d(value: F64) -> F32 {
    F32::from(value.into_inner() as f32)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_f_dN(value: Option<F64>) -> F32 {
    F32::from(value.unwrap().into_inner() as f32)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_f_f(value: F32) -> F32 {
    value
}

#[doc(hidden)]
#[inline]
pub fn cast_to_f_fN(value: Option<F32>) -> F32 {
    value.unwrap()
}

#[doc(hidden)]
#[inline]
pub fn cast_to_f_s(value: String) -> F32 {
    match value.trim().parse() {
        Err(_) => F32::zero(),
        Ok(x) => x,
    }
}

#[doc(hidden)]
#[inline]
pub fn cast_to_f_sN(value: Option<String>) -> F32 {
    match value.unwrap().trim().parse() {
        Err(_) => F32::zero(),
        Ok(x) => x,
    }
}

/////////// cast to floatN

#[doc(hidden)]
#[inline]
pub fn cast_to_fN_nullN(_value: Option<()>) -> Option<F32> {
    None
}

#[doc(hidden)]
#[inline]
pub fn cast_to_fN_b(value: bool) -> Option<F32> {
    if value {
        Some(F32::one())
    } else {
        Some(F32::zero())
    }
}

#[doc(hidden)]
#[inline]
pub fn cast_to_fN_bN(value: Option<bool>) -> Option<F32> {
    value.map(|x| if x { F32::one() } else { F32::zero() })
}

#[doc(hidden)]
#[inline]
pub fn cast_to_fN_decimal(value: Decimal) -> Option<F32> {
    value.to_f32().map(F32::from)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_fN_decimalN(value: Option<Decimal>) -> Option<F32> {
    match value {
        None => None,
        Some(x) => x.to_f32().map(F32::from),
    }
}

#[doc(hidden)]
#[inline]
pub fn cast_to_fN_d(value: F64) -> Option<F32> {
    Some(F32::from(value.into_inner() as f32))
}

#[doc(hidden)]
#[inline]
pub fn cast_to_fN_dN(value: Option<F64>) -> Option<F32> {
    value.map(|x| F32::from(x.into_inner() as f32))
}

#[doc(hidden)]
#[inline]
pub fn cast_to_fN_f(value: F32) -> Option<F32> {
    Some(value)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_fN_fN(value: Option<F32>) -> Option<F32> {
    value
}

#[doc(hidden)]
#[inline]
pub fn cast_to_fN_s(value: String) -> Option<F32> {
    match value.trim().parse::<f32>() {
        Err(_) => Some(F32::zero()),
        Ok(x) => Some(F32::from(x)),
    }
}

#[doc(hidden)]
#[inline]
pub fn cast_to_fN_sN(value: Option<String>) -> Option<F32> {
    match value {
        None => None,
        Some(x) => match x.trim().parse::<f32>() {
            Err(_) => Some(F32::zero()),
            Ok(x) => Some(F32::from(x)),
        },
    }
}

/////////// cast to GeoPoint

#[doc(hidden)]
#[inline]
pub fn cast_to_geopointN_geopoint(value: GeoPoint) -> Option<GeoPoint> {
    Some(value)
}

/////////// cast to String

// True if the size means "unlimited"
fn is_unlimited_size(size: i32) -> bool {
    size < 0
}

#[doc(hidden)]
#[inline]
pub fn s_helper<T>(value: Option<T>) -> String
where
    T: ToString,
{
    match value {
        None => String::from("NULL"),
        Some(x) => x.to_string(),
    }
}

#[inline(always)]
pub fn truncate(value: String, size: usize) -> String {
    let mut result = value;
    result.truncate(size);
    result
}

/// Make sure the specified string has exactly the
/// specified size.
#[inline(always)]
pub fn size_string(value: String, size: i32) -> String {
    if is_unlimited_size(size) {
        value.trim_end().to_string()
    } else {
        let sz = size as usize;
        match value.len().cmp(&sz) {
            Ordering::Equal => value,
            Ordering::Greater => truncate(value, sz),
            Ordering::Less => format!("{value:<sz$}"),
        }
    }
}

/// Make sure that the specified string does not exceed
/// the specified size.
#[inline(always)]
pub fn limit_string(value: String, size: i32) -> String {
    if is_unlimited_size(size) {
        value.trim_end().to_string()
    } else {
        let sz = size as usize;
        if value.len() < sz {
            value
        } else {
            // TODO: this is legal only of all excess characters are spaces
            truncate(value, sz)
        }
    }
}

#[inline(always)]
pub fn limit_or_size_string(value: String, size: i32, fixed: bool) -> String {
    if fixed {
        size_string(value, size)
    } else {
        limit_string(value, size)
    }
}

macro_rules! cast_to_string {
    ($type_name: ident, $arg_type: ty) => {
        ::paste::paste! {
            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_s_ $type_name N >]( value: Option<$arg_type>, size: i32, fixed: bool ) -> String {
                [<cast_to_s_ $type_name>](value.unwrap(), size, fixed)
            }

            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_sN_ $type_name >]( value: $arg_type, size: i32, fixed: bool ) -> Option<String> {
                Some([< cast_to_s_ $type_name >](value, size, fixed))
            }

            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_sN_ $type_name N >]( value: Option<$arg_type>, size: i32, fixed: bool ) -> Option<String> {
                let value = value?;
                [<cast_to_sN_ $type_name >](value, size, fixed)
            }
        }
    };
}

#[doc(hidden)]
#[inline]
pub fn cast_to_s_b(value: bool, size: i32, fixed: bool) -> String {
    let result = value.to_string();
    limit_or_size_string(result, size, fixed)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_s_decimal(value: Decimal, size: i32, fixed: bool) -> String {
    let result = value.to_string();
    limit_or_size_string(result, size, fixed)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_s_d(value: F64, size: i32, fixed: bool) -> String {
    let result = format!("{1:.0$}", DOUBLE_DISPLAY_PRECISION, value);
    let result = result.trim_end_matches('0').to_string();
    let result = result.trim_end_matches('.').to_string();

    let result = match result.parse::<f64>() {
        Ok(val) if val.is_infinite() && val.is_sign_positive() => String::from("Infinity"),
        Ok(val) if val.is_infinite() && val.is_sign_negative() => String::from("-Infinity"),
        _ => result,
    };

    limit_or_size_string(result, size, fixed)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_s_f(value: F32, size: i32, fixed: bool) -> String {
    let result = format!("{1:.0$}", FLOAT_DISPLAY_PRECISION, value);
    let result = result.trim_end_matches('0').to_string();
    let result = result.trim_end_matches('.').to_string();

    let result = match result.parse::<f32>() {
        Ok(val) if val.is_infinite() && val.is_sign_positive() => String::from("Infinity"),
        Ok(val) if val.is_infinite() && val.is_sign_negative() => String::from("-Infinity"),
        _ => result,
    };

    limit_or_size_string(result, size, fixed)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_s_s(value: String, size: i32, fixed: bool) -> String {
    let result = value;
    limit_or_size_string(result, size, fixed)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_s_Timestamp(value: Timestamp, size: i32, fixed: bool) -> String {
    let dt = value.to_dateTime();
    let month = dt.month();
    let day = dt.day();
    let year = dt.year();
    let hr = dt.hour();
    let min = dt.minute();
    let sec = dt.second();
    let result = format!(
        "{}-{:02}-{:02} {:02}:{:02}:{:02}",
        year, month, day, hr, min, sec
    );
    limit_or_size_string(result, size, fixed)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_s_Date(value: Date, size: i32, fixed: bool) -> String {
    let dt = value.to_date();
    let month = dt.month();
    let day = dt.day();
    let year = dt.year();
    let result = format!("{}-{:02}-{:02}", year, month, day);
    limit_or_size_string(result, size, fixed)
}

pub fn cast_to_s_Time(value: Time, size: i32, fixed: bool) -> String {
    let dt = value.to_time();
    let hr = dt.hour();
    let min = dt.minute();
    let sec = dt.second();
    let result = format!("{:02}:{:02}:{:02}", hr, min, sec);
    limit_or_size_string(result, size, fixed)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_s_i(value: isize, size: i32, fixed: bool) -> String {
    let result = value.to_string();
    limit_or_size_string(result, size, fixed)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_s_i8(value: i8, size: i32, fixed: bool) -> String {
    let result = value.to_string();
    limit_or_size_string(result, size, fixed)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_s_i16(value: i16, size: i32, fixed: bool) -> String {
    let result = value.to_string();
    limit_or_size_string(result, size, fixed)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_s_i32(value: i32, size: i32, fixed: bool) -> String {
    let result = value.to_string();
    limit_or_size_string(result, size, fixed)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_s_i64(value: i64, size: i32, fixed: bool) -> String {
    let result = value.to_string();
    limit_or_size_string(result, size, fixed)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_s_u(value: usize, size: i32, fixed: bool) -> String {
    let result = value.to_string();
    limit_or_size_string(result, size, fixed)
}

pub fn cast_to_s_V(value: Variant, size: i32, fixed: bool) -> String {
    let result: String = value.try_into().unwrap();
    limit_or_size_string(result, size, fixed)
}

cast_to_string!(b, bool);
cast_to_string!(decimal, Decimal);
cast_to_string!(f, F32);
cast_to_string!(d, F64);
cast_to_string!(s, String);
cast_to_string!(i, isize);
cast_to_string!(u, usize);
cast_to_string!(i8, i8);
cast_to_string!(i16, i16);
cast_to_string!(i32, i32);
cast_to_string!(i64, i64);
cast_to_string!(Timestamp, Timestamp);
cast_to_string!(Time, Time);
cast_to_string!(Date, Date);
cast_to_string!(V, Variant);

#[doc(hidden)]
#[inline]
pub fn cast_to_sN_nullN(_value: Option<()>, _size: i32, _fixed: bool) -> Option<String> {
    None
}

/////////// cast to integer

macro_rules! cast_to_i_i {
    ($result_type: ty, $arg_type_name: ident, $arg_type: ty) => {
        ::paste::paste! {
            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_ $result_type _ $arg_type_name>]( value: $arg_type ) -> $result_type {
                $result_type::try_from(value)
                    .unwrap_or_else(|_| panic!("Value '{}' out of range for type '{}'",
                                               value,
                                               stringify!($result_type)))
            }

            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_ $result_type _ $arg_type_name N>]( value: Option<$arg_type> ) -> $result_type {
                $result_type::try_from(value.unwrap())
                    .unwrap_or_else(|_| panic!("Value '{:?}' out of range for type '{}'",
                                               value,
                                               stringify!($result_type)))
            }

            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_ $result_type N_ $arg_type_name >]( value: $arg_type ) -> Option<$result_type> {
                Some($result_type::try_from(value)
                    .unwrap_or_else(|_| panic!("Value '{:?}' out of range for type '{}'",
                                               value,
                                               stringify!($result_type))))
            }

            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_ $result_type N_ $arg_type_name N>]( value: Option<$arg_type> ) -> Option<$result_type> {
                let value = value?;
                [<cast_to_ $result_type N_ $arg_type_name >](value)
            }
        }
    }
}

macro_rules! cast_to_i {
    ($result_type: ty) => {
        ::paste::paste! {
            #[doc(hidden)]
            #[inline]
            pub fn [< cast_to_ $result_type N_nullN >](_value: Option<()>) -> Option<$result_type> {
                None
            }

            // From bool

            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_ $result_type _ b >]( value: bool ) -> $result_type {
                if value { 1 } else { 0 }
            }

            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_ $result_type _ bN >]( value: Option<bool> ) -> $result_type {
                [< cast_to_ $result_type _ b >]( value.unwrap() )
            }

            #[doc(hidden)]
            #[inline]
            pub fn [< cast_to_ $result_type N_b >](value: bool) -> Option<$result_type> {
                Some(if value { 1 } else { 0 })
            }

            #[doc(hidden)]
            #[inline]
            pub fn [< cast_to_ $result_type N_bN >](value: Option<bool>) -> Option<$result_type> {
                value.map(|x| if x { 1 } else { 0 })
            }

            // From decimal

            #[doc(hidden)]
            #[inline]
            pub fn [< cast_to_ $result_type _decimal >](value: Decimal) -> $result_type {
                value.trunc().[<to_ $result_type >]()
                    .unwrap_or_else(|| panic!("Value '{}' out of range for type '{}'",
                                              value,
                                              stringify!($result_type)))
            }

            #[doc(hidden)]
            #[inline]
            pub fn [< cast_to_ $result_type N_decimal >](value: Decimal) -> Option<$result_type> {
                Some(value.trunc().[<to_ $result_type >]()
                    .unwrap_or_else(|| panic!("Value '{}' out of range for type '{}'",
                                              value,
                                              stringify!($result_type))))
            }

            #[doc(hidden)]
            #[inline]
            pub fn [< cast_to_ $result_type N_decimalN >](value: Option<Decimal>) -> Option<$result_type> {
                let value = value?;
                [< cast_to_ $result_type N_decimal >](value.trunc())
            }

            #[doc(hidden)]
            #[inline]
            pub fn [< cast_to_ $result_type _decimalN >](value: Option<Decimal>) -> $result_type {
                [< cast_to_ $result_type _decimal >](value.unwrap().trunc())
            }

            // F64

            #[doc(hidden)]
            #[inline]
            pub fn [< cast_to_ $result_type _d >](value: F64) -> $result_type {
                let value = value.into_inner().trunc();
                <$result_type as NumCast>::from(value)
                    .unwrap_or_else(|| panic!("Value '{}' out of range for type '{}'",
                                              value,
                                              stringify!($result_type)))
            }

            #[doc(hidden)]
            #[inline]
            pub fn [< cast_to_ $result_type _dN >](value: Option<F64>) -> $result_type {
                let value = value.unwrap().into_inner().trunc();
                <$result_type as NumCast>::from(value)
                    .unwrap_or_else(|| panic!("Value '{}' out of range for type '{}'",
                                              value,
                                              stringify!($result_type)))
            }

            #[doc(hidden)]
            #[inline]
            pub fn [< cast_to_ $result_type N_d >](value: F64) -> Option<$result_type> {
                let value = value.into_inner().trunc();
                Some(<$result_type as NumCast>::from(value)
                     .unwrap_or_else(|| panic!("Value '{}' out of range for type '{}'",
                                               value,
                                               stringify!($result_type))))
            }

            #[doc(hidden)]
            #[inline]
            pub fn [< cast_to_ $result_type N_dN >](value: Option<F64>) -> Option<$result_type> {
                let value = value?;
                let value = value.into_inner().trunc();
                Some(<$result_type as NumCast>::from(value)
                     .unwrap_or_else(|| panic!("Value '{}' out of range for type '{}'",
                                               value,
                                               stringify!($result_type))))
            }

            // F32

            #[doc(hidden)]
            #[inline]
            pub fn [< cast_to_ $result_type _f >](value: F32) -> $result_type {
                let value = value.into_inner().trunc();
                <$result_type as NumCast>::from(value)
                     .unwrap_or_else(|| panic!("Value '{}' out of range for type '{}'",
                                               value,
                                               stringify!($result_type)))
            }

            #[doc(hidden)]
            #[inline]
            pub fn [< cast_to_ $result_type _fN >](value: Option<F32>) -> $result_type {
                let value = value.unwrap().into_inner().trunc();
                <$result_type as NumCast>::from(value)
                     .unwrap_or_else(|| panic!("Value '{}' out of range for type '{}'",
                                               value,
                                               stringify!($result_type)))
            }

            #[doc(hidden)]
            #[inline]
            pub fn [< cast_to_ $result_type N_f >](value: F32) -> Option<$result_type> {
                let value = value.into_inner().trunc();
                Some(<$result_type as NumCast>::from(value)
                     .unwrap_or_else(|| panic!("Value '{}' out of range for type '{}'",
                                               value,
                                               stringify!($result_type))))
            }

            #[doc(hidden)]
            #[inline]
            pub fn [< cast_to_ $result_type N_fN >](value: Option<F32>) -> Option<$result_type> {
                let value = value?;
                let value = value.into_inner().trunc();
                Some(<$result_type as NumCast>::from(value)
                     .unwrap_or_else(|| panic!("Value '{}' out of range for type '{}'",
                                               value,
                                               stringify!($result_type))))
            }

            // From string

            #[doc(hidden)]
            #[inline]
            pub fn [< cast_to_ $result_type _s >](value: String) -> $result_type {
                value.trim().parse()
                    .unwrap_or_else(|_| panic!("Could not parse '{:?}' as a value of type '{}'",
                                               value.clone(),
                                               stringify!($result_type)))
            }

            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_ $result_type _sN >](value: Option<String>) -> $result_type {
                value.as_ref().unwrap().trim().parse()
                    .unwrap_or_else(|_| panic!("Could not parse '{:?}' as a value of type '{}'",
                                               &value,
                                               stringify!($result_type)))
            }

            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_ $result_type N_s >](value: String) -> Option<$result_type> {
                value.trim().parse().ok()
            }

            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_ $result_type N_sN >](value: Option<String>) -> Option<$result_type> {
                value.unwrap().trim().parse().ok()
            }

            // From other integers

            cast_to_i_i!($result_type, i8, i8);
            cast_to_i_i!($result_type, i16, i16);
            cast_to_i_i!($result_type, i32, i32);
            cast_to_i_i!($result_type, i64, i64);
            cast_to_i_i!($result_type, i, isize);
            cast_to_i_i!($result_type, u, usize);
        }
    }
}

cast_to_i!(i8);
cast_to_i!(i16);
cast_to_i!(i32);
cast_to_i!(i64);
cast_to_i!(u16);
cast_to_i!(u32);
cast_to_i!(u64);
cast_to_i!(u128);

#[doc(hidden)]
#[inline]
#[allow(clippy::unnecessary_cast)]
pub fn cast_to_i64_Weight(w: Weight) -> i64 {
    w as i64
}

#[doc(hidden)]
#[inline]
pub fn cast_to_i64_ShortInterval(value: ShortInterval) -> i64 {
    value.milliseconds()
}

#[doc(hidden)]
#[inline]
pub fn cast_to_i64N_ShortIntervalN(value: Option<ShortInterval>) -> Option<i64> {
    let value = value?;
    Some(cast_to_i64_ShortInterval(value))
}

#[doc(hidden)]
#[inline]
pub fn cast_to_i64_LongInterval(value: LongInterval) -> i64 {
    value.months() as i64
}

#[doc(hidden)]
#[inline]
pub fn cast_to_i64N_LongIntervalN(value: Option<LongInterval>) -> Option<i64> {
    let value = value?;
    Some(cast_to_i64_LongInterval(value))
}

//////// casts to Short interval

#[doc(hidden)]
#[inline]
pub fn cast_to_ShortInterval_i8(value: i8) -> ShortInterval {
    ShortInterval::from(value as i64)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_ShortInterval_i16(value: i16) -> ShortInterval {
    ShortInterval::from(value as i64)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_ShortInterval_i32(value: i32) -> ShortInterval {
    ShortInterval::from(value as i64)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_ShortInterval_i64(value: i64) -> ShortInterval {
    ShortInterval::from(value)
}

//////// casts to ShortIntervalN

#[doc(hidden)]
#[inline]
pub fn cast_to_ShortIntervalN_nullN(_value: Option<()>) -> Option<ShortInterval> {
    None
}

//////// casts to Timestamp

#[doc(hidden)]
#[inline]
pub fn cast_to_Timestamp_s(value: String) -> Timestamp {
    if let Ok(v) = NaiveDateTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S%.f") {
        // round the number of microseconds
        let nanos = v.and_utc().timestamp_subsec_nanos();
        let nanos = (nanos + 500000) / 1000000;
        let result = Timestamp::new(v.and_utc().timestamp() * 1000 + (nanos as i64));
        //println!("Parsed successfully {} using {} into {:?} ({})",
        //         value, "%Y-%m-%d %H:%M:%S%.f", result, result.milliseconds());
        return result;
    }

    // Try just a date.
    // parse_from_str fails to parse a datetime if there is no time in the format!
    if let Ok(v) = NaiveDate::parse_from_str(&value, "%Y-%m-%d") {
        let dt = v.and_hms_opt(0, 0, 0).unwrap();
        let result = Timestamp::new(dt.and_utc().timestamp_millis());
        //println!("Parsed successfully {} using {} into {:?} ({})",
        //         value, "%Y-%m-%d", result, result.milliseconds());
        return result;
    }

    panic!("Failed to parse '{value}' as a Timestamp");
}

cast_function!(Timestamp, Timestamp, s, String);

#[doc(hidden)]
#[inline]
pub fn cast_to_Timestamp_Date(value: Date) -> Timestamp {
    value.to_timestamp()
}

cast_function!(Timestamp, Timestamp, Date, Date);

#[doc(hidden)]
#[inline]
pub fn cast_to_TimestampN_nullN(_value: Option<()>) -> Option<Timestamp> {
    None
}

#[doc(hidden)]
#[inline]
pub fn cast_to_Timestamp_Timestamp(value: Timestamp) -> Timestamp {
    value
}

cast_function!(Timestamp, Timestamp, Timestamp, Timestamp);

//////////////////// Other casts

#[doc(hidden)]
#[inline]
pub fn cast_to_u_i32(value: i32) -> usize {
    value
        .try_into()
        .unwrap_or_else(|_| panic!("Value '{}' out of range for type 'usize'", value))
}

cast_function!(u, usize, i32, i32);

#[doc(hidden)]
#[inline]
pub fn cast_to_u_i64(value: i64) -> usize {
    value
        .try_into()
        .unwrap_or_else(|_| panic!("Value '{}' out of range for type 'usize'", value))
}

cast_function!(u, usize, i64, i64);

#[doc(hidden)]
#[inline]
pub fn cast_to_i_i32(value: i32) -> isize {
    value as isize
}

cast_function!(i, isize, i32, i32);

#[doc(hidden)]
#[inline]
pub fn cast_to_i_i64(value: i64) -> isize {
    value as isize
}

cast_function!(i, isize, i64, i64);

pub fn cast_to_bytesN_nullN(_value: Option<()>) -> Option<ByteArray> {
    None
}

#[doc(hidden)]
#[inline]
pub fn cast_to_bytes_bytes(value: ByteArray) -> ByteArray {
    value
}

#[doc(hidden)]
#[inline]
pub fn cast_to_bytes_bytesN(value: Option<ByteArray>) -> ByteArray {
    value.unwrap()
}

#[doc(hidden)]
#[inline]
pub fn cast_to_bytesN_bytes(value: ByteArray) -> Option<ByteArray> {
    Some(value)
}

///////////////////// Cast to Variant

// Synthesizes 6 functions for the argument type, e.g.:
// cast_to_V_i32
// cast_to_VN_i32
// cast_to_V_i32N
// cast_to_VN_i32N
macro_rules! cast_to_variant {
    ($result_name: ident, $result_type: ty, $enum: ident) => {
        ::paste::paste! {
            // cast_to_V_i32
            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_ V_ $result_name >]( value: $result_type ) -> Variant {
                Variant::from(value)
            }

            // cast_to_VN_i32
            #[doc(hidden)]
            pub fn [<cast_to_ VN_ $result_name >]( value: $result_type ) -> Option<Variant> {
                Some(Variant::from(value))
            }

            // cast_to_V_i32N
            #[doc(hidden)]
            pub fn [<cast_to_ V_ $result_name N>]( value: Option<$result_type> ) -> Variant {
                match value {
                    None => Variant::SqlNull,
                    Some(value) => Variant::from(value),
                }
            }

            // cast_to_VN_i32N
            #[doc(hidden)]
            pub fn [<cast_to_ VN_ $result_name N>]( value: Option<$result_type> ) -> Option<Variant> {
                Some([ <cast_to_ V_ $result_name N >](value))
            }
        }
    };
}

// Synthesizes 2 functions
// cast_to_i32N_VN
// cast_to_i32N_V
macro_rules! cast_from_variant {
    ($result_name: ident, $result_type: ty, $enum: ident) => {
        ::paste::paste! {
            // cast_to_i32N_V
            #[doc(hidden)]
            #[inline]
            pub fn [< cast_to_ $result_name N _V >](value: Variant) -> Option<$result_type> {
                match value {
                    Variant::$enum(value) => Some(value),
                    _            => None,
                }
            }

            // cast_to_i32N_VN
            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_ $result_name N_ VN >]( value: Option<Variant> ) -> Option<$result_type> {
                let value = value?;
                [<cast_to_ $result_name N_V >](value)
            }
        }
    };
}

macro_rules! cast_variant {
    ($result_name: ident, $result_type: ty, $enum: ident) => {
        cast_to_variant!($result_name, $result_type, $enum);
        cast_from_variant!($result_name, $result_type, $enum);
    };
}

macro_rules! cast_from_variant_numeric {
    ($result_name: ident, $result_type: ty) => {
        ::paste::paste! {
            // cast_to_i32N_V
            #[doc(hidden)]
            #[inline]
            pub fn [< cast_to_ $result_name N _V >](value: Variant) -> Option<$result_type> {
                match value {
                    Variant::TinyInt(value) => Some([< cast_to_ $result_name _i8 >](value)),
                    Variant::SmallInt(value) => Some([< cast_to_ $result_name _i16 >](value)),
                    Variant::Int(value) => Some([< cast_to_ $result_name _i32 >](value)),
                    Variant::BigInt(value) => Some([< cast_to_ $result_name _i64 >](value)),
                    Variant::Real(value) => Some([< cast_to_ $result_name _f >](value)),
                    Variant::Double(value) => Some([< cast_to_ $result_name _d >](value)),
                    Variant::Decimal(value) => Some([< cast_to_ $result_name _decimal >](value)),
                    _            => None,
                }
            }

            // cast_to_i32N_VN
            #[doc(hidden)]
            #[inline]
            pub fn [<cast_to_ $result_name N_ VN >]( value: Option<Variant> ) -> Option<$result_type> {
                let value = value?;
                [<cast_to_ $result_name N_V >](value)
            }
        }

    };
}

macro_rules! cast_variant_numeric {
    ($result_name: ident, $result_type: ty, $enum: ident) => {
        cast_to_variant!($result_name, $result_type, $enum);
        cast_from_variant_numeric!($result_name, $result_type);
    };
}

cast_variant!(b, bool, Boolean);
cast_variant_numeric!(i8, i8, TinyInt);
cast_variant_numeric!(i16, i16, SmallInt);
cast_variant_numeric!(i32, i32, Int);
cast_variant_numeric!(i64, i64, BigInt);
cast_variant_numeric!(f, F32, Real);
cast_variant_numeric!(d, F64, Double);
cast_to_variant!(decimal, Decimal, Decimal); // The other two take extra arguments
cast_to_variant!(s, String, String); // The other two take extra arguments
cast_variant!(Date, Date, Date);
cast_variant!(Time, Time, Time);
cast_variant!(bytes, ByteArray, Binary);
cast_variant!(Timestamp, Timestamp, Timestamp);
cast_variant!(ShortInterval, ShortInterval, ShortInterval);
cast_variant!(LongInterval, LongInterval, LongInterval);
cast_variant!(GeoPoint, GeoPoint, Geometry);

#[doc(hidden)]
pub fn cast_to_V_vec<T>(vec: Vec<T>) -> Variant
where
    Variant: From<T>,
{
    vec.into()
}

#[doc(hidden)]
pub fn cast_to_V_vecN<T>(vec: Option<Vec<T>>) -> Variant
where
    Variant: From<T>,
{
    match vec {
        None => Variant::SqlNull,
        Some(vec) => vec.into(),
    }
}

#[doc(hidden)]
pub fn cast_to_vec_V<T>(value: Variant) -> Vec<T>
where
    Vec<T>: TryFrom<Variant, Error = Box<dyn Error>>,
{
    value
        .try_into()
        .unwrap_or_else(|_| panic!("Cannot convert to vector"))
}

#[doc(hidden)]
pub fn cast_to_vec_VN<T>(value: Option<Variant>) -> Option<Vec<T>>
where
    Vec<T>: TryFrom<Variant, Error = Box<dyn Error>>,
{
    let value = value?;
    cast_to_vecN_V(value)
}

#[doc(hidden)]
pub fn cast_to_vecN_V<T>(value: Variant) -> Option<Vec<T>>
where
    Vec<T>: TryFrom<Variant, Error = Box<dyn Error>>,
{
    value.try_into().ok()
}

#[doc(hidden)]
pub fn cast_to_vecN_VN<T>(value: Option<Variant>) -> Option<Vec<T>>
where
    Vec<T>: TryFrom<Variant, Error = Box<dyn Error>>,
{
    let value = value?;
    cast_to_vecN_V(value)
}

#[doc(hidden)]
#[inline]
pub fn cast_to_V_VN(value: Option<Variant>) -> Variant {
    match value {
        None => Variant::SqlNull,
        Some(x) => x,
    }
}

/////// cast variant to map

#[doc(hidden)]
pub fn cast_to_V_map<K, V>(map: BTreeMap<K, V>) -> Variant
where
    Variant: From<K> + From<V>,
    K: Clone + Ord,
    V: Clone,
{
    map.into()
}

#[doc(hidden)]
pub fn cast_to_V_mapN<K, V>(map: Option<BTreeMap<K, V>>) -> Variant
where
    Variant: From<K> + From<V>,
    K: Clone + Ord,
    V: Clone,
{
    match map {
        None => Variant::SqlNull,
        Some(map) => map.into(),
    }
}

#[doc(hidden)]
pub fn cast_to_map_V<K, V>(value: Variant) -> BTreeMap<K, V>
where
    BTreeMap<K, V>: TryFrom<Variant, Error = Box<dyn Error>>,
{
    value
        .try_into()
        .unwrap_or_else(|_| panic!("Cannot convert to map"))
}

#[doc(hidden)]
pub fn cast_to_map_VN<K, V>(value: Option<Variant>) -> Option<BTreeMap<K, V>>
where
    BTreeMap<K, V>: TryFrom<Variant, Error = Box<dyn Error>>,
{
    let value = value?;
    cast_to_mapN_V(value)
}

#[doc(hidden)]
pub fn cast_to_mapN_V<K, V>(value: Variant) -> Option<BTreeMap<K, V>>
where
    BTreeMap<K, V>: TryFrom<Variant, Error = Box<dyn Error>>,
{
    value.try_into().ok()
}

#[doc(hidden)]
pub fn cast_to_mapN_VN<K, V>(value: Option<Variant>) -> Option<BTreeMap<K, V>>
where
    BTreeMap<K, V>: TryFrom<Variant, Error = Box<dyn Error>>,
{
    let value = value?;
    cast_to_mapN_V(value)
}
