#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use sp_std::prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_support::{
        dispatch::DispatchResult,
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
        TokenCreated(Token<HashOf<T>>),
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
    #[pallet::getter(fn token_by_id)]
    pub(super) type TokenById<T: Config>  = StorageMap<_, Blake2_128Concat, Vec<u8>, Token<T::Hash>>;

    #[pallet::storage]
    #[pallet::getter(fn tokens)]
    pub(super) type Tokens<T: Config> = StorageValue<_, Vec<Token<T::Hash>>>;

    #[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Pallet run from this pallet::call
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1_000)]
        pub fn add_token(
            origin: OriginFor<T>, 
            token_id: Vec<u8>,
            token_name: Vec<u8>,
            token_decimal: u16,
            token_address_format: u16,
            token_rpc_address: Vec<u8>
        ) -> DispatchResult {
            let creator = ensure_signed(origin)?;
            let mut tokens = Self::tokens().unwrap_or(Vec::new());
            let token_count: u32 = tokens.len() as u32;
            let generate_id = Self::generate_id(&creator, &token_id, &token_name, token_decimal, token_address_format, &token_rpc_address, token_count);

            let new_token = Token {
                id: generate_id.clone(),
                token_id: token_id.clone(), 
                token_name: token_name.clone(), 
                token_decimal: token_decimal.clone(), 
                token_address_format: token_address_format.clone(), 
                token_rpc_address: token_rpc_address.clone()
            };

            let token  = Self::token_by_id(token_id.clone());

            ensure!(token == None, Error::<T>::TokenAlreadyExist);

            tokens.push(new_token.clone());

            Tokens::<T>::put(tokens);

            TokenById::<T>::insert(token_id.clone(), new_token.clone());

            Self::deposit_event(Event::TokenCreated(new_token));

            Ok(().into())
        }

        #[pallet::weight(1_000)]
        pub fn remove_token(origin: OriginFor<T>, token_id: Vec<u8>) -> DispatchResult {
            let _destroyer = ensure_signed(origin)?;
            let token  = Self::token_by_id(token_id.clone());

            ensure!(token != None, Error::<T>::TokenNotExist);

            TokenById::<T>::remove(token_id.clone());

            match Tokens::<T>::get() {
                None => {},
                Some(tokens) => {
                    let updated_tokens: Vec<Token<HashOf<T>>> = tokens
                        .into_iter()
                        .filter(|token| token.token_id != token_id.clone())
                        .collect();

                    Tokens::<T>::put(updated_tokens);

                    Self::deposit_event(Event::TokenRemoved)
                }
            }

            Ok(().into())
        }
    }

    impl <T: Config> Pallet<T> {
        pub fn generate_id(owner_id: &T::AccountId, token_id: &Vec<u8>, token_name: &Vec<u8>, token_decimal: u16, token_address_format: u16, token_rpc_address: &Vec<u8>, token_count: u32) -> HashOf<T> {
            let mut owner_id_bytes = owner_id.encode();
            let mut token_id_bytes = token_id.encode();
            let mut token_name_bytes = token_name.encode();
            let mut token_decimal_bytes = token_decimal.encode();
            let mut token_address_format_bytes = token_address_format.encode();
            let mut token_rpc_address_bytes = token_rpc_address.encode();
            let mut token_count_bytes = token_count.encode();

            owner_id_bytes.append(&mut token_id_bytes);
            owner_id_bytes.append(&mut token_name_bytes);
            owner_id_bytes.append(&mut token_decimal_bytes);
            owner_id_bytes.append(&mut token_address_format_bytes);
            owner_id_bytes.append(&mut token_rpc_address_bytes);
            owner_id_bytes.append(&mut token_count_bytes);

            let seed = &owner_id_bytes;

            T::Hashing::hash(seed)
        }
    }

    pub type HashOf<T> = <T as frame_system::Config>::Hash;

    #[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
    pub struct Token<Hash> {
        id: Hash,
        token_id: Vec<u8>,
        token_name: Vec<u8>,
        token_decimal: u16,
        token_address_format: u16,
        token_rpc_address: Vec<u8>,
    }
}
