### Calculator Pallet

In this pallet we perform some basic arithmatic operations like:
* Addition
* Subtraction
* Division
* Multiplication
* Modulo
* Power 

### Traits

This pallet depends on on the [FRAME EnsureOrigin System trait]
```
frame_support::traits::EnsureOrigin;
```


## How to use in your runtime

### Runtime `Cargo.toml`

To add this pallet to your runtime, simply include the following to your runtime's `Cargo.toml` file:

```TOML
[dependencies.pallet-calculator]
default-features = false
path = '../pallets/calculator'
version = '3.0.0'
```

and update your runtime's `std` feature to include this pallet:

```TOML
std = [
    # --snip--
    'pallet-calculator/std',
]
```

### Runtime `lib.rs`

You should implement it's trait like so:

```rust
impl pallet_calculator::Config for Runtime {
	type Event = Event;
}
```

and include it in your `construct_runtime!` macro:

```rust
Calculator: pallet_calculator::{Pallet, Call, Storage, Event<T>},
```
### How to add two numbers and verify the result

* After executing `node-template --dev`, Use this link to open the Polkadot JS Apps UI `https://polkadot.js.org/apps/#/explorer?rpc=ws://127.0.0.1:9944`
  
* Open Extrinsics and select the calculator pallet then select add function and provide 2 input values.
 ![img.png](img.png)

* Click on `Submit Transaction` and addition will be performed.
 ![img1.png](img1.png)

* To verify the result open Chain state and select the calculator pallet after that select finalResult then click on `+` sign.
 ![img2.png](img2.png)

Like the same way you can try other functions !!!.

