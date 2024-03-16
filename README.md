# ArcaneVote
## Description
Arcane-Vote is a secure and decentralized voting system built on the Radix ledger. It allows users to create and participate in proposals, with all data securely recorded on the ledger, preventing any manipulation of data. 
The key feature of Arcane-Vote is its utilization of tokens committed to the proposal voting process. Participants must commit tokens to a proposal, and in return, they receive rewards based on a calculated component (total tokens committed by the voter).

## Voter
Voters have the freedom to select and commit to any proposal of their choosing, Once a voter commits tokens to a proposal, they cannot cancel their vote, add more tokens to the proposal, engage in double voting, or withdraw their tokens until the end of the proposal's epoch.

## Proposer
Proposal Creation: Proposers can create proposals based on their own interests. Proposers have the authority to set the duration of their proposals, typically based on predetermined quarters that the component has provided.

## How it works

Proposers and voters have to sign-up, mint a soulbound NFT badge.
```js
CALL_METHOD
Address("{arcane_main_component_address}")
"sign_up"
;

CALL_METHOD
Address("{account_address}")
"try_deposit_batch_or_refund"
Expression("ENTIRE_WORKTOP")
Enum<0u8>()
;
```

After signing up, users can proceed to create a vote.
```js
CALL_METHOD
    Address("{account_address}")
    "create_proof_of_non_fungibles"
    Address("{nft_badge_address}")
    Array<NonFungibleLocalId>(
        NonFungibleLocalId("{nft_badge_id}")
    )
;

POP_FROM_AUTH_ZONE
    Proof("nft_proof")
;

CALL_METHOD
    Address("{arcane_main_component_address}")
    "create_vote"
    Proof("nft_proof")
    Enum<1u8>("https://arcanedev.site/pict.png")
    "full dezentralized the arcane labyrinth"
    1u64
    Array<String>("abstain", "for", "against")
;
```
The create_vote method takes five arguments: `Proof`, `URL` (to picture), `Description`, `Quarter` (an integer limited to 1-3), and `Keys` (list of keys string for proposal).

After creating a vote, users can proceed to vote for the component address emitted after the previous RTM has been signed by the wallet.
```js
CALL_METHOD
    Address("{account_address}")
    "create_proof_of_non_fungibles"
    Address("{nft_badge_address}")
    Array<NonFungibleLocalId>(
        NonFungibleLocalId("{nft_badge_id}")
    )
;

POP_FROM_AUTH_ZONE
    Proof("nft_proof")
;

CALL_METHOD
  Address("{account_address}")
  "withdraw"
  Address("{token_commited_address}")
  Decimal("{amount}") 
;

TAKE_FROM_WORKTOP
  Address("{token_commited_address}")
  Decimal("{amount}")
  Bucket("arc_bucket")
;

CALL_METHOD
    Address("{vote_component_address}")
    "vote"
    Proof("nft_proof")
    "for"
    Bucket("arc_bucket")
;
```
The vote method takes three arguments: `Proof`, `key`, and the `Bucket`.

After the epoch duration of the component has ended, users can withdraw their tokens.
```js
CALL_METHOD
    Address("{account_address}")
    "create_proof_of_non_fungibles"
    Address("{nft_badge_address}")
    Array<NonFungibleLocalId>(
        NonFungibleLocalId("{nft_badge_id}")
    )
;

POP_FROM_AUTH_ZONE
    Proof("nft_proof")
;

CALL_METHOD
    Address("{vote_component_address}")
    "withdraw"
    Proof("nft_proof")
;

CALL_METHOD
    Address("{account_address}")
    "try_deposit_batch_or_refund"
    Expression("ENTIRE_WORKTOP")
    Enum<0u8>()
;
```
The withdraw method only takes a `Proof` argument since the data is already recorded on the ledger.
