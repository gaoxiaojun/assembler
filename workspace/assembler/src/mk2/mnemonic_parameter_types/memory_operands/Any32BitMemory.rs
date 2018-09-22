// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Any 32-bit (four bytes) of memory referenced by `MemoryOperand`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Any32BitMemory(pub MemoryOperand);

impl MemoryOrRegister for Any32BitMemory
{
	/// Value.
	#[inline(always)]
	fn value(self) -> u8
	{
		self.0.value()
	}
}
