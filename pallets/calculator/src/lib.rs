#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>



pub use pallet::*;


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	pub type FinalResult<T> = StorageValue<_, u64, ValueQuery>;
	// #[pallet::getter(fn finalresult)]
	// pub(super) type FinalResult<T> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn something)]

	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
        Addition(T::AccountId, u64, u64, u64),
        Substraction(T::AccountId, u64, u64, u64),
        Multiplication(T::AccountId, u64, u64, u64),
        Division(T::AccountId, u64, u64, u64),
		Modulo(T::AccountId, u64, u64, u64),
		Power(T::AccountId, u64, u32, u64),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with num1 weight and must return num1 DispatchResult.
	#[pallet::call]
	impl<T:Config> Pallet<T> {
		/// An example dispatchable that takes num1 singles value as num1 parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by num1 signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return num1 successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw num1 custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read num1 value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}

	   // This function is for to add two numbers
	   #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]	
       pub fn add(origin: OriginFor<T>, num1: u64, num2: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            let result = num1.checked_add(num2).ok_or("Numbers are too large to store")?;
            <FinalResult<T>>::put(result);
            Self::deposit_event(Event::Addition(sender, num1, num2, result));
            Ok(())
        }

		 // This function is for to substract two numbers
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]	
		pub fn sub(origin: OriginFor<T>, num1: u64, num2: u64) -> DispatchResult {
			 let sender = ensure_signed(origin)?;
			 let result = num1.checked_sub(num2).ok_or("Numbers are too large to store")?;
			 <FinalResult<T>>::put(result);
			 Self::deposit_event(Event::Substraction(sender, num1, num2, result));
			 Ok(())
		 }

		  // This function is for to multiply two numbers
		 #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]	
		 pub fn mul(origin: OriginFor<T>, num1: u64, num2: u64) -> DispatchResult {
			  let sender = ensure_signed(origin)?;
			  let result = num1.checked_mul(num2).ok_or("Numbers are too large to store")?;
			  <FinalResult<T>>::put(result);
			  Self::deposit_event(Event::Multiplication(sender, num1, num2, result));
			  Ok(())
		  }

		   // This function is for to divide two numbers
		  #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]	
		  pub fn div(origin: OriginFor<T>, num1: u64, num2: u64) -> DispatchResult {
			   let sender = ensure_signed(origin)?;
			   let result = num1.checked_div(num2).ok_or("Numbers are too large to store")?;
			   <FinalResult<T>>::put(result);
			   Self::deposit_event(Event::Division(sender, num1, num2, result));
			   Ok(())
		   }

		    // This function is for to find out remainder 
		   #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]	
		  pub fn modulo(origin: OriginFor<T>, num1: u64, num2: u64) -> DispatchResult {
			   let sender = ensure_signed(origin)?;
			   let result = num1.checked_rem(num2).ok_or("Numbers are too large to store")?;
			   <FinalResult<T>>::put(result);
			   Self::deposit_event(Event::Modulo(sender, num1, num2, result));
			   Ok(())
		   }

		      // This function is for to find out power of a number 
		   #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]	
		  pub fn pow(origin: OriginFor<T>, num1: u64, num2: u32) -> DispatchResult {
			   let sender = ensure_signed(origin)?;
			   let result = num1.checked_pow(num2).ok_or("Numbers are too large to store in num1 u64")?;
			   <FinalResult<T>>::put(result);
			   Self::deposit_event(Event::Power(sender, num1, num2, result));
			   Ok(())
		   }

		  


	}
}







