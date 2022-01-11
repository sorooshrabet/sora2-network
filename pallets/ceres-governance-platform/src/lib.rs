#![cfg_attr(not(feature = "std"), no_std)]
#![feature(destructuring_assignment)]

pub mod weights;

mod benchmarking;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use codec::{Decode, Encode};
use frame_support::weights::Weight;

pub trait WeightInfo {
    fn vote() -> Weight;
    fn create_poll() -> Weight;
    fn withdraw() -> Weight;
}

#[derive(Encode, Decode, Default, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct VotingInfo<Balance> {
    /// Voting option
    voting_option: u32,
    /// Number of votes
    number_of_votes: Balance,
    /// Ceres withdrawn
    ceres_withdrawn: bool,
}

#[derive(Encode, Decode, Default, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct PollInfo<BlockNumber> {
    /// Number of options
    number_of_options: u32,
    /// Poll start block
    poll_start_block: BlockNumber,
    /// Poll end block
    poll_end_block: BlockNumber,
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use crate::{PollInfo, VotingInfo, WeightInfo};
    use common::prelude::Balance;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::Vec;
    use frame_system::ensure_signed;
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::AccountIdConversion;
    use sp_runtime::ModuleId;

    const PALLET_ID: ModuleId = ModuleId(*b"crsgvrnc");

    #[pallet::config]
    pub trait Config: frame_system::Config + assets::Config + technical::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Ceres asset id
        type CeresAssetId: Get<Self::AssetId>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    type Assets<T> = assets::Pallet<T>;
    pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    /// Poll_id -> Account_id -> VotingInfo
    #[pallet::storage]
    #[pallet::getter(fn votings)]
    pub type Voting<T: Config> = StorageDoubleMap<
        _,
        Identity,
        Vec<u8>,
        Identity,
        AccountIdOf<T>,
        VotingInfo<Balance>,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn poll_data)]
    pub type PollData<T: Config> =
        StorageMap<_, Identity, Vec<u8>, PollInfo<T::BlockNumber>, ValueQuery>;

    #[pallet::event]
    #[pallet::metadata(AccountIdOf<T> = "AccountId", T::BlockNumber = "BlockNumber")]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Voting [who, option, balance]
        Voted(AccountIdOf<T>, u32, Balance),
        /// Create poll [who, option, start_block, end_block]
        Created(AccountIdOf<T>, u32, T::BlockNumber, T::BlockNumber),
        /// Withdrawn [who, balance]
        Withdrawn(AccountIdOf<T>, Balance),
    }

    #[pallet::error]
    pub enum Error<T> {
        ///Invalid Votes
        InvalidVotes,
        ///Poll Is Finished
        PollIsFinished,
        ///Poll Is Not Started
        PollIsNotStarted,
        ///Not Enough Funds
        NotEnoughFunds,
        ///Invalid Number Of Option
        InvalidNumberOfOption,
        ///Vote Denied
        VoteDenied,
        ///Invalid Start Block
        InvalidStartBlock,
        ///Invalid End Block
        InvalidEndBlock,
        ///Poll Is Not Finished
        PollIsNotFinished,
        ///InvalidNumberOfVotes
        InvalidNumberOfVotes,
        ///FundsAlreadyWithdrawn,
        FundsAlreadyWithdrawn,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Voting for option
        #[pallet::weight(<T as Config>::WeightInfo::vote())]
        pub fn vote(
            origin: OriginFor<T>,
            poll_id: Vec<u8>,
            voting_option: u32,
            number_of_votes: Balance,
        ) -> DispatchResultWithPostInfo {
            let user = ensure_signed(origin)?;

            ensure!(number_of_votes > 0, Error::<T>::InvalidNumberOfVotes);

            let poll_info = PollData::<T>::get(&poll_id);
            let current_block = frame_system::Pallet::<T>::block_number();

            ensure!(
                current_block >= poll_info.poll_start_block,
                Error::<T>::PollIsNotStarted
            );

            ensure!(
                current_block <= poll_info.poll_end_block,
                Error::<T>::PollIsFinished
            );

            ensure!(
                voting_option <= poll_info.number_of_options && voting_option != 0,
                Error::<T>::InvalidNumberOfOption
            );

            let mut voting_info = <Voting<T>>::get(&poll_id, &user);

            if voting_info.voting_option == 0 {
                voting_info.voting_option = voting_option;
            } else {
                ensure!(
                    voting_info.voting_option == voting_option,
                    Error::<T>::VoteDenied
                )
            }

            ensure!(
                number_of_votes
                    <= Assets::<T>::free_balance(&T::CeresAssetId::get(), &user).unwrap_or(0),
                Error::<T>::NotEnoughFunds
            );

            voting_info.number_of_votes += number_of_votes;

            // Transfer Ceres to pallet
            Assets::<T>::transfer_from(
                &T::CeresAssetId::get().into(),
                &user,
                &Self::account_id(),
                number_of_votes,
            )?;

            // Update storage
            <Voting<T>>::insert(&poll_id, &user, voting_info);

            //Emit event
            Self::deposit_event(Event::<T>::Voted(user, voting_option, number_of_votes));

            // Return a successful DispatchResult
            Ok(().into())
        }

        /// Create Poll
        #[pallet::weight(<T as Config>::WeightInfo::create_poll())]
        pub fn create_poll(
            origin: OriginFor<T>,
            poll_id: Vec<u8>,
            number_of_options: u32,
            poll_start_block: T::BlockNumber,
            poll_end_block: T::BlockNumber,
        ) -> DispatchResultWithPostInfo {
            let user = ensure_signed(origin)?;

            let current_block = frame_system::Pallet::<T>::block_number();

            ensure!(number_of_options >= 2, Error::<T>::InvalidNumberOfOption);

            ensure!(
                poll_start_block >= current_block,
                Error::<T>::InvalidStartBlock
            );

            ensure!(
                poll_end_block > poll_start_block,
                Error::<T>::InvalidEndBlock
            );

            let poll_info = PollInfo {
                number_of_options,
                poll_end_block,
                poll_start_block,
            };

            <PollData<T>>::insert(&poll_id, poll_info);

            //Emit event
            Self::deposit_event(Event::<T>::Created(
                user,
                number_of_options,
                poll_start_block,
                poll_end_block,
            ));

            // Return a successful DispatchResult
            Ok(().into())
        }

        /// Withdraw
        #[pallet::weight(<T as Config>::WeightInfo::withdraw())]
        pub fn withdraw(origin: OriginFor<T>, poll_id: Vec<u8>) -> DispatchResultWithPostInfo {
            let user = ensure_signed(origin)?;

            let poll_info = PollData::<T>::get(&poll_id);
            let current_block = frame_system::Pallet::<T>::block_number();

            ensure!(
                current_block > poll_info.poll_end_block,
                Error::<T>::PollIsNotFinished
            );

            // Update storage
            let mut voting_info = <Voting<T>>::get(&poll_id, &user);
            ensure!(
                voting_info.ceres_withdrawn == false,
                Error::<T>::FundsAlreadyWithdrawn
            );

            // Withdraw CERES
            Assets::<T>::transfer_from(
                &T::CeresAssetId::get().into(),
                &Self::account_id(),
                &user,
                voting_info.number_of_votes,
            )?;

            voting_info.ceres_withdrawn = true;
            <Voting<T>>::insert(&poll_id, &user, &voting_info);

            //Emit event
            Self::deposit_event(Event::<T>::Withdrawn(user, voting_info.number_of_votes));

            // Return a successful DispatchResult
            Ok(().into())
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    impl<T: Config> Pallet<T> {
        /// The account ID of pallet
        fn account_id() -> T::AccountId {
            PALLET_ID.into_account()
        }
    }
}
