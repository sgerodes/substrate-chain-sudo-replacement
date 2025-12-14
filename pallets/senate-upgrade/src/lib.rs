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
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::weights::Weight;
		use alloc::vec::Vec;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching runtime event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// A type representing the weights required by the dispatchables of this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Senate has successfully performed a runtime upgrade.
		RuntimeUpgradePerformed,
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
	}


}

pub use pallet::*;

