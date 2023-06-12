//! define pallet storage for using at the consensus level

#[frame_support::pallet]
mod pallet {

    /// ## `StorageValue` that can be get/set/take/kill
    /// ### Usage:
    /// 1. Any type - primitive, derived or custom. Please ensure the trait bounds, otherwise you may get error.
    /// Define a storage value with the name `Value` of type `bool`
    #[pallet::storage]
    #[pallet::getter(fn value)] // here, the getter function name for this storage value is `value()`
    type Value<T> = StorageValue<_, bool>
    /// 
    /// `<Value<T>>::put(true);`: set the value to `true`
    /// `<Value<T>>::get();`: get the value
    /// `<Value<T>>::take();`: take the value and set it to `None` i.e. clear the storage
    /// `<Value<T>>::kill();`: kill the storage i.e. remove the storage from the storage trie
    /// For more such methods of `StorageValue`, refer this: https://crates.parity.io/frame_support/pallet_prelude/struct.StorageValue.html
    /// ------------------------------------
    /// ### 2. A list of item i.e. fixed size array
    /// The one config trait which includes the max size of array 
	#[pallet::config]
	pub trait Config: frame_system::Config + Get<u32> {
		/// A constant for the max size of owners
		#[pallet::constant]
		type MaxOwners: Get<u32>;
		// ==snip==
	}

	#[pallet::storage]
	#[pallet::getter(fn array)]
	// NOTE: Had to add trait bound for T with `Config`
	/// List of owners
    /// `MaxOwners` needs to be defined in the `Config` trait as `type MaxOwners: Get<u32>;`
	pub type SomeArray<T: Config> = StorageValue<_, BoundedVec<T::AccountId, T::MaxOwners>>;
    /// 
    /// For functions, refer this: https://docs.rs/sp-runtime/latest/sp_runtime/bounded_vec/struct.BoundedVec.html
    /// ------------------------------------
    /// ### 3. A vector of item i.e. dynamic size array. Although NOT recommended
	#[pallet::storage]
	#[pallet::getter(fn land_owners)]
	// NOTE: Had to add trait bound for T with `Config`
	/// List of owners
	pub type LandOwners<T: Config> = StorageValue<_, Vec<T::AccountId>>;
    /// For `Vec` functions, refer this: https://crates.parity.io/frame_support/dispatch/struct.Vec.html
    /// 
    /// ====================================
    /// ## `StorageMap` that can be get/set/take/kill
    /// ### Usage:
    /// 1. A map of item i.e. key-value pair
    /// The key-value pair is of type `AccountId` and `bool`
	#[pallet::storage]
	#[pallet::getter(fn has_property)]
	// NOTE: Had to add trait bound for T with `Config`
	/// List of owners
	pub type HasProperty<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, bool>;
    /// 
    /// For methods of `StorageMap`, refer this: https://crates.parity.io/frame_support/pallet_prelude/struct.StorageMap.html
    /// ------------------------------------
    /// 2. A map of key -> fixed sized array
    /// TODO: add your notes here
    /// 3. A map of key -> dynamic sized array
    /// TODO: add your notes here
    /// 4. A map of key -> tuple
    /// TODO: add your notes here
}