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
    pub trait Config: frame_system::Config + pallet_platform::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        NewCredentialCreated(T::AccountId, UserCredentialInfo<T::AccountId>),
        CredentialRemoved(PeopleId),
        UserCredentialsUpdated(T::AccountId)
    }

    #[pallet::error]
    pub enum Error<T> {
        CredentialAlreadyExist,
        CredentialAlreadyBelong,
        MaximumCredentialExceeded,
        CredentialNotBelong,
        CredentialNotExist,
        PlatformNotExist,
        WrongCredential
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn credentials)]
    pub(super) type Credentials<T: Config> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, UserCredential<HashOf<T>, AccountIdOf<T>>>;

    #[pallet::storage] 
	#[pallet::getter(fn credential)]
    pub(super) type Credential<T: Config> = StorageMap<_, Blake2_128Concat, PeopleId, UserCredentialInfo<AccountIdOf<T>>>;

    #[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn insert_credential(
            origin: OriginFor<T>, 
            credential_info: UserCredentialInfo<AccountIdOf<T>>
        ) -> DispatchResultWithPostInfo {
            let creator = ensure_signed(origin)?;

            match Self::create_credential(&creator, &credential_info) {
                Ok(user_credential) => {
                    Self::deposit_event(Event::NewCredentialCreated(creator, user_credential));
                    
                    Ok(().into())
                },
                Err(error) => Err(error)? 
            }
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn remove_credential(origin: OriginFor<T>, people_id: Vec<u8>) -> DispatchResultWithPostInfo {
            let destroyer = ensure_signed(origin)?;

            match Self::delete_credential(&destroyer, &people_id) {
                Ok(people_id) => {
                    Self::deposit_event(Event::CredentialRemoved(people_id.clone()));
                    Self::deposit_event(Event::UserCredentialsUpdated(destroyer.clone()));
        
                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn create_credential(
            owner_id: &T::AccountId,
            credential_info: &UserCredentialInfo<AccountIdOf<T>>
        ) -> Result<UserCredentialInfo<AccountIdOf<T>>, Error<T>> {
            if owner_id.clone() != credential_info.user_id {
                return Err(Error::<T>::WrongCredential);
            } 

            let platforms = pallet_platform::Pallet::<T>::platforms().unwrap_or(Vec::new());

            let found_platform = platforms
                .clone()
                .into_iter()
                .find(|el| el == &credential_info.platform);

            if found_platform.is_none() {
                return Err(Error::<T>::PlatformNotExist)?;
            }

            if Self::credential(credential_info.people_id.clone()).is_some() {
                return Err(Error::<T>::CredentialAlreadyExist)?;
            }

            let credential_owner = Self::credentials(owner_id.clone());
            let credential_id: HashOf<T>;
            let new_credential: UserCredential<HashOf<T>, AccountIdOf<T>>;
            let new_credential_info: UserCredentialInfo<AccountIdOf<T>>;
            let mut credentials: Vec<UserCredentialInfo<AccountIdOf<T>>>;

            if credential_owner.is_none() {
                new_credential_info = UserCredentialInfo {
                    user_id: owner_id.clone(),
                    people_id: credential_info.people_id.clone(),
                    platform: credential_info.platform.clone()
                };
                credentials = vec![new_credential_info.clone()];
                credential_id = Self::generate_credential_id(&owner_id, &credential_info, 0);
                new_credential = UserCredential {
                    id: credential_id,
                    owner_id: owner_id.clone(),
                    credentials: credentials.clone()
                };
            } else {
                let credential_owner = credential_owner.unwrap();
                let credential_count: u16 = credential_owner.credentials.len() as u16;

                if credential_count >= platforms.len() as u16 {
                    return Err(Error::<T>::MaximumCredentialExceeded)?;
                }

                if credential_owner.credentials
                    .iter()
                    .find(|credential| credential.platform == credential_info.platform)
                    .is_some() 
                {
                    return Err(Error::<T>::CredentialAlreadyExist)?;
                }
                
                new_credential_info = UserCredentialInfo {
                    user_id: owner_id.clone(),
                    people_id: credential_info.people_id.clone(),
                    platform: credential_info.platform.clone()
                };
                credential_id = Self::generate_credential_id(&owner_id, &credential_info, credential_count);
                credentials = credential_owner.credentials;
                credentials.push(new_credential_info.clone());
                new_credential = UserCredential {
                    id: credential_id,
                    owner_id: owner_id.clone(),
                    credentials: credentials.clone()
                };
            }

            Credential::<T>::insert(&credential_info.people_id, &new_credential_info);
            Credentials::<T>::insert(&owner_id, &new_credential);
            
            Ok(new_credential_info)
        }

        pub fn delete_credential(owner_id: &T::AccountId, people_id: &Vec<u8>) -> Result<Vec<u8>, Error<T>> {
            let credential = Self::credentials(&owner_id);

            if credential.is_none() {
                return Err(Error::<T>::CredentialNotExist)?;
            }

            let credential = credential.unwrap();
            let credentials = credential.credentials;
            let found_credential = credentials
                .iter()
                .find(|credential| credential.people_id == people_id.clone());

            if found_credential.is_none() {
                return Err(Error::<T>::CredentialNotExist)?;
            }

            if found_credential.unwrap().user_id != owner_id.clone() {
                return Err(Error::<T>::CredentialNotBelong)?;
            }

            let new_credentials: Vec<UserCredentialInfo<AccountIdOf<T>>> = credentials
                .into_iter()
                .filter(|credential| credential.people_id != people_id.clone())
                .collect();

            Credential::<T>::remove(&people_id);
            Credentials::<T>::insert(&owner_id, UserCredential {
                id: credential.id,
                owner_id: credential.owner_id,
                credentials: new_credentials

            });

            Ok(people_id.clone())
        }

        pub fn generate_credential_id(owner_id: &T::AccountId, credential_info: &UserCredentialInfo<AccountIdOf<T>>, credential_count: u16) -> HashOf<T> {
            let mut account_id_bytes = owner_id.encode();
            let mut credential_info_bytes = credential_info.encode();
            let mut credential_count_byte = credential_count.encode();

            account_id_bytes.append(&mut credential_info_bytes);
            account_id_bytes.append(&mut credential_count_byte);

            let seed = &account_id_bytes;
            T::Hashing::hash(seed)
        }
    }

    pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    pub type HashOf<T> = <T as frame_system::Config>::Hash;
    pub type UserCredentialOf<T> = UserCredential<HashOf<T>, AccountIdOf<T>>;
    pub type PeopleId = Vec<u8>;

    #[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
    pub struct UserCredential<Hash, AccountId> {
        id: Hash,
        owner_id: AccountId,
        credentials: Vec<UserCredentialInfo<AccountId>>
    }

    #[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
    pub struct UserCredentialInfo<AccountId> {
        user_id: AccountId,
        people_id: Vec<u8>,
        platform: Vec<u8>
    }

    impl <Hash, AccountId> UserCredential<Hash, AccountId> {
        pub fn new(
            id: Hash,
            owner_id: AccountId,
            credentials: Vec<UserCredentialInfo<AccountId>>
        ) -> Self {
            Self {id, owner_id, credentials}
        }
    }
}
