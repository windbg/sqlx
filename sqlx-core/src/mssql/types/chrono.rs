use crate::{
    decode::Decode,
    encode::{Encode, IsNull},
    error::BoxDynError,
    mssql::{
        protocol::type_info::{DataType, TypeInfo},
        Mssql, MssqlTypeInfo, MssqlValueRef,
    },
    types::Type,
};
use byteorder::{ByteOrder, LittleEndian};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

impl Type<Mssql> for NaiveTime {
    fn type_info() -> MssqlTypeInfo {
        MssqlTypeInfo(TypeInfo::new(DataType::TimeN, 8))
    }
}

impl Type<Mssql> for NaiveDate {
    fn type_info() -> MssqlTypeInfo {
        MssqlTypeInfo(TypeInfo::new(DataType::DateTime, 8))
    }
}

impl Type<Mssql> for NaiveDateTime {
    fn type_info() -> MssqlTypeInfo {
        MssqlTypeInfo(TypeInfo::new(DataType::DateTime, 8))
    }

    fn compatible(ty: &MssqlTypeInfo) -> bool {
        matches!(ty.0.ty, DataType::DateTime | DataType::DateTimeN) && ty.0.size == 8
    }
}

impl<Tz: TimeZone> Type<Mssql> for DateTime<Tz> {
    fn type_info() -> MssqlTypeInfo {
        MssqlTypeInfo(TypeInfo::new(DataType::DateTimeOffsetN, 8))
    }
}

impl Encode<'_, Mssql> for NaiveTime {
    fn encode_by_ref(&self, _buf: &mut Vec<u8>) -> IsNull {
        todo!()
    }
}

impl<'r> Decode<'r, Mssql> for NaiveTime {
    fn decode(_value: MssqlValueRef<'r>) -> Result<Self, BoxDynError> {
        todo!()
    }
}

impl Encode<'_, Mssql> for NaiveDate {
    fn encode_by_ref(&self, _buf: &mut Vec<u8>) -> IsNull {
        todo!()
    }
}

impl<'r> Decode<'r, Mssql> for NaiveDate {
    fn decode(_value: MssqlValueRef<'r>) -> Result<Self, BoxDynError> {
        todo!()
    }
}

impl Encode<'_, Mssql> for NaiveDateTime {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> IsNull {
        let days_duration = self.date() - NaiveDate::from_ymd(1900, 1, 1);
        let ms_duration = self.time() - NaiveTime::from_hms(0, 0, 0);
        let days = days_duration.num_days() as i32;
        let ms = ms_duration.num_milliseconds() as u32 * 3 / 10;
        buf.extend(&days.to_le_bytes());
        buf.extend_from_slice(&ms.to_le_bytes());
        IsNull::No
    }
}

impl<'r> Decode<'r, Mssql> for NaiveDateTime {
    fn decode(value: MssqlValueRef<'r>) -> Result<Self, BoxDynError> {
        let days = LittleEndian::read_i32(&value.as_bytes()?[0..4]);
        let third_seconds = LittleEndian::read_u32(&value.as_bytes()?[4..8]);
        let ms = third_seconds / 3 * 10;
        let time = NaiveTime::from_hms(0, 0, 0) + Duration::milliseconds(ms.into());
        let date = NaiveDate::from_ymd(1900, 1, 1) + Duration::days(days.into());

        Ok(date.and_time(time))
    }
}

impl<Tz: TimeZone> Encode<'_, Mssql> for DateTime<Tz> {
    fn encode_by_ref(&self, _buf: &mut Vec<u8>) -> IsNull {
        todo!()
    }
}

impl<'r> Decode<'r, Mssql> for DateTime<Local> {
    fn decode(_value: MssqlValueRef<'r>) -> Result<Self, BoxDynError> {
        todo!()
    }
}

impl<'r> Decode<'r, Mssql> for DateTime<Utc> {
    fn decode(_value: MssqlValueRef<'r>) -> Result<Self, BoxDynError> {
        todo!()
    }
}

/*
impl<'r> Decode<'r, Mssql> for NaiveDateTime {
    fn decode(value: MssqlValueRef<'r>) -> Result<Self, BoxDynError> {
        let mut buf = value.as_bytes()?;
        let days = buf.get_i32_le();
        let ticks = buf.get_u32_le();
        Ok(NaiveDateTime::new(
            from_days(days.into(), 1900),
            from_sec_fragments(ticks.into()),
        ))
    }
}

impl Encode<'_, Mssql> for NaiveDateTime {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> IsNull {
        let date = self.date();
        let time = self.time();
        let days = to_days(date, 1900) as i32;
        let seconds_fragments = to_sec_fragments(time);
        buf.extend_from_slice(&days.to_le_bytes());
        buf.extend_from_slice(&seconds_fragments.to_le_bytes());
        IsNull::No
    }

    fn size_hint(&self) -> usize {
        8
    }
}

#[inline]
fn from_days(days: i64, start_year: i32) -> NaiveDate {
    NaiveDate::from_ymd(start_year, 1, 1) + chrono::Duration::days(days as i64)
}

#[inline]
fn from_sec_fragments(sec_fragments: i64) -> NaiveTime {
    NaiveTime::from_hms(0, 0, 0) + chrono::Duration::nanoseconds(sec_fragments * (1e9 as i64) / 300)
}

#[inline]
fn to_days(date: NaiveDate, start_year: i32) -> i64 {
    date.signed_duration_since(NaiveDate::from_ymd(start_year, 1, 1))
        .num_days()
}

#[inline]
fn to_sec_fragments(time: NaiveTime) -> i64 {
    time.signed_duration_since(NaiveTime::from_hms(0, 0, 0))
        .num_nanoseconds()
        .unwrap()
        * 300
        / (1e9 as i64)
}
*/
