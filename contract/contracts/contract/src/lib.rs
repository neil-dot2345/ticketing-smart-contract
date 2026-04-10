#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, Map, String, Symbol,
};

#[contract]
pub struct TicketContract;

#[derive(Clone)]
#[contracttype]
pub struct Ticket {
    pub event_name: String,
    pub owner: Address,
}

const TICKETS: Symbol = symbol_short!("TICKETS");

#[contractimpl]
impl TicketContract {

    // Initialize storage safely
    pub fn init(env: Env) {
        if env.storage().instance().has(&TICKETS) {
            panic!("Already initialized");
        }

        let map: Map<u32, Ticket> = Map::new(&env);
        env.storage().instance().set(&TICKETS, &map);
    }

    // Create a new ticket
    pub fn create_ticket(env: Env, ticket_id: u32, event_name: String, owner: Address) {
        // Require the owner to authorize ticket creation
        owner.require_auth();

        let mut tickets: Map<u32, Ticket> = match env.storage().instance().get(&TICKETS) {
            Some(map) => map,
            None => panic!("Contract not initialized"),
        };

        if tickets.contains_key(ticket_id) {
            panic!("Ticket ID already exists");
        }

        let ticket = Ticket {
            event_name,
            owner,
        };

        tickets.set(ticket_id, ticket);
        env.storage().instance().set(&TICKETS, &tickets);
    }

    // Transfer ticket ownership (SECURE)
    pub fn transfer_ticket(env: Env, ticket_id: u32, new_owner: Address) {
        let mut tickets: Map<u32, Ticket> = match env.storage().instance().get(&TICKETS) {
            Some(map) => map,
            None => panic!("Contract not initialized"),
        };

        let mut ticket = match tickets.get(ticket_id) {
            Some(t) => t,
            None => panic!("Ticket does not exist"),
        };

        // 🔐 Only current owner can transfer
        ticket.owner.require_auth();

        ticket.owner = new_owner;

        tickets.set(ticket_id, ticket);
        env.storage().instance().set(&TICKETS, &tickets);
    }

    // Get ticket details
    pub fn get_ticket(env: Env, ticket_id: u32) -> Ticket {
        let tickets: Map<u32, Ticket> = match env.storage().instance().get(&TICKETS) {
            Some(map) => map,
            None => panic!("Contract not initialized"),
        };

        match tickets.get(ticket_id) {
            Some(ticket) => ticket,
            None => panic!("Ticket not found"),
        }
    }
}