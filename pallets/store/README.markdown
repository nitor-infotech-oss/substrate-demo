### Store Pallet

In this pallet we simply store the basic employee details like emp_id, emp_name & designation.

### Traits

This pallet depends on on the [FRAME EnsureOrigin System trait]
```
frame_support::traits::EnsureOrigin;
```


## How to use in your runtime

### Runtime `Cargo.toml`

To add this pallet to your runtime, simply include the following to your runtime's `Cargo.toml` file:

```TOML
[dependencies.pallet-store]
default-features = false
path = '../pallets/store'
version = '3.0.0'
```

and update your runtime's `std` feature to include this pallet:

```TOML
std = [
    # --snip--
   'pallet-store/std',
]
```

### Runtime `lib.rs`

You should implement it's trait like so:

```rust
impl pallet_store::Config for Runtime {
	type Event = Event;
}
```

and include it in your `construct_runtime!` macro:

```rust
StoreDetails: pallet_store::{Pallet, Call, Storage, Event<T>},
```
### How to store employee details

* After executing `node-template --dev`, Use this link to open the Polkadot JS Apps UI `https://polkadot.js.org/apps/#/explorer?rpc=ws://127.0.0.1:9944`
  
* Open Extrinsics and select the storeDetails pallet then select storeDetails function and provide id, name & designation of an employee.
 ![img6.png](img6.png)

* Click on `Submit Transaction` and employee details will be stored.
 ![img7.png](img7.png)

* To check the values are getting stored in a variable or not. Open Chain state and select the storeDetails pallet after that select id or name or designation then click on `+` sign.
 ![img8.png](img8.png)


