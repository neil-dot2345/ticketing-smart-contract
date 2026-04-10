# 🎟️ Soroban Ticketing Smart Contract

## 📌 Project Description

This project is a decentralized ticketing system built using Soroban on the Stellar network. It enables secure creation, ownership, and transfer of event tickets directly on-chain.

The contract ensures that ticket ownership is verifiable, tamper-proof, and controlled by cryptographic authentication.

<img width="1920" height="1040" alt="ticketing smart contract" src="https://github.com/user-attachments/assets/fd09a894-5093-420d-9cb7-ebd943c40d72" />

---

## 🚀 What It Does

This smart contract allows users to:

* Create tickets linked to an event
* Store ticket ownership on-chain
* Transfer tickets securely between users
* Retrieve ticket details anytime

Each ticket is uniquely identified by a `ticket_id` and contains:

* Event name
* Owner address

---

## ✨ Features

* 🎫 **Ticket Creation**

  * Users can mint tickets tied to an event
  * Prevents duplicate ticket IDs

* 🔐 **Secure Ownership**

  * Ticket creation and transfers require authorization (`require_auth`)

* 🔄 **Ownership Transfer**

  * Only the current owner can transfer a ticket

* 🔍 **On-chain Verification**

  * Anyone can fetch ticket details using the ticket ID

* 🧱 **Decentralized Storage**

  * All ticket data is stored on the Stellar blockchain

---

## 🛠️ Tech Stack

* Rust (Soroban SDK)
* Soroban Smart Contracts
* Stellar Network

---

## 📦 Contract Functions

| Function          | Description                          |
| ----------------- | ------------------------------------ |
| `init`            | Initializes contract storage         |
| `create_ticket`   | Creates a new ticket (auth required) |
| `transfer_ticket` | Transfers ownership (owner only)     |
| `get_ticket`      | Fetches ticket details               |

---

## 🔗 Deployed Smart Contract

**Contract ID:**
`CDZTFA7ZIG5VT63JFEIUBATI2XXXNBZ2KYF2FJC4UEZTAIWQOOKV5AMU`

You can interact with this contract using the Stellar CLI or integrate it into a frontend dApp.

---

## 🧪 Example Workflow

1. Initialize the contract
2. Create a ticket with a unique ID
3. Assign ownership to a user
4. Transfer the ticket to another user
5. Verify ownership anytime on-chain

---

## 📌 Future Improvements

* 🎟️ NFT-style ticket metadata (seat, time, venue)
* 💰 Paid ticketing (XLM or token integration)
* 👤 Event organizer roles & permissions
* ❌ Ticket cancellation & validation logic
* 📱 Frontend dApp integration

---

## ⚠️ Notes

* Contract must be initialized before use
* Ticket IDs must be unique
* Unauthorized transfers are prevented via authentication

---

## 📜 License

MIT License
