#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use sp_std::prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*
    };

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_token::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
		CreateNewPost(Post),
    }

    #[pallet::error]
    pub enum Error<T> {
        PostNotBelongToYou,
        BalanceNotBelongToYou,
        TokenNotExist
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    pub type FreeBalanceOf = FreeBalance;
    pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

    #[pallet::storage]
    #[pallet::getter(fn post_by_people)]
    pub(super) type PostByPeople<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<Post>>;

    #[pallet::storage] 
	#[pallet::getter(fn post_by_id)]
    pub(super) type PostById<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, Post>;

    #[pallet::storage]
    #[pallet::getter(fn person_balance)]
    pub(super) type PersonBalance<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, Person>;

    #[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn insert_post(
            origin: OriginFor<T>, 
            post_id: Vec<u8>,
            people_id: Vec<u8>,
            platform: Vec<u8>,
            token_id: Vec<u8>,
            free: u32 
        ) -> DispatchResult {
			let _creator = ensure_signed(origin)?;

            match Self::create_post(
                &post_id, 
                &people_id,
                &platform, 
                &token_id, 
                free
            ) {
                Ok(new_post) => {
                    let new_people_post = Self::create_people_post(people_id.clone(), new_post.clone());

                    <PostById<T>>::insert(post_id.clone(), new_post.clone());
                    
                    <PostByPeople<T>>::insert(people_id.clone(), new_people_post.clone());
 
			        Self::deposit_event(Event::CreateNewPost(new_post.clone()));
                },
                Err(err) => {
                    match err {
                        Error::PostNotBelongToYou => ensure!(false, Error::<T>::PostNotBelongToYou),
                        Error::TokenNotExist => ensure!(false, Error::<T>::TokenNotExist),
                        _ => {}
                    }
                }
            }

			Ok(())
		}

        #[pallet::weight(10_000)]
        pub fn insert_balance(
            origin: OriginFor<T>,
            people_id: Vec<u8>, 
            post_id: Vec<u8>,
            platform: Vec<u8>,
            token_id: Vec<u8>,
            free: u32, 
        ) -> DispatchResult {
            let _creator = ensure_signed(origin)?;

            match Self::create_post(
                &post_id, 
                &people_id, 
                &platform, 
                &token_id, 
                free
            ) {
                Ok(new_post) => {
                    <PostById<T>>::insert(post_id.clone(), new_post.clone());

			        Self::deposit_event(Event::CreateNewPost(new_post.clone()));
                },
                Err(err) => {
                    match err {
                        Error::PostNotBelongToYou => ensure!(false, Error::<T>::PostNotBelongToYou),
                        Error::TokenNotExist => ensure!(false, Error::<T>::TokenNotExist),
                        _ => {}
                    }
                }
            }
            
            let person_balance: Person;

            match PersonBalance::<T>::get(people_id.clone()) {
                None => {
                    person_balance = Person {
                        people_id: people_id.clone(),
                        platform: platform.clone(),
                        balances: vec![
                            FreeBalance {
                                free,
                                token_id: token_id.clone()
                            }
                        ]
                    };
                },
                Some(person) => {
                    ensure!(person.people_id == people_id.clone(), Error::<T>::BalanceNotBelongToYou);
                    ensure!(person.platform == platform.clone(), Error::<T>::BalanceNotBelongToYou);

                    let mut updated_balance: Vec<FreeBalance>;
                    let found_balance = person.balances
                        .clone()
                        .into_iter()
                        .find(|balance| balance.token_id == token_id.clone());


                    if found_balance == None {
                        updated_balance = person.balances.clone();
                        updated_balance.push(FreeBalance {
                            token_id: token_id.clone(),
                            free: free
                        })
                    } else {
                        updated_balance = person.balances
                            .into_iter()
                            .map(|balance| {
                                if balance.token_id == token_id.clone() {
                                    return FreeBalance {
                                        token_id: token_id.clone(),
                                        free: balance.free + free
                                    }
                                }
        
                                balance
                            })
                            .collect();     
                    }

                    person_balance = Person {
                        people_id: people_id.clone(),
                        platform: platform.clone(),
                        balances: updated_balance
                    }
                }
            }

            <PersonBalance<T>>::insert(people_id, person_balance);
            
            Ok(())
        }
    }

    impl <T: Config> Pallet <T> {
        pub fn create_post(
            post_id: &Vec<u8>,
            people_id: &Vec<u8>,
            platform: &Vec<u8>,
            token_id: &Vec<u8>,
            free: u32 
        ) -> Result<Post, Error<T>> {
            let new_post: Post;

            let get_token = pallet_token::Pallet::<T>::token_by_id(token_id.clone());

            if get_token == None {
                return Err(Error::TokenNotExist);
            }

            match PostById::<T>::get(post_id.clone()) {
                None => {
                    new_post = Post {
                        post_id: post_id.clone(),
                        people_id: people_id.clone(),
                        platform: platform.clone(),
                        balances: vec![
                            FreeBalance {
                                free: free,
                                token_id: token_id.clone()
                            }
                        ]
                    };
                },
                Some(post) => {
                    if post.people_id != people_id.clone() {
                        return Err(Error::PostNotBelongToYou);
                    }

                    if post.platform != platform.clone() {
                        return Err(Error::PostNotBelongToYou);
                    }

                    let mut updated_balances: Vec<FreeBalance>;

                    let found_balance = post.balances
                        .clone()
                        .into_iter()
                        .find(|balance| balance.token_id == token_id.clone());

                    if found_balance == None {
                        updated_balances = post.balances.clone();
                        updated_balances.push(FreeBalance {
                            token_id: token_id.clone(),
                            free: free
                        });
                    } else {
                        updated_balances = post.balances
                            .into_iter()
                            .map(|balance| {
                                if balance.token_id == token_id.clone() {
                                    return FreeBalance {
                                        token_id: token_id.clone(),
                                        free: balance.free + free
                                    }
                                }

                                balance
                            })
                            .collect();
                    }

                    new_post = Post {
                        post_id: post_id.clone(),
                        people_id: people_id.clone(),
                        platform: platform.clone(),
                        balances: updated_balances
                    }
                }
            }

            Ok(new_post)
        }

        pub fn create_people_post(
            people_id: Vec<u8>, 
            new_post: Post
        ) -> Vec<Post> {
            let mut filter_posts: Vec<Post>;

            match PostByPeople::<T>::get(people_id.clone()) {
                None => {
                    filter_posts = vec![new_post];
                },
                Some(posts) => {
                    let found_post = posts
                        .clone()
                        .into_iter()
                        .find(|post| post.post_id == new_post.post_id.clone());

                    if found_post == None {
                        filter_posts = posts.clone();
                        filter_posts.push(new_post.clone()); 
                    } else {
                        filter_posts = posts
                            .into_iter()
                            .map(|post| {
                                if post.post_id == new_post.post_id.clone() {
                                    return new_post.clone();
                                }

                                post
                            })
                            .collect();
                    }
                }
            }

            filter_posts
        }
    }

    #[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
    pub struct Person {
        people_id: Vec<u8>,
        platform: Vec<u8>,
        balances: Vec<FreeBalance>
    }

    #[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
    pub struct FreeBalance {
        free: u32,
        token_id: Vec<u8>
    }

    #[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
    pub struct Post {
        post_id: Vec<u8>,
        people_id: Vec<u8>,
        platform: Vec<u8>,
        balances: Vec<FreeBalance>
    }
}
