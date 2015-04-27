

#[allow(non_camel_case_types)]
mod internal
{
	pub type int8_t = i8;
	pub type int16_t = i16;
	pub type int32_t = i32;
	pub type int64_t = i64;

	pub type uint8_t = u8;
	pub type uint16_t = u16;
	pub type uint32_t = u32;
	pub type uint64_t = u64;

	include!(concat!(env!("OUT_DIR"), "/gen.rs"));
}

pub use self::internal::*;
