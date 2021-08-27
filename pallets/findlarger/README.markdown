### FindLarger Pallet

In this pallet we find out large number between 3 numbers and 2 numbers. For that we create two function; As per our requirement we can select either findLargerBetweenThree()
or findLargerBetweenTwo().
### Traits

This pallet depends on on the [FRAME EnsureOrigin System trait]
```
frame_support::traits::EnsureOrigin;
```
## How to use in your runtime

### Runtime `Cargo.toml`

To add this pallet to your runtime, simply include the following to your runtime's `Cargo.toml` file:

```TOML
[dependencies.pallet-findlarger]
default-features = false
path = '../pallets/findlarger'
version = '3.0.0'
```

and update your runtime's `std` feature to include this pallet:

```TOML
std = [
    # --snip--
    'pallet-findlarger/std',
]
```

### Runtime `lib.rs`

You should implement it's trait like so:

```rust
impl pallet_findlarger::Config for Runtime {
	type Event = Event;
}

```

and include it in your `construct_runtime!` macro:

```rust
FindLarger: pallet_findlarger::{Pallet, Call, Storage, Event<T>},
```
### How to check larger between 3 numbers

* After executing `node-template --dev`, Use this link to open the Polkadot JS Apps UI `https://polkadot.js.org/apps/#/explorer?rpc=ws://127.0.0.1:9944`
  
* Open Extrinsics and select the findLarger pallet then select findLargerBetweenThree function and provide 3 input values.
 ![img3.png](img3.png)

* Click on `Submit Transaction` .
 ![img4.png](img4.png)

* To verify the result open Chain state and select the findLarger pallet after that select finalResult then click on `+` sign.
 ![img5.png](img5.png)

Like the same way you can try other function !!!.


