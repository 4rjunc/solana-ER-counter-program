# Ephemeral Counter - Solana State Delegation Demo

This project demonstrates Magicblock's Ephemeral Rollups (ER) technology on Solana. The example implements a simple counter program that can operate on both the Solana base layer and an ephemeral rollup for improved performance.

## What are Ephemeral Rollups?

Magicblock's Ephemeral Rollups leverage Solana's account-based structure and parallel execution to optimize state management. By structuring state into clusters, users can lock accounts and temporarily shift execution to a dedicated auxiliary layer—the "ephemeral rollup."

### Key Concepts

- **State Delegation**: Users lock state accounts to an ephemeral rollup, enabling a separate runtime that operates independently.
- **Sequencer Control**: The sequencer temporarily modifies state in the rollup. If constraints are violated, the state is forcefully reverted and unlocked on the L1.
- **Parallel Read Access**: Even as accounts are delegated, transactions on the base layer can still read their state, ensuring compatibility.

## Benefits of Ephemeral Rollups

- ✅ **Gasless Transactions**: Enable scalability and mass adoption
- ✅ **Faster Block Times**: Enables real-time interactions for seamless UX
- ✅ **Integrated Scheduling**: Built-in automation to execute transactions on schedule
- ✅ **Program and States Synchronization**: No fragmentation between layers
- ✅ **Horizontal Scaling**: Launch multiple rollups on-demand for millions of transactions
- ✅ **Familiar Tooling**: Reuse existing programming languages, libraries, and testing tools

## Project Overview

This demo implements a counter application that can operate on both the Solana base layer and an ephemeral rollup. It showcases the state delegation mechanism and transaction flow between layers.

### Core Functionality

The program implements six main instructions:

1. `InitializeCounter`: Initialize and sets the counter to 0 (called on Base Layer)
2. `IncreaseCounter`: Increments the initialized counter by X amount (called on Base Layer or ER)
3. `Delegate`: Delegates counter from Base Layer to ER (called on Base Layer)
4. `CommitAndUndelegate`: Schedules sync of counter from ER to Base Layer, and undelegates counter on ER (called on ER)
5. `Commit`: Schedules sync of counter from ER to Base Layer (called on ER)
6. `Undelegate`: Undelegates counter on the Base Layer (called on Base Layer through validator CPI)

## Project Structure

```
ephemeral-counter/
├── src/
│   ├── lib.rs                # Program entry point
│   ├── instruction.rs        # Instruction definitions and deserialization
│   ├── processor.rs          # Instruction processing logic
│   ├── state.rs              # State management for counter
│   ├── error.rs              # Custom error definitions
│   └── utils.rs              # Helper functions
├── tests/
│   ├── test.ts               # Integration tests
│   ├── schema.ts             # Typescript schema definitions
│   └── initializeKeypair.ts  # Keypair utilities for testing
├── .env.example              # Example environment variables
├── .env                      # Environment configuration (gitignored)
├── package.json              # Node.js dependencies
├── Cargo.toml                # Rust dependencies
└── README.md                 # Project documentation
```

## Prerequisites

- Rust and Cargo
- Solana CLI tools
- Node.js and npm/yarn
- A Solana devnet wallet with SOL

## Setup Instructions

1. Clone the repository:
   ```bash
   git clone https://github.com/4rjunc/solana-ER-counter-program.git
   cd solana-ER-counter-program
   ```

2. Install dependencies:
   ```bash
   bun install
   ```

3. Create a `.env` file from the example:
   ```bash
   cp .env.example .env
   ```

4. Edit the `.env` file to add your keypair path or generate a new one:
   ```
    PRIVATE_KEY="[YOUR_KEYAPAIR]"
   ```

## Building and Deploying

1. Build the Solana program:
   ```bash
   cargo build-sbf
   ```

2. Deploy the program to Solana devnet:
   ```bash
   solana program deploy ./target/deploy/ephemeral_counter.so
   ```

## Running Tests

Run the integration tests to demonstrate the functionality:
```bash
bun test
```

## Understanding the Flow

1. **Base Layer Initialization**: The counter is initialized on the Solana base layer with a value of 0.
2. **Base Layer Operation**: The counter can be incremented directly on the Solana base layer.
3. **Delegation to Rollup**: The counter's state is delegated to the ephemeral rollup.
4. **Rollup Operations**: Multiple increment operations can be performed faster on the rollup.
5. **Commit to Base Layer**: The final state is committed back to the Solana base layer.
6. **Undelegation**: The counter's state is undelegated, making it fully operational on the base layer again.

## Account Structure

The program uses several accounts for operation:

1. **Counter Account (PDA)**: Stores the counter value, derived from user public key
2. **User Account**: The owner/initializer of the counter
3. **Delegation-related PDAs**: Various accounts that help manage the delegation process
4. **Magic Program and Context**: Accounts related to the Magicblock ephemeral rollup system

## Security Considerations

- State delegation creates a temporary trust assumption in the rollup sequencer
- The rollup uses force-reverting mechanisms to ensure state integrity
- All operations eventually sync back to the base layer for final settlement

## Acknowledgements

This project uses [Magicblock](https://docs.magicblock.gg/pages/get-started/introduction/ephemeral-rollup) Ephemeral Rollups SDK to demonstrate state delegation on Solana.

## License

[MIT License](LICENSE)
