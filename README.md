# Solana Hello World

A simple "Hello World" smart contract deployed on the Solana blockchain (devnet).

## Overview

This project demonstrates the basic setup, development, and deployment of a Rust-based smart contract on the Solana blockchain network. The smart contract logs "Hello, world!" along with program and account information when invoked.

## Technical Details

- **Solana CLI Version**: 2.1.20
- **Wallet Address**: 3FBr3ZiiZDMdgDobtgVkHmmVCpbkJTbpseDNxJNsxLF7
- **Program ID**: CzT1hzaHyi65MSxJs4ynvWKg3g8tXjGz6tPDBK4NvtP6

## Demo Screenshots
![image](https://github.com/user-attachments/assets/d6fe67d3-7657-4bfd-ae12-eac4da49f349)
![image](https://github.com/user-attachments/assets/1b5c7ca8-975e-4368-bef7-b816d40144d4)
![image](https://github.com/user-attachments/assets/fbfe93dc-7930-4746-a06b-59ea2eb2bae8)
![image](https://github.com/user-attachments/assets/c8831365-362d-40aa-bfc9-2004041bb501)
![image](https://github.com/user-attachments/assets/fd68cb9a-cb71-4734-800a-710376f74172)
![image](https://github.com/user-attachments/assets/80bb25b7-f851-43f1-84e2-d41f73be86c4)
![image](https://github.com/user-attachments/assets/652e694f-64cb-40fa-a850-b60bb159eaea)
![image](https://github.com/user-attachments/assets/e0b4aca1-1ac4-4afe-a420-816a0026eb05)
![image](https://github.com/user-attachments/assets/0e81499d-2911-4d05-a754-bcf04183c5bd)



## Prerequisites

- Rust and Cargo
- Solana CLI tools
- Basic knowledge of blockchain concepts

## Installation

1. Install Solana CLI tools:
```bash
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
```

2. Create a new Solana wallet:
```bash
solana-keygen new
```

3. Configure for devnet:
```bash
solana config set --url https://api.devnet.solana.com
```

4. Get SOL from devnet faucet:
```bash
solana airdrop 1
```

## Usage

1. Clone the repository:
```bash
git clone https://github.com/Isaksend/solana-assignment.git
cd solana-assignment
```

2. Build the program:
```bash
cargo build-sbf
```

3. Deploy to devnet:
```bash
solana program deploy target/deploy/solana_hello_world.so
```

4. Verify deployment:
```bash
solana program show <PROGRAM_ID>
```

## Project Structure

- `src/lib.rs` - Contains the smart contract code
- `Cargo.toml` - Rust package configuration and dependencies

## Examples

### Basic Hello World Program
```rust
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

// Declare the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Log a message to the blockchain
    msg!("Hello, world!");
    
    // Print program and account information
    msg!("Program ID: {}", program_id);
    msg!("Number of accounts: {}", accounts.len());
    
    // Process the instruction data if any
    if !instruction_data.is_empty() {
        msg!("Instruction data: {:?}", instruction_data);
    }
    
    Ok(())
}
```

## License

MIT License

Copyright (c) 2025 Islambek

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
