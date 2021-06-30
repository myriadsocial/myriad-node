#![cfg_attr(not(feature = "std"), no_std)]

#[frame_support::pallet]
pub mod pallet {
    use frame_system::pallet_prelude::*;
    use frame_support::{
        dispatch::DispatchResultWithPostInfo,
        pallet_prelude::*,
		traits::{Currency, Imbalance, OnUnbalanced, ReservableCurrency},
    };

	// balance type using reservable currency type
	type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	type PositiveImbalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
	>>::PositiveImbalance;
	type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
	>>::NegativeImbalance;

    #[pallet::config]
    pub trait Config: frame_system::Config + Sized {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Currency type for this pallet.
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

		/// Handler for the unbalanced increment when rewarding (minting rewards)
		type Reward: OnUnbalanced<PositiveImbalanceOf<Self>>;

		/// Handler for the unbalanced decrement when slashing (burning collateral)
		type Slash: OnUnbalanced<NegativeImbalanceOf<Self>>;
    }

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        SlashFunds(<T as frame_system::Config>::AccountId, BalanceOf<T>, <T as frame_system::Config>::BlockNumber),
		RewardFunds(<T as frame_system::Config>::AccountId, BalanceOf<T>, <T as frame_system::Config>::BlockNumber),
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Pallet run from this pallet::call
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn slash_funds(
            origin: OriginFor<T>,
            to_punish: T::AccountId,
			collateral: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;

			let imbalance = T::Currency::slash_reserved(&to_punish, collateral).0;
			T::Slash::on_unbalanced(imbalance);

			let now = <frame_system::Module<T>>::block_number();
			Self::deposit_event(Event::SlashFunds(to_punish, collateral, now));
			Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn reward_funds(
			origin: OriginFor<T>,
			to_reward: T::AccountId,
			reward: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;

            let mut total_imbalance = <PositiveImbalanceOf<T>>::zero();

			let r = T::Currency::deposit_into_existing(&to_reward, reward).ok();
			total_imbalance.maybe_subsume(r);
			T::Reward::on_unbalanced(total_imbalance);

			let now = <frame_system::Module<T>>::block_number();
			Self::deposit_event(Event::RewardFunds(to_reward, reward, now));
			Ok(().into())
        }
    }
}
