# Penumbra Token Info

A robust service that tracks and provides real-time information about Penumbra's token supply metrics.

## Overview

Penumbra Token Info is a lightweight service that:
- Polls a PostgreSQL database for current token supply data
- Tracks static balances for foundation and investor accounts
- Provides HTTP endpoints to query circulating and total supply

## Installation

### Prerequisites
- Rust toolchain (1.80+)
- PostgreSQL database with Penumbra chain data

### Building from Source

```bash
git clone https://github.com/penumbra-zone/penumbra-token-info.git
cd penumbra-token-info
cargo build --release
```

The compiled binary will be available at `target/release/penumbrainfo`.

## Usage

Run the service by providing a PostgreSQL database URL:

```bash
./target/release/penumbrainfo postgres://user:password@localhost/database
```

### Configuration

The service uses the following default configuration:

- HTTP server binds to `0.0.0.0:5000`
- gRPC endpoint configured to `http://localhost:50051`
- Database polling interval: 10 seconds
- Failed query retry count: 3

## API Endpoints

### GET /v1/circulating-supply

Returns the current circulating supply (total supply minus locked tokens).

**Response Format:**
```json
{
  "circulating_supply": "<circulating-supply-in-tokens>"
}
```

### GET /v1/total-supply

Returns the total token supply, including locked tokens.

**Response Format:**
```json
{
  "total_supply": "<total-supply-in-tokens>"
}
```

## Implementation Details

### Static Balances

The service tracks the following static balances that are subtracted from the total supply:
- Penumbra Labs: 100,000,000 tokens
- Radiant: 100,000,000 tokens
- Numographica: 100,000,000 tokens
- Investor locked supply: Computed based on genesis values (TOTAL_LOCKED_GENESIS_SUPPLY + COMMUNITY_POOL_GENESIS_SUPPLY)

### Data Flow

1. The service polls the PostgreSQL database every 10 seconds
2. The SQL query aggregates data from:
   - Staked validator tokens (`supply_validators` and `supply_total_staked` tables)
   - Unstaked tokens (`supply_total_unstaked` table)
   - Auction tokens 
   - DEX tokens
3. API endpoints calculate total and circulating supply based on the latest data

### Database Schema Dependencies

The service relies on the following database tables:
- `supply_validators`
- `supply_total_staked`
- `supply_total_unstaked`

## Technical Notes

The service is built using:
- **Axum**: Web framework for the HTTP API
- **sqlx**: PostgreSQL database connectivity
- **Tokio**: Async runtime and concurrency
- **tracing**: Logging functionality
- **serde/serde_json**: Serialization for API responses
- **Penumbra Proto**: Penumbra protocol definitions
- **chrono**: Date and time handling for investor token unlocks

## License

This project is part of the Penumbra ecosystem.