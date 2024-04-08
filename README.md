# Liquity ActivePool Substream

This substream tracks the Liquity ActivePool contract: 0xDf9Eb223bAFBE5c5271415C75aeCD68C21fE3D7F
Besides creating standard entities for every event, we provide a new entity activePool that provides basic information on the status of the contract aswell as the cumulative total ETH sent by the contract. Purpose is to provide the most important of the contract in one entity. For more special information we still provide the standard entities for all the events

## Quickstart

Make sure you have the latest versions of the following installed:

- [Rust](https://rustup.rs/)
- [Make](https://formulae.brew.sh/formula/make)
- [graph-cli](https://thegraph.com/docs/en/cookbook/quick-start/#2-install-the-graph-cli)
- [substreams-cli](https://substreams.streamingfast.io/getting-started/installing-the-cli)

### 1. Clone the project

Clone the project with git clone on your system

### 2. Compile the Project with `make build`

We now need to recompile our WASM binary with the new changes we made to the rust files.

### 3. Pack the spkg with `make package`

We need to bundle the protobuf definitions and the WASM binary into a single file. This is what we will deploy the subgraph.

### 4. Deploy the subgraph with `graph deploy`