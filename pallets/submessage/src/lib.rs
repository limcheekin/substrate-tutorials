#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::collections::btree_map::BTreeMap;
	use sp_std::vec::Vec;
	use log;
	
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type MaxBytesInChannelId: Get<u32>;
		type MaxBytesInCommonKey: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);


	#[pallet::storage]
	#[pallet::getter(fn common_key_by_channel_id_account_id)]
	pub type CommonKeyByChannelIdAccountId<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxBytesInChannelId>, // channel_id
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<u8, T::MaxBytesInCommonKey>, // common_key
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ChannelCreated(BoundedVec<u8, T::MaxBytesInChannelId>, Vec<T::AccountId>),
		AccountsAddedToChannel(BoundedVec<u8, T::MaxBytesInChannelId>, Vec<T::AccountId>),
		AccountRemovedFromChannel(BoundedVec<u8, T::MaxBytesInChannelId>, T::AccountId),
		ChannelRemoved(BoundedVec<u8, T::MaxBytesInChannelId>),
	}

	#[pallet::error]
	pub enum Error<T> {
		CommonKeyOfOriginRequired,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// #[pallet::weight(100_000 + T::DbWeight::get().reads_writes(5,7))]
		#[pallet::weight(100_000)]
		pub fn new_channel(
			origin: OriginFor<T>,
			channel_id: BoundedVec<u8, T::MaxBytesInChannelId>, 
			account_common_keys: BTreeMap<T::AccountId, BoundedVec<u8, T::MaxBytesInCommonKey>>
		) -> DispatchResultWithPostInfo {
			let from = ensure_signed(origin)?;

			log::debug!("new_conversation channel_id {:?}", channel_id);
			log::debug!("account_common_keys {:?}", account_common_keys);
			
			// validate the common key of origin exists
			ensure!(account_common_keys.contains_key(&from), Error::<T>::CommonKeyOfOriginRequired);
			
			for (account_id, common_key) in &account_common_keys {
				<CommonKeyByChannelIdAccountId<T>>::insert(&channel_id, account_id, common_key);	
			}

			Self::deposit_event(Event::ChannelCreated(channel_id, account_common_keys.into_keys().collect()));

			Ok(().into())
		}
	}
}
