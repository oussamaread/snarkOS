use crate::ledger::*;
use snarkos_errors::{dpc::DPCError, objects::TransactionError};
use snarkos_utilities::bytes::{FromBytes, ToBytes};

use rand::Rng;
use std::hash::Hash;

pub mod address;
pub mod base_dpc;

pub trait AddressKeyPair {
    type AddressPublicKey: Default;
    type AddressSecretKey: Default;
}

pub trait Predicate: Clone {
    type PublicInput;
    type PrivateWitness;

    /// Returns the evaluation of the predicate on given input and witness.
    fn evaluate(&self, primary: &Self::PublicInput, witness: &Self::PrivateWitness) -> bool;

    fn into_compact_repr(&self) -> Vec<u8>;
}

pub trait Record: Default {
    type AddressPublicKey;
    type Commitment;
    type CommitmentRandomness;
    type Payload;
    type Predicate;
    type SerialNumberNonce;
    type SerialNumber: Clone + Eq + Hash + FromBytes + ToBytes;

    /// Returns the address public key.
    fn address_public_key(&self) -> &Self::AddressPublicKey;

    /// Returns whether or not the record is dummy.
    fn is_dummy(&self) -> bool;

    /// Returns the record payload.
    fn payload(&self) -> &Self::Payload;

    /// Returns the birth predicate of this record.
    fn birth_predicate_repr(&self) -> &[u8];

    /// Returns the death predicate of this record.
    fn death_predicate_repr(&self) -> &[u8];

    /// Returns the randomness used for the serial number.
    fn serial_number_nonce(&self) -> &Self::SerialNumberNonce;

    /// Returns the commitment of this record.
    fn commitment(&self) -> Self::Commitment;

    /// Returns the randomness used for the commitment.
    fn commitment_randomness(&self) -> Self::CommitmentRandomness;
}

pub trait Transaction: Clone + Eq + FromBytes + ToBytes {
    type SerialNumber: Clone + Eq + Hash + FromBytes + ToBytes;
    type Commitment: Clone + Eq + Hash + FromBytes + ToBytes;
    type Memorandum: Clone + Eq + Hash + FromBytes + ToBytes;
    type Stuff;

    /// Returns the old serial numbers.
    fn old_serial_numbers(&self) -> &[Self::SerialNumber];

    /// Returns the new commitments.
    fn new_commitments(&self) -> &[Self::Commitment];

    /// Returns the memorandum.
    fn memorandum(&self) -> &Self::Memorandum;

    /// Returns the stuff field.
    fn stuff(&self) -> &Self::Stuff;

    /// Returns the transaction identifier.
    fn transaction_id(&self) -> Result<[u8; 32], TransactionError>;
}

pub trait DPCScheme<L: Ledger> {
    type AddressKeyPair: AddressKeyPair;
    type Auxiliary;
    type Metadata: ?Sized;
    type Payload;
    type Parameters;
    type Predicate: Predicate<PrivateWitness = Self::PrivatePredInput>;
    type PrivatePredInput;
    type Record: Record<
        AddressPublicKey = <Self::AddressKeyPair as AddressKeyPair>::AddressPublicKey,
        Predicate = Self::Predicate,
    >;
    type Transaction: Transaction<SerialNumber = <Self::Record as Record>::SerialNumber>;
    type Block;
    type LocalData;

    /// Returns public parameters for the DPC.
    fn setup<R: Rng>(ledger_parameters: &L::Parameters, rng: &mut R) -> Result<Self::Parameters, DPCError>;

    /// Returns an address key pair, given public parameters, metadata, and an
    /// rng.
    fn create_address<R: Rng>(
        parameters: &Self::Parameters,
        metadata: &Self::Metadata,
        rng: &mut R,
    ) -> Result<Self::AddressKeyPair, DPCError>;

    /// Returns new records and a transaction based on the authorized
    /// consumption of old records.
    fn execute<R: Rng>(
        parameters: &Self::Parameters,

        old_records: &[Self::Record],
        old_address_secret_keys: &[<Self::AddressKeyPair as AddressKeyPair>::AddressSecretKey],
        old_private_pred_input: impl FnMut(&Self::LocalData) -> Result<Vec<Self::PrivatePredInput>, DPCError>,

        new_address_public_keys: &[<Self::AddressKeyPair as AddressKeyPair>::AddressPublicKey],
        new_is_dummy_flags: &[bool],
        new_payloads: &[Self::Payload],
        new_birth_predicates: &[Self::Predicate],
        new_death_predicates: &[Self::Predicate],
        new_private_pred_input: impl FnMut(&Self::LocalData) -> Result<Vec<Self::PrivatePredInput>, DPCError>,

        auxiliary: &Self::Auxiliary,
        memorandum: &<Self::Transaction as Transaction>::Memorandum,
        ledger: &L,
        rng: &mut R,
    ) -> Result<(Vec<Self::Record>, Self::Transaction), DPCError>;

    /// Returns true iff the transaction is valid according to the ledger.
    fn verify(parameters: &Self::Parameters, transaction: &Self::Transaction, ledger: &L) -> Result<bool, DPCError>;

    /// Returns true iff all the transactions in the block are valid according to the ledger.
    fn verify_block(parameters: &Self::Parameters, block: &Self::Block, ledger: &L) -> Result<bool, DPCError>;
}
