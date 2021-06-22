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
        NewCredentialCreated(T::AccountId, Credential<T::Hash, T::AccountId>),
        CredentialRemoved(T::AccountId),
        UserCredentialsUpdated(T::AccountId)
    }

    #[pallet::error]
    pub enum Error<T> {
        CredentialAlreadyExist,
        CredentialAlreadyBelong,
        MaximumCredentialExceeded,
        CredentialNotBelong,
        CredentialNotExist,
        PlatformNotExist
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn credential_by_owner)]
    pub(super) type Credentials<T: Config> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Vec<Credential<HashOf<T>, AccountIdOf<T>>>>;

    #[pallet::storage] 
	#[pallet::getter(fn credential_by_people)]
    pub(super) type CredentialByPeople<T: Config> = StorageMap<_, Blake2_128Concat, PeopleId, Credential<HashOf<T>, AccountIdOf<T>>>;

    #[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn add_credential(
            origin: OriginFor<T>, 
            people_id: Vec<u8>, 
            platform: Vec<u8>
        ) -> DispatchResult {
            let creator = ensure_signed(origin)?;
            let platforms: Vec<Vec<u8>> = vec![
                "twitter".as_bytes().to_vec(),
                "facebook".as_bytes().to_vec(),
                "reddit".as_bytes().to_vec()
            ];

            let found_platform = platforms.iter().find(|x| x == &&platform);

            ensure!(found_platform != None, Error::<T>::PlatformNotExist);

            let found_credential_by_people = Self::credential_by_people(people_id.clone());

            ensure!(found_credential_by_people == None, Error::<T>::CredentialAlreadyExist);

            let mut credentials = Self::credential_by_owner(&creator).unwrap_or(Vec::new());

            ensure!(credentials.len() < 3, Error::<T>::MaximumCredentialExceeded);

            let new_credential = Self::create_credential(&creator, &people_id, &platform);

            credentials.push(new_credential.clone());

            Credentials::<T>::insert(&creator, &credentials);

            Self::deposit_event(Event::NewCredentialCreated(creator, new_credential));

            Ok(().into())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn remove_credential(origin: OriginFor<T>, people_id: Vec<u8>) -> DispatchResult {
            let destroyer = ensure_signed(origin)?;

            match CredentialByPeople::<T>::get(&people_id) {
                None => {
                    ensure!(!true, Error::<T>::CredentialNotExist);
                },
                Some(credential) => {
                    ensure!(credential.owner_id == destroyer, Error::<T>::CredentialNotBelong);
                    
                    CredentialByPeople::<T>::remove(&credential.people_id);

                    Self::deposit_event(Event::CredentialRemoved(destroyer.clone()));
                }
            }

            match Credentials::<T>::get(&destroyer) {
                None => {},
                Some(credentials) => {
                    let new_credential: Vec<Credential<HashOf<T>, AccountIdOf<T>>> = credentials
                        .into_iter()
                        .filter(|credential| credential.people_id != people_id)
                        .collect();

                    Credentials::<T>::insert(&destroyer, &new_credential);

                    Self::deposit_event(Event::UserCredentialsUpdated(destroyer.clone()))
                }
            }

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn create_credential(
            owner_id: &T::AccountId,
            people_id: &Vec<u8>,
            platform: &Vec<u8>
        ) -> Credential<HashOf<T>, AccountIdOf<T>> {
            let credentials_owner = Self::credential_by_owner(owner_id.clone()).unwrap_or(Vec::new());

            let credential_count: u16 = credentials_owner.len() as u16; 

            let credential_id = Self::generate_credential_id(&owner_id, &people_id, &platform, credential_count);

            let new_credential = Credential::new(credential_id.clone(), owner_id.clone(), people_id.clone(), platform.clone());

            CredentialByPeople::<T>::insert(&people_id, &new_credential);
            
            new_credential
        }

        pub fn generate_credential_id(owner_id: &T::AccountId, people_id: &Vec<u8>, platform: &Vec<u8>, credential_count: u16) -> HashOf<T> {
            let mut account_id_bytes = owner_id.encode();
            let mut people_id_byte = people_id.encode();
            let mut platform_byte = platform.encode();
            let mut credential_count_byte = credential_count.encode();

            account_id_bytes.append(&mut people_id_byte);
            account_id_bytes.append(&mut platform_byte);
            account_id_bytes.append(&mut credential_count_byte);

            let seed = &account_id_bytes;
            T::Hashing::hash(seed)
        }
    }

    pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    pub type HashOf<T> = <T as frame_system::Config>::Hash;
    pub type PeopleId = Vec<u8>;

    #[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
    pub struct Credential<Hash, AccountId> {
        id: Hash,
        owner_id: AccountId,
        people_id: Vec<u8>,
        platform: Vec<u8>
    }

    impl <Hash, AccountId> Credential<Hash, AccountId> {
        pub fn new(
            id: Hash,
            owner_id: AccountId,
            people_id: Vec<u8>,
            platform: Vec<u8>
        ) -> Self {
            Self {id, owner_id, people_id, platform}
        }
    }
}
