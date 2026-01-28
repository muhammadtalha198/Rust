# you can get the solana configration by simply typing the 
- solana config get



# Switch to Localhost (for local testing)
- solana config set --url localhost
# Short flag version:
- solana config set -ul
​
# Switch to Devnet (for public testing)
- solana config set --url devnet
# Short flag version:
- solana config set -ud




# Generating a New Wallet
-> solana-keygen new

# View your public key (Wallet Address)
- solana address
​
# Check current SOL balance
- solana balance


The Airdrop Workflow
# Ensure you are on Devnet:
- solana config set -ud


# Request the Airdrop:

- Request 2 SOL
- solana airdrop 2


How to Start the Validator
# Set config to Localhost:
- solana config set -ul

# Start the process:
- solana-test-validator




Wrote new keypair to /Users/mrmacbook/.config/solana/id.json
==========================================================================
pubkey: AL2vX4DVzvrBNqAwtgJ5o8SHAuMpAJoNGDYLU7ryjFPD
==========================================================================
Save this seed phrase and your BIP39 passphrase to recover your new keypair:
auto public force enough minor cross wink jelly royal pass engage exercise