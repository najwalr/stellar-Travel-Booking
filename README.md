# Travel Booking DApp

**Travel Booking DApp** - Blockchain-Based Travel and Ticket Reservation System

## Project Description

Travel Booking DApp is a decentralized smart contract application built on the Stellar blockchain using the Soroban SDK and Rust programming language.

This application provides a transparent, secure, and trustless travel ticket booking system without relying on third-party services. All data, including travel schedules and user tickets, is stored directly on the blockchain, making it immutable and verifiable.

The system allows users to:
- Create travel schedules
- View available travel data
- Book travel tickets
- Cancel booked tickets

All operations are executed through smart contracts without depending on centralized servers.

---

## Project Vision

The vision of this project is to build a modern digital transportation system that is:

- **Decentralized**: Eliminates dependency on traditional booking platforms  
- **Transparent**: All data can be verified on the blockchain  
- **Secure**: Data cannot be manipulated by unauthorized parties  
- **Efficient**: Faster booking process without intermediaries  
- **Trustless**: The system operates based on code, not trust in institutions  

---

## Key Features

### 1. Travel Data Management
- Create new travel schedules (origin, destination, date, price)
- Automatically generated travel IDs
- Data stored permanently on the blockchain

### 2. Ticket Booking
- Book tickets using travel ID
- Store passenger name
- Automatically generated ticket IDs
- Secure and transparent data storage

### 3. Ticket Cancellation
- Cancel tickets using ticket ID
- Data is instantly updated in contract storage
- Validates ticket existence before deletion

### 4. Data Retrieval
- `get_travels()` to fetch all travel data
- `get_tickets()` to fetch all ticket data
- Structured data for easy frontend integration

### 5. Blockchain Storage
- Data stored in smart contract instance storage
- No traditional database required
- Ensures consistency and security

---

## Data Structure

### Travel
- `id` → Unique travel ID  
- `from` → Origin location  
- `to` → Destination location  
- `date` → Travel date  
- `price` → Ticket price  

### Ticket
- `id` → Unique ticket ID  
- `travel_id` → Selected travel ID  
- `passenger_name` → Passenger name  

---

## Smart Contract Functions

### create_travel()
Creates a new travel schedule  
**Parameters:** from, to, date, price  
**Output:** "Travel successfully added"

### get_travels()
Retrieves all travel data

### book_ticket()
Books a ticket  
**Parameters:** travel_id, passenger_name  
**Output:** "Ticket successfully booked"

### get_tickets()
Retrieves all ticket data

### cancel_ticket()
Cancels a ticket  
**Parameters:** id  
**Output:** "Ticket successfully canceled" / "Ticket not found"

---

## Technical Stack

- Blockchain: Stellar  
- Smart Contract: Soroban SDK  
- Programming Language: Rust  
- Storage: Contract Instance Storage  

---

## How It Works

1. Admin creates travel data using `create_travel()`  
2. Users view available travels using `get_travels()`  
3. Users book tickets using `book_ticket()`  
4. Data is stored securely on the blockchain  
5. Users can cancel tickets using `cancel_ticket()`  

---

## Future Scope

### Short-Term
- Travel validation during booking
- Filtering by date or location
- Simple web/mobile UI

### Medium-Term
- Wallet integration
- Crypto payment system
- Transaction history tracking

### Long-Term
- NFT-based ticketing
- Real-world transportation integration
- Cross-chain booking system
- Travel provider reputation system

---

## Getting Started

1. Deploy the smart contract to the Soroban network  
2. Use the following functions:
   - `create_travel()`
   - `get_travels()`
   - `book_ticket()`
   - `get_tickets()`
   - `cancel_ticket()`

---

## Conclusion

Travel Booking DApp demonstrates how blockchain technology can be used to build a secure, transparent, and efficient ticket booking system without relying on centralized services.

---

**Travel Booking DApp** - Decentralizing Travel Reservations 🚀  

**Smart Contract ID:**  
`CBHIMY4HJOA5MPAKR6ZFWIXPTNLSTB2A2JUZXO2BQQK5AEZE2LSYKA4V`