#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use sp_std::prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_support::{
        dispatch::DispatchResultWithPostInfo,
        pallet_prelude::*,
        sp_runtime::traits::Hash
    };
    
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        TokenCreated(TokenDetail<HashOf<T>>),
        TokenRemoved
    }

    #[pallet::error]
    pub enum Error<T> {
        TokenAlreadyExist,
        TokenNotExist
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn token)]
    pub(super) type Token<T: Config>  = StorageMap<_, Blake2_128Concat, TokenId, TokenDetail<T::Hash>>;

    #[pallet::storage]
    #[pallet::getter(fn tokens)]
    pub(super) type Tokens<T: Config> = StorageValue<_, Vec<TokenInfo>>;

    #[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Pallet run from this pallet::call
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn insert_token(
            origin: OriginFor<T>, 
            token_info: TokenInfo
        ) -> DispatchResultWithPostInfo {
            let creator = ensure_signed(origin)?;
            
            match Self::create_token(&creator, &token_info) {
                Ok(new_token) => {
                    Self::deposit_event(Event::TokenCreated(new_token));

                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }

        #[pallet::weight(1_000)]
        pub fn remove_token(origin: OriginFor<T>, token_id: Vec<u8>) -> DispatchResultWithPostInfo {
            let _destroyer = ensure_signed(origin)?;

            match Self::delete_token(&token_id) {
                Ok(_) => {
                    Self::deposit_event(Event::TokenRemoved);

                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }
    }

    impl <T: Config> Pallet<T> {
        pub fn create_token(owner_id: &T::AccountId, token_info: &TokenInfo) -> Result<TokenDetail<HashOf<T>>, Error<T>> {
            let mut tokens = Self::tokens().unwrap_or(Vec::new());
            let token_count: u32 = tokens.len() as u32;
            let generate_id = Self::generate_id(&owner_id, &token_info, token_count);

            let token  = Self::token(token_info.token_id.clone());

            if token.is_some() {
                Err(Error::<T>::TokenAlreadyExist)?
            }

            let new_token = TokenDetail {
                id: generate_id.clone(),
                token_info: token_info.clone()
            };

            tokens.push(token_info.clone());

            Tokens::<T>::put(tokens);
            Token::<T>::insert(token_info.token_id.clone(), new_token.clone());

            Ok(new_token)
        }

        pub fn delete_token(token_id: &Vec<u8>) -> Result<(), Error<T>> {
            let tokens: Vec<TokenInfo> = Self::tokens().unwrap_or(Vec::new());

            let found_token = tokens
                .iter()
                .find(|token| token.token_id == token_id.clone());
                
            if found_token.is_none() {
                return Err(Error::<T>::TokenNotExist)?;
            }

            let updated_tokens: Vec<TokenInfo> = tokens
                .into_iter()
                .filter(|token| token.token_id != token_id.clone())
                .collect();

            Token::<T>::remove(token_id.clone());
            Tokens::<T>::put(updated_tokens);

            Ok(())
        }

        pub fn generate_id(owner_id: &T::AccountId, token_info: &TokenInfo, token_count: u32) -> HashOf<T> {
            let mut owner_id_bytes = owner_id.encode();
            let mut token_info_bytes = token_info.encode();
            let mut token_count_bytes = token_count.encode();

            owner_id_bytes.append(&mut token_info_bytes);
            owner_id_bytes.append(&mut token_count_bytes);

            let seed = &owner_id_bytes;

            T::Hashing::hash(seed)
        }
    }

    pub type HashOf<T> = <T as frame_system::Config>::Hash;
    pub type TokenId = Vec<u8>;

    #[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
    pub struct TokenDetail<Hash> {
        id: Hash,
        token_info: TokenInfo
    }

    #[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
    pub struct TokenInfo {
        token_id: Vec<u8>,
        token_name: Vec<u8>,
        token_decimal: u16,
        token_address_format: u16,
        token_rpc_address: Vec<u8>,
    }
}
