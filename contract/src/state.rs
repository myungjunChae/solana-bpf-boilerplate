use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

pub struct AccountData {
    pub is_initialized: bool,
    pub wallet_pubkey: Pubkey,
    pub custom_account_pubkey: Pubkey,
    pub data: u64,
}

impl AccountData {
    pub fn init(
        &mut self,
        is_initialized: bool,
        wallet_pubkey: Pubkey,
        custom_account_pubkey: Pubkey,
        data: u64,
    ) {
        self.is_initialized = is_initialized;
        self.wallet_pubkey = wallet_pubkey;
        self.custom_account_pubkey = custom_account_pubkey;
        self.data = data;
    }
}

impl Sealed for AccountData {}

impl IsInitialized for AccountData {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for AccountData {
    const LEN: usize = 73; // 1 + 32 + 32 + 8
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, AccountData::LEN];
        let (is_initialized, wallet_pubkey, custom_account_pubkey, data) =
            array_refs![src, 1, 32, 32, 8];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        return Ok(AccountData {
            is_initialized,
            wallet_pubkey: Pubkey::new_from_array(*wallet_pubkey),
            custom_account_pubkey: Pubkey::new_from_array(*custom_account_pubkey),
            data: u64::from_le_bytes(*data),
        });
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, AccountData::LEN];
        let (is_initialized_dst, wallet_pubkey_dst, custom_account_pubkey_dst, data_dst) =
            mut_array_refs![dst, 1, 32, 32, 8];

        let AccountData {
            is_initialized,
            wallet_pubkey,
            custom_account_pubkey,
            data,
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        wallet_pubkey_dst.copy_from_slice(wallet_pubkey.as_ref());
        custom_account_pubkey_dst.copy_from_slice(custom_account_pubkey.as_ref());
        *data_dst = data.to_le_bytes();
    }
}
