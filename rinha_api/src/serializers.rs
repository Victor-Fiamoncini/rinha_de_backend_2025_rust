use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use serde::Serializer;

pub fn serialize<S>(decimal: &Decimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let float_val = decimal.to_f64().unwrap_or(0.0);

    serializer.serialize_f64(float_val)
}
