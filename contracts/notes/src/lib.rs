#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// =======================
// STRUCT DATA
// =======================

// Data Travel (jadwal)
#[contracttype]
#[derive(Clone, Debug)]
pub struct Travel {
    id: u64,
    from: String,
    to: String,
    date: String,
    price: u64,
}

// Data Ticket (pesanan user)
#[contracttype]
#[derive(Clone, Debug)]
pub struct Ticket {
    id: u64,
    travel_id: u64,
    passenger_name: String,
}

// =======================
// STORAGE KEY
// =======================
const TRAVEL_DATA: Symbol = symbol_short!("TRAVEL");
const TICKET_DATA: Symbol = symbol_short!("TICKET");

// =======================
// CONTRACT
// =======================
#[contract]
pub struct TravelContract;

#[contractimpl]
impl TravelContract {

    // =======================
    // GET DATA
    // =======================

    pub fn get_travels(env: Env) -> Vec<Travel> {
        env.storage().instance().get(&TRAVEL_DATA).unwrap_or(Vec::new(&env))
    }

    pub fn get_tickets(env: Env) -> Vec<Ticket> {
        env.storage().instance().get(&TICKET_DATA).unwrap_or(Vec::new(&env))
    }

    // =======================
    // CREATE TRAVEL
    // =======================

    pub fn create_travel(env: Env, from: String, to: String, date: String, price: u64) -> String {
        let mut travels: Vec<Travel> = env.storage().instance().get(&TRAVEL_DATA).unwrap_or(Vec::new(&env));

        let travel = Travel {
            id: env.prng().gen::<u64>(),
            from,
            to,
            date,
            price,
        };

        travels.push_back(travel);
        env.storage().instance().set(&TRAVEL_DATA, &travels);

        String::from_str(&env, "Travel berhasil ditambahkan")
    }

    // =======================
    // BOOK TICKET
    // =======================

    pub fn book_ticket(env: Env, travel_id: u64, passenger_name: String) -> String {
        let mut tickets: Vec<Ticket> = env.storage().instance().get(&TICKET_DATA).unwrap_or(Vec::new(&env));

        let ticket = Ticket {
            id: env.prng().gen::<u64>(),
            travel_id,
            passenger_name,
        };

        tickets.push_back(ticket);
        env.storage().instance().set(&TICKET_DATA, &tickets);

        String::from_str(&env, "Tiket berhasil dipesan")
    }

    // =======================
    // CANCEL TICKET
    // =======================

    pub fn cancel_ticket(env: Env, id: u64) -> String {
        let mut tickets: Vec<Ticket> = env.storage().instance().get(&TICKET_DATA).unwrap_or(Vec::new(&env));

        for i in 0..tickets.len() {
            if tickets.get(i).unwrap().id == id {
                tickets.remove(i);

                env.storage().instance().set(&TICKET_DATA, &tickets);
                return String::from_str(&env, "Tiket berhasil dibatalkan");
            }
        }

        String::from_str(&env, "Tiket tidak ditemukan")
    }
}