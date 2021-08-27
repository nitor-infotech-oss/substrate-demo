# Substrate Node Template
In this repository, Here we build 3 pallets
* Calculator: This pallet is build to perform Basic arithmatic operations.
* Store: This pallet is build to store employee details.
* FindLarge: This pallet is build to find out the larger number between 3 or 2 numbers.

## Rust Setup
First, complete the basic [Rust setup instructions](https://www.rust-lang.org/tools/install).

### Run
Use Rust's native cargo command to build and launch the template node:
```sh
cargo run --release -- --dev --tmp
```
### Build
The cargo run command will perform an initial build. Use the following command to build the node without launching it:
```sh
cargo build --release
```

### Run
The provided cargo run command will launch a temporary node and its state will be discarded after you terminate the process. After the project has been built, there are other ways to launch the node.

This command will start the single-node development chain with persistent state:

```sh
./target/release/node-template --dev
```

## Connect with Polkadot-JS Apps Front-end
Once the node template is running locally, you can connect it with Polkadot-JS Apps front-end to interact with your chain. [Click here](https://polkadot.js.org/apps/#/explorer?rpc=ws://127.0.0.1:9944) connecting the Apps to your local node template.

## Template Structure
A Substrate project such as this consists of a number of components that are spread across a few directories.

### Node
A blockchain node is an application that allows users to participate in a blockchain network. Substrate-based blockchain nodes expose a number of capabilities:

Networking: Substrate nodes use the libp2p networking stack to allow the nodes in the network to communicate with one another.

Consensus: Blockchains must have a way to come to consensus on the state of the network. Substrate makes it possible to supply custom consensus engines and also ships with several consensus mechanisms that have been built on top of Web3 Foundation research.

RPC Server: A remote procedure call (RPC) server is used to interact with Substrate nodes.
There are several files in the node directory - take special note of the following:

chain_spec.rs: A chain specification is a source code file that defines a Substrate chain's initial (genesis) state. Chain specifications are useful for development and testing, and critical when architecting the launch of a production chain. Take note of the development_config and testnet_genesis functions, which are used to define the genesis state for the local development chain configuration. These functions identify some well-known accounts and use them to configure the blockchain's initial state.

service.rs: This file defines the node implementation. Take note of the libraries that this file imports and the names of the functions it invokes. In particular, there are references to consensus-related topics, such as the longest chain rule, the Aura block authoring mechanism and the GRANDPA finality gadget.
After the node has been built, refer to the embedded documentation to learn more about the capabilities and configuration parameters that it exposes:

```sh
./target/release/node-template --help
```

### Runtime
In Substrate, the terms "runtime" and "state transition function" are analogous - they refer to the core logic of the blockchain that is responsible for validating blocks and executing the state changes they define. The Substrate project in this repository uses the FRAME framework to construct a blockchain runtime. FRAME allows runtime developers to declare domain-specific logic in modules called "pallets". At the heart of FRAME is a helpful macro language that makes it easy to create pallets and flexibly compose them to create blockchains that can address a variety of needs.

Review the FRAME runtime implementation included in this template and note the following:

This file configures several pallets to include in the runtime. Each pallet configuration is defined by a code block that begins with impl $PALLET_NAME::Config for Runtime.
The pallets are composed into a single runtime by way of the construct_runtime! macro, which is part of the core FRAME Support library.

### Pallets
The runtime in this project is constructed using many FRAME pallets that ship with the core Substrate repository and a template pallet that is defined in the pallets directory.

A FRAME pallet is compromised of a number of blockchain primitives:

Storage: FRAME defines a rich set of powerful storage abstractions that makes it easy to use Substrate's efficient key-value database to manage the evolving state of a blockchain.

Dispatchables: FRAME pallets define special types of functions that can be invoked (dispatched) from outside of the runtime in order to update its state.

Events: Substrate uses events to notify users of important changes in the runtime.

Errors: When a dispatchable fails, it returns an error.

Config: The Config configuration interface is used to define the types and parameters upon which a FRAME pallet depends.
