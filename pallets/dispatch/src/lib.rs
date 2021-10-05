#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch::{DispatchResult, Dispatchable, Parameter};
use frame_support::log::{debug, warn};
use frame_support::traits::{Contains, EnsureOrigin};
use frame_support::weights::GetDispatchInfo;
use snowbridge_core::MessageDispatch;
use sp_core::H160;

use sp_core::RuntimeDebug;

use codec::{Decode, Encode};

pub struct EnsureEthereumAccount;

impl<OuterOrigin> EnsureOrigin<OuterOrigin> for EnsureEthereumAccount
where
    OuterOrigin: Into<Result<Origin, OuterOrigin>> + From<Origin>,
{
    type Success = H160;

    fn try_origin(o: OuterOrigin) -> Result<Self::Success, OuterOrigin> {
        o.into().and_then(|o| Ok(o.0))
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn successful_origin() -> OuterOrigin {
        OuterOrigin::from(Origin(H160::repeat_byte(2)))
    }
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    pub type MessageIdOf<T> = <T as Config>::MessageId;
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The overarching origin type.
        type Origin: From<Origin>;

        /// Id of the message. Whenever message is passed to the dispatch module, it emits
        /// event with this id + dispatch result.
        type MessageId: Parameter;

        /// The overarching dispatch call type.
        type Call: Parameter
            + GetDispatchInfo
            + Dispatchable<
                Origin = <Self as Config>::Origin,
                PostInfo = frame_support::dispatch::PostDispatchInfo,
            >;

        /// The pallet will filter all incoming calls right before they're dispatched. If this filter
        /// rejects the call, special event (`Event::MessageRejected`) is emitted.
        type CallFilter: Contains<<Self as Config>::Call>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::event]
    #[pallet::metadata(MessageIdOf<T> = "MessageId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    /// Events for the Bridge module.
    pub enum Event<T: Config> {
        /// Message has been dispatched with given result.
        MessageDispatched(MessageIdOf<T>, DispatchResult),
        /// Message has been rejected
        MessageRejected(MessageIdOf<T>),
        /// We have failed to decode a Call from the message.
        MessageDecodeFailed(MessageIdOf<T>),
    }

    impl<T: Config> MessageDispatch<T, MessageIdOf<T>> for Pallet<T> {
        fn dispatch(source: H160, id: MessageIdOf<T>, payload: &[u8]) {
            let call = match <T as Config>::Call::decode(&mut &payload[..]) {
                Ok(call) => call,
                Err(_) => {
                    warn!("Failed to decode call");
                    Self::deposit_event(Event::MessageDecodeFailed(id));
                    return;
                }
            };
            debug!("Decoded call: {:?}", call);

            if !T::CallFilter::contains(&call) {
                Self::deposit_event(Event::MessageRejected(id));
                return;
            }

            let origin = Origin(source).into();
            let result = call.dispatch(origin);

            Self::deposit_event(Event::MessageDispatched(
                id,
                result.map(drop).map_err(|e| e.error),
            ));
        }

        #[cfg(feature = "runtime-benchmarks")]
        fn successful_dispatch_event(id: MessageIdOf<T>) -> Option<<T as system::Config>::Event> {
            let event: <T as Config>::Event = RawEvent::MessageDispatched(id, Ok(())).into();
            Some(event.into())
        }
    }

    #[pallet::origin]
    #[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug)]
    pub struct Origin(pub H160);

    impl From<H160> for Origin {
        fn from(hash: H160) -> Origin {
            Origin(hash)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::dispatch::DispatchError;
    use frame_support::parameter_types;
    use frame_support::traits::Everything;
    use frame_system::{EventRecord, Phase};
    use sp_core::{H160, H256};
    use sp_runtime::testing::Header;
    use sp_runtime::traits::{BlakeTwo256, IdentityLookup};

    use crate as dispatch;

    type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
    type Block = frame_system::mocking::MockBlock<Test>;

    frame_support::construct_runtime!(
        pub enum Test where
            Block = Block,
            NodeBlock = Block,
            UncheckedExtrinsic = UncheckedExtrinsic,
        {
            System: frame_system::{Pallet, Call, Storage, Event<T>},
            Dispatch: dispatch::{Pallet, Storage, Origin, Event<T>},
        }
    );

    type AccountId = u64;

    parameter_types! {
        pub const BlockHashCount: u64 = 250;
    }

    impl frame_system::Config for Test {
        type Origin = Origin;
        type Index = u64;
        type Call = Call;
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = AccountId;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type Event = Event;
        type BlockHashCount = BlockHashCount;
        type Version = ();
        type PalletInfo = PalletInfo;
        type AccountData = ();
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type SystemWeightInfo = ();
        type BlockWeights = ();
        type BlockLength = ();
        type DbWeight = ();
        type SS58Prefix = ();
        type OnSetCode = ();
        type BaseCallFilter = Everything;
    }

    pub struct CallFilter;
    impl Contains<Call> for CallFilter {
        fn contains(call: &Call) -> bool {
            match call {
                Call::System(frame_system::pallet::Call::<Test>::remark(_)) => true,
                _ => false,
            }
        }
    }

    impl dispatch::Config for Test {
        type Origin = Origin;
        type Event = Event;
        type MessageId = u64;
        type Call = Call;
        type CallFilter = CallFilter;
    }

    fn new_test_ext() -> sp_io::TestExternalities {
        let t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();
        sp_io::TestExternalities::new(t)
    }

    #[test]
    fn test_dispatch_bridge_message() {
        new_test_ext().execute_with(|| {
            let id = 37;
            let source = H160::repeat_byte(7);

            let message = Call::System(<frame_system::Call<Test>>::remark(vec![])).encode();

            System::set_block_number(1);
            Dispatch::dispatch(source, id, &message);

            assert_eq!(
                System::events(),
                vec![EventRecord {
                    phase: Phase::Initialization,
                    event: Event::Dispatch(crate::Event::<Test>::MessageDispatched(
                        id,
                        Err(DispatchError::BadOrigin)
                    )),
                    topics: vec![],
                }],
            );
        })
    }

    #[test]
    fn test_message_decode_failed() {
        new_test_ext().execute_with(|| {
            let id = 37;
            let source = H160::repeat_byte(7);

            let message: Vec<u8> = vec![1, 2, 3];

            System::set_block_number(1);
            Dispatch::dispatch(source, id, &message);

            assert_eq!(
                System::events(),
                vec![EventRecord {
                    phase: Phase::Initialization,
                    event: Event::Dispatch(crate::Event::<Test>::MessageDecodeFailed(id)),
                    topics: vec![],
                }],
            );
        })
    }

    #[test]
    fn test_message_rejected() {
        new_test_ext().execute_with(|| {
            let id = 37;
            let source = H160::repeat_byte(7);

            let message = Call::System(<frame_system::Call<Test>>::set_code(vec![])).encode();

            System::set_block_number(1);
            Dispatch::dispatch(source, id, &message);

            assert_eq!(
                System::events(),
                vec![EventRecord {
                    phase: Phase::Initialization,
                    event: Event::Dispatch(crate::Event::<Test>::MessageRejected(id)),
                    topics: vec![],
                }],
            );
        })
    }
}
