use solana_program::program_error::ProgramError;
use crate::error::EscrowError::InvalidInstruction;


pub enum EscrowInstruction {
    /// 这个占领需要的账户
    ///
    /// 0. `[signer]` 初始化escrow的账户,需要是一个signer
    /// 1. `[writable]` 一个临时的token account
    /// 2. `[]` 初始化者的token账户,用于接收交易完成后获得的token
    /// 3. `[writable]` escrow账户,用于存储所有交易相关的必要信息
    /// 4. `[]` rent系统变量
    /// 5. `[]` token程序
    InitEscrow {
        /// The amount party A expects to receive of token Y
        amount: u64,
    },
}
impl EscrowInstruction {
    /// Unpacks a byte buffer into a [EscrowInstruction](enum.EscrowInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitEscrow {
                amount: Self::unpack_amount(rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}