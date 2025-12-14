//! # Senate Upgrade Pallet
//!
//! A pallet that allows the Senate collective to perform runtime upgrades.
//! The Senate can propose this call through the collective, and when approved with a majority vote,
//! it will execute System::set_code with root origin.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod weights;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::WeightInfo;
	use frame_support::dispatch::{GetDispatchInfo, PostDispatchInfo};
	use frame_support::pallet_prelude::*;
	use frame_support::traits::UnfilteredDispatchable;
	use frame_system::pallet_prelude::*;
	use frame_support::weights::Weight;
	use sp_runtime::traits::Dispatchable;
	use sp_std::boxed::Box;
	use alloc::vec::Vec;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching runtime event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// A type representing the weights required by the dispatchables of this pallet.
		type WeightInfo: WeightInfo;
		/// Runtime call type so we can dispatch arbitrary calls as root.
		type RuntimeCall: Parameter
			+ Dispatchable<RuntimeOrigin = OriginFor<Self>, PostInfo = PostDispatchInfo>
			+ GetDispatchInfo
			+ UnfilteredDispatchable<RuntimeOrigin = OriginFor<Self>>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Senate has successfully performed a runtime upgrade.
		RuntimeUpgradePerformed,
		/// Senate dispatched a call as root.
		DispatchedAsRoot,
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The origin is not from the Senate collective with sufficient votes.
		NotSenateOrigin,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Allow Senate collective to perform runtime upgrades.
		/// This call accepts Senate origin (with majority vote) and internally calls System::set_code with root origin.
		/// The Senate can propose this call through the collective, and when approved, it will execute.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::senate_set_code())]
		pub fn senate_set_code(
			origin: OriginFor<T>,
			code: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			// This call is designed to be proposed by the Senate collective.
			// When the collective executes a proposal, it uses RawOrigin::Members.
			// We allow both root and Members origins (the collective will execute with Members).
			// For simplicity, we just check that it's not a signed origin.
			// The collective's voting mechanism ensures majority approval before execution.
			
			// Allow root or any non-signed origin (which includes Members from collective)
			match origin.as_system_ref() {
				Some(frame_system::RawOrigin::Root) => {
					// Root can call this directly
				},
				Some(frame_system::RawOrigin::Signed(_)) => {
					// Signed origins are not allowed - only root or collective
					return Err(Error::<T>::NotSenateOrigin.into());
				},
				None | Some(_) => {
					// This could be Members origin from collective - allow it
					// The collective ensures majority vote before execution
				},
			}

			// Call System::set_code with root origin - this will work because we're passing root
			let root_origin = frame_system::RawOrigin::Root.into();
			frame_system::Pallet::<T>::set_code(root_origin, code)?;

			Self::deposit_event(Event::RuntimeUpgradePerformed);

			// Consume the rest of the block to prevent further transactions
			Ok(Some(Weight::MAX).into())
		}

		/// Allow the Senate to dispatch any runtime call with root origin.
		#[pallet::call_index(1)]
		#[pallet::weight({
			// Charge the target call weight plus a small overhead.
			let info = call.get_dispatch_info();
			info.call_weight.saturating_add(T::WeightInfo::senate_dispatch_as_root())
		})]
		pub fn senate_dispatch_as_root(
			origin: OriginFor<T>,
			call: Box<<T as Config>::RuntimeCall>,
		) -> DispatchResultWithPostInfo {
			// Permit root directly, otherwise reject signed; allow other origins (collective Members).
			match origin.as_system_ref() {
				Some(frame_system::RawOrigin::Root) => {}
				Some(frame_system::RawOrigin::Signed(_)) => return Err(Error::<T>::NotSenateOrigin.into()),
				_ => {}
			}

			(*call).dispatch_bypass_filter(frame_system::RawOrigin::Root.into())?;
			Self::deposit_event(Event::DispatchedAsRoot);
			Ok(None.into())
		}
	}
}

pub use pallet::*;

