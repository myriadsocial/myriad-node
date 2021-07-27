#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;

	#[pallet::config]
	pub trait Config:
		frame_system::Config + pallet_token::Config + pallet_platform::Config
	{
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CreateNewBalance(Person),
	}

	#[pallet::error]
	pub enum Error<T> {
		PostNotBelongToYou,
		BalanceNotBelongToYou,
		PlatformNotExist,
		TokenNotExist,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	pub type FreeBalanceOf = FreeBalance;
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type PeopleId = Vec<u8>;
	pub type PostId = Vec<u8>;

	#[pallet::storage]
	#[pallet::getter(fn post_balance)]
	pub(super) type PostBalance<T: Config> = StorageMap<_, Blake2_128Concat, PostId, Post>;

	#[pallet::storage]
	#[pallet::getter(fn person_balance)]
	pub(super) type PersonBalance<T: Config> = StorageMap<_, Blake2_128Concat, PeopleId, Person>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn insert_balance(
			origin: OriginFor<T>,
			post_info: PostInfo,
		) -> DispatchResultWithPostInfo {
			let _creator = ensure_signed(origin)?;

			match Self::create_balance(&post_info) {
				Ok((_, new_person)) => {
					Self::deposit_event(Event::CreateNewBalance(new_person));

					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn create_post(post_info: &PostInfo) -> Result<Post, Error<T>> {
			let get_token = pallet_token::Pallet::<T>::token(post_info.token_id.clone());

			if get_token.is_none() {
				return Err(Error::TokenNotExist)
			}

			let PostInfo { post_id, people_id, platform, token_id, free } = post_info.clone();

			let new_post: Post;
			let get_post_balance = Self::post_balance(post_id.clone());

			if get_post_balance.is_some() {
				let post_balance = get_post_balance.unwrap_or_default();

				if post_balance.people_id != people_id && post_balance.platform != platform {
					return Err(Error::PostNotBelongToYou)
				}

				let updated_balances: Vec<FreeBalance> =
					Self::get_balance(post_balance.balances, token_id, free);

				new_post = Post::new(post_info.clone(), updated_balances);
			} else {
				let platforms = pallet_platform::Pallet::<T>::platforms().unwrap_or_default();
				let found_platform =
					platforms.iter().find(|platform| platform == &&post_info.platform);

				if found_platform.is_none() {
					return Err(Error::<T>::PlatformNotExist)
				}

				new_post = Post::new(post_info.clone(), vec![FreeBalance { free, token_id }]);
			}

			<PostBalance<T>>::insert(post_id, new_post.clone());

			Ok(new_post)
		}

		pub fn create_person(post_info: &PostInfo) -> Result<Person, Error<T>> {
			let PostInfo { post_id: _post_id, people_id, platform: _platform, token_id, free } =
				post_info.clone();

			let new_person: Person;
			let get_person_balance = Self::person_balance(people_id.clone());

			if get_person_balance.is_none() {
				let platforms = pallet_platform::Pallet::<T>::platforms().unwrap_or_default();
				let found_platform =
					platforms.iter().find(|platform| platform == &&post_info.platform);

				if found_platform.is_none() {
					return Err(Error::<T>::PlatformNotExist)
				}

				new_person = Person::new(post_info.clone(), vec![FreeBalance { free, token_id }]);
			} else {
				let person_balance = get_person_balance.unwrap_or_default();
				let updated_balance: Vec<FreeBalance> =
					Self::get_balance(person_balance.balances, token_id, free);

				new_person = Person::new(post_info.clone(), updated_balance);
			}

			<PersonBalance<T>>::insert(people_id, new_person.clone());

			Ok(new_person)
		}

		pub fn get_balance(
			free_balances: Vec<FreeBalance>,
			token_id: Vec<u8>,
			free: u32,
		) -> Vec<FreeBalance> {
			let mut updated_balance: Vec<FreeBalance>;
			let found_balance = free_balances
				.clone()
				.into_iter()
				.find(|balance| balance.token_id == token_id.clone());

			if found_balance.is_none() {
				updated_balance = free_balances;
				updated_balance.push(FreeBalance { token_id, free });
			} else {
				updated_balance = free_balances
					.into_iter()
					.map(|balance| {
						if balance.token_id == token_id.clone() {
							return FreeBalance {
								token_id: token_id.clone(),
								free: balance.free + free,
							}
						}

						balance
					})
					.collect();
			}

			updated_balance
		}

		pub fn create_balance(post_info: &PostInfo) -> Result<(Post, Person), Error<T>> {
			let new_post: Post;
			let new_person: Person;

			match Self::create_post(post_info) {
				Ok(post) => {
					new_post = post;
				},
				Err(error) => return Err(error),
			}

			match Self::create_person(post_info) {
				Ok(person) => {
					new_person = person;
				},
				Err(error) => return Err(error),
			}

			Ok((new_post, new_person))
		}
	}

	#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
	pub struct Person {
		people_id: Vec<u8>,
		platform: Vec<u8>,
		balances: Vec<FreeBalance>,
	}

	#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
	pub struct FreeBalance {
		free: u32,
		token_id: Vec<u8>,
	}

	#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
	pub struct Post {
		post_id: Vec<u8>,
		people_id: Vec<u8>,
		platform: Vec<u8>,
		balances: Vec<FreeBalance>,
	}

	#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
	pub struct PostInfo {
		post_id: Vec<u8>,
		people_id: Vec<u8>,
		platform: Vec<u8>,
		token_id: Vec<u8>,
		free: u32,
	}

	impl Post {
		pub fn new(post_info: PostInfo, balances: Vec<FreeBalance>) -> Self {
			Self {
				post_id: post_info.post_id,
				people_id: post_info.people_id,
				platform: post_info.platform,
				balances,
			}
		}
	}

	impl Person {
		pub fn new(post_info: PostInfo, balances: Vec<FreeBalance>) -> Self {
			Self { people_id: post_info.people_id, platform: post_info.platform, balances }
		}
	}
}
