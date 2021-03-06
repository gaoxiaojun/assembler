// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// 16-bit memory offset.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MemoryOffset16Bit
{
	/// `segment:offset` form.
	///
	/// Segment register is ignored in 64-bit long mode.
	SegmentOffsetForm16(SegmentRegister, Immediate64Bit),

	/// `offset` form.
	OffsetForm16(Immediate64Bit),
}

impl Default for MemoryOffset16Bit
{
	#[inline(always)]
	fn default() -> Self
	{
		MemoryOffset16Bit::OffsetForm16(Immediate64Bit::default())
	}
}

impl AsDisplacement for MemoryOffset16Bit
{
	type D = u64;
	
	#[inline(always)]
	fn displacement(self) -> Self::D
	{
		self.get_offset().displacement()
	}
}

impl From<(SegmentRegister, Immediate64Bit)> for MemoryOffset16Bit
{
	#[inline(always)]
	fn from(value: (SegmentRegister, Immediate64Bit)) -> Self
	{
		MemoryOffset16Bit::SegmentOffsetForm16(value.0, value.1)
	}
}

impl From<(Option<SegmentRegister>, Immediate64Bit)> for MemoryOffset16Bit
{
	#[inline(always)]
	fn from(value: (Option<SegmentRegister>, Immediate64Bit)) -> Self
	{
		use self::MemoryOffset16Bit::*;
		
		match value.0
		{
			Some(segment_register) => SegmentOffsetForm16(segment_register, value.1),
			None => OffsetForm16(value.1),
		}
	}
}

impl From<Immediate64Bit> for MemoryOffset16Bit
{
	#[inline(always)]
	fn from(value: Immediate64Bit) -> Self
	{
		MemoryOffset16Bit::OffsetForm16(value)
	}
}

impl From<i8> for MemoryOffset16Bit
{
	#[inline(always)]
	fn from(value: i8) -> Self
	{
		MemoryOffset16Bit::OffsetForm16(value.into())
	}
}

impl From<u8> for MemoryOffset16Bit
{
	#[inline(always)]
	fn from(value: u8) -> Self
	{
		MemoryOffset16Bit::OffsetForm16(value.into())
	}
}

impl From<i16> for MemoryOffset16Bit
{
	#[inline(always)]
	fn from(value: i16) -> Self
	{
		MemoryOffset16Bit::OffsetForm16(value.into())
	}
}

impl From<u16> for MemoryOffset16Bit
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		MemoryOffset16Bit::OffsetForm16(value.into())
	}
}

impl From<i32> for MemoryOffset16Bit
{
	#[inline(always)]
	fn from(value: i32) -> Self
	{
		MemoryOffset16Bit::OffsetForm16(value.into())
	}
}

impl From<u32> for MemoryOffset16Bit
{
	#[inline(always)]
	fn from(value: u32) -> Self
	{
		MemoryOffset16Bit::OffsetForm16(value.into())
	}
}

impl From<i64> for MemoryOffset16Bit
{
	#[inline(always)]
	fn from(value: i64) -> Self
	{
		MemoryOffset16Bit::OffsetForm16(value.into())
	}
}

impl From<u64> for MemoryOffset16Bit
{
	#[inline(always)]
	fn from(value: u64) -> Self
	{
		MemoryOffset16Bit::OffsetForm16(value.into())
	}
}

impl Into<(Option<SegmentRegister>, Immediate64Bit)> for MemoryOffset16Bit
{
	#[inline(always)]
	fn into(self) -> (Option<SegmentRegister>, Immediate64Bit)
	{
		use self::MemoryOffset16Bit::*;
		
		match self
		{
			SegmentOffsetForm16(segment_register, immediate) => (Some(segment_register), immediate),
			OffsetForm16(immediate) => (None, immediate),
		}
	}
}

impl Into<Immediate64Bit> for MemoryOffset16Bit
{
	#[inline(always)]
	fn into(self) -> Immediate64Bit
	{
		use self::MemoryOffset16Bit::*;
		
		match self
		{
			SegmentOffsetForm16(_, immediate) => immediate,
			OffsetForm16(immediate) => immediate,
		}
	}
}

impl Into<i64> for MemoryOffset16Bit
{
	#[inline(always)]
	fn into(self) -> i64
	{
		let immediate: Immediate64Bit = self.into();
		immediate.into()
	}
}

impl Into<u64> for MemoryOffset16Bit
{
	#[inline(always)]
	fn into(self) -> u64
	{
		let immediate: Immediate64Bit = self.into();
		immediate.into()
	}
}

impl MemoryOffset for MemoryOffset16Bit
{
	#[inline(always)]
	fn get_segment_register(&self) -> Option<SegmentRegister>
	{
		use self::MemoryOffset16Bit::*;
		
		match *self
		{
			SegmentOffsetForm16(segment_register, _) => Some(segment_register),
			OffsetForm16(_) => None,
		}
	}
	
	#[inline(always)]
	fn get_offset(&self) -> Immediate64Bit
	{
		use self::MemoryOffset16Bit::*;
		
		match *self
		{
			SegmentOffsetForm16(_, immediate) => immediate,
			OffsetForm16(immediate) => immediate,
		}
	}
}
