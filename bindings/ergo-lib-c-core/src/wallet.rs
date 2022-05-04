//! Wallet-like features

use ergo_lib::ergotree_ir::sigma_protocol::sigma_boolean::SigmaBoolean;

use crate::{
    address::{Address, ConstAddressPtr},
    collections::ConstCollectionPtr,
    ergo_box::ErgoBox,
    ergo_state_ctx::ConstErgoStateContextPtr,
    reduced::ConstReducedTransactionPtr,
    secret_key::SecretKey,
    transaction::{
        ConstTransactionHintsBagPtr, ConstUnsignedTransactionPtr, Transaction, TransactionHintsBag,
        TransactionHintsBagPtr, TransactionPtr,
    },
    util::{const_ptr_as_ref, mut_ptr_as_mut},
    Error,
};

/// A collection of secret keys. This simplified signing by matching the secret keys to the correct inputs automatically.
pub struct Wallet(ergo_lib::wallet::Wallet);
pub type WalletPtr = *mut Wallet;
pub type ConstWalletPtr = *const Wallet;

/// Create `Wallet` instance loading secret key from mnemonic Returns Err if a DlogSecretKey cannot be
/// parsed from the provided phrase
pub unsafe fn wallet_from_mnemonic(
    mnemonic_phrase: &str,
    mnemonic_pass: &str,
    wallet_out: *mut WalletPtr,
) -> Result<(), Error> {
    let wallet_out = mut_ptr_as_mut(wallet_out, "wallet_out")?;
    if let Some(wallet_inner) =
        ergo_lib::wallet::Wallet::from_mnemonic(mnemonic_phrase, mnemonic_pass)
    {
        *wallet_out = Box::into_raw(Box::new(Wallet(wallet_inner)));
        Ok(())
    } else {
        Err(Error::Misc(
            "Wallet.from_mnemonic: DlogSecretKey can't be parsed from the provided phrase".into(),
        ))
    }
}

/// Create `Wallet` from secrets
pub unsafe fn wallet_from_secrets(
    secret_keys_ptr: ConstCollectionPtr<SecretKey>,
    wallet_out: *mut WalletPtr,
) -> Result<(), Error> {
    let secret_keys = const_ptr_as_ref(secret_keys_ptr, "secret_keys_ptr")?;
    let wallet_out = mut_ptr_as_mut(wallet_out, "wallet_out")?;
    *wallet_out = Box::into_raw(Box::new(Wallet(ergo_lib::wallet::Wallet::from_secrets(
        secret_keys.0.clone().into_iter().map(|s| s.0).collect(),
    ))));
    Ok(())
}

/// Add a new secret to the wallets prover
pub unsafe fn wallet_add_secret(
    wallet_ptr: WalletPtr,
    secret_key_ptr: *mut SecretKey,
) -> Result<(), Error> {
    let wallet = mut_ptr_as_mut(wallet_ptr, "wallet_ptr")?;
    let sk = mut_ptr_as_mut(secret_key_ptr, "secret_key_ptr")?;
    wallet.0.add_secret(sk.0.clone());
    Ok(())
}

/// Signs a transaction
pub unsafe fn wallet_sign_transaction(
    wallet_ptr: ConstWalletPtr,
    state_context_ptr: ConstErgoStateContextPtr,
    unsigned_tx_ptr: ConstUnsignedTransactionPtr,
    boxes_to_spend_ptr: ConstCollectionPtr<ErgoBox>,
    data_boxes_ptr: ConstCollectionPtr<ErgoBox>,
    transaction_out: *mut TransactionPtr,
) -> Result<(), Error> {
    let wallet = const_ptr_as_ref(wallet_ptr, "wallet_ptr")?;
    let state_context = const_ptr_as_ref(state_context_ptr, "state_context_ptr")?;
    let unsigned_tx = const_ptr_as_ref(unsigned_tx_ptr, "unsigned_tx_ptr")?;
    let boxes_to_spend = const_ptr_as_ref(boxes_to_spend_ptr, "boxes_to_spend_ptr")?;
    let data_boxes = const_ptr_as_ref(data_boxes_ptr, "data_boxes_ptr")?;
    let transaction_out = mut_ptr_as_mut(transaction_out, "transaction_out")?;
    let boxes_to_spend = boxes_to_spend.0.clone().into_iter().map(|b| b.0).collect();
    let data_boxes = data_boxes.0.clone().into_iter().map(|b| b.0).collect();
    let tx_context = ergo_lib::wallet::signing::TransactionContext::new(
        unsigned_tx.0.clone(),
        boxes_to_spend,
        data_boxes,
    )?;
    let tx = wallet
        .0
        .sign_transaction(tx_context, &state_context.0, None)?;
    *transaction_out = Box::into_raw(Box::new(Transaction(tx)));
    Ok(())
}

/// Signs a multi signature transaction
pub unsafe fn wallet_sign_transaction_multi(
    wallet_ptr: ConstWalletPtr,
    state_context_ptr: ConstErgoStateContextPtr,
    unsigned_tx_ptr: ConstUnsignedTransactionPtr,
    boxes_to_spend_ptr: ConstCollectionPtr<ErgoBox>,
    data_boxes_ptr: ConstCollectionPtr<ErgoBox>,
    tx_hints_ptr: ConstTransactionHintsBagPtr,
    transaction_out: *mut TransactionPtr,
) -> Result<(), Error> {
    let wallet = const_ptr_as_ref(wallet_ptr, "wallet_ptr")?;
    let state_context = const_ptr_as_ref(state_context_ptr, "state_context_ptr")?;
    let unsigned_tx = const_ptr_as_ref(unsigned_tx_ptr, "unsigned_tx_ptr")?;
    let boxes_to_spend = const_ptr_as_ref(boxes_to_spend_ptr, "boxes_to_spend_ptr")?;
    let data_boxes = const_ptr_as_ref(data_boxes_ptr, "data_boxes_ptr")?;
    let tx_hints = const_ptr_as_ref(tx_hints_ptr, "tx_hints_ptr")?;
    let transaction_out = mut_ptr_as_mut(transaction_out, "transaction_out")?;
    let boxes_to_spend = boxes_to_spend.0.clone().into_iter().map(|b| b.0).collect();
    let data_boxes = data_boxes.0.clone().into_iter().map(|b| b.0).collect();
    let tx_context = ergo_lib::wallet::signing::TransactionContext::new(
        unsigned_tx.0.clone(),
        boxes_to_spend,
        data_boxes,
    )?;
    let tx = wallet
        .0
        .sign_transaction(tx_context, &state_context.0, Some(&tx_hints.0))?;
    *transaction_out = Box::into_raw(Box::new(Transaction(tx)));
    Ok(())
}

/// Signs a reduced transaction (generating proofs for inputs)
pub unsafe fn wallet_sign_reduced_transaction(
    wallet_ptr: ConstWalletPtr,
    reduced_tx_ptr: ConstReducedTransactionPtr,
    transaction_out: *mut TransactionPtr,
) -> Result<(), Error> {
    let wallet = const_ptr_as_ref(wallet_ptr, "wallet_ptr")?;
    let reduced_tx = const_ptr_as_ref(reduced_tx_ptr, "reduced_tx_ptr")?;
    let transaction_out = mut_ptr_as_mut(transaction_out, "transaction_out")?;
    let tx = wallet
        .0
        .sign_reduced_transaction(reduced_tx.0.clone(), None)
        .map(Transaction)?;
    *transaction_out = Box::into_raw(Box::new(tx));
    Ok(())
}

/// Signs a multi signature reduced transaction
pub unsafe fn wallet_sign_reduced_transaction_multi(
    wallet_ptr: ConstWalletPtr,
    reduced_tx_ptr: ConstReducedTransactionPtr,
    tx_hints_ptr: ConstTransactionHintsBagPtr,
    transaction_out: *mut TransactionPtr,
) -> Result<(), Error> {
    let wallet = const_ptr_as_ref(wallet_ptr, "wallet_ptr")?;
    let reduced_tx = const_ptr_as_ref(reduced_tx_ptr, "reduced_tx_ptr")?;
    let transaction_out = mut_ptr_as_mut(transaction_out, "transaction_out")?;
    let tx_hints = const_ptr_as_ref(tx_hints_ptr, "tx_hints_ptr")?;
    let tx = wallet
        .0
        .sign_reduced_transaction(reduced_tx.0.clone(), Some(&tx_hints.0))
        .map(Transaction)?;
    *transaction_out = Box::into_raw(Box::new(tx));
    Ok(())
}

/// Generate Commitments for unsigned tx
pub unsafe fn wallet_generate_commitments(
    wallet_ptr: ConstWalletPtr,
    state_context_ptr: ConstErgoStateContextPtr,
    unsigned_tx_ptr: ConstUnsignedTransactionPtr,
    boxes_to_spend_ptr: ConstCollectionPtr<ErgoBox>,
    data_boxes_ptr: ConstCollectionPtr<ErgoBox>,
    transaction_hints_bag_out: *mut TransactionHintsBagPtr,
) -> Result<(), Error> {
    let wallet = const_ptr_as_ref(wallet_ptr, "wallet_ptr")?;
    let state_context = const_ptr_as_ref(state_context_ptr, "state_context_ptr")?;
    let unsigned_tx = const_ptr_as_ref(unsigned_tx_ptr, "unsigned_tx_ptr")?;
    let boxes_to_spend = const_ptr_as_ref(boxes_to_spend_ptr, "boxes_to_spend_ptr")?;
    let data_boxes = const_ptr_as_ref(data_boxes_ptr, "data_boxes_ptr")?;
    let transaction_hints_bag_out = mut_ptr_as_mut(transaction_hints_bag_out, "transaction_out")?;
    let boxes_to_spend = boxes_to_spend.0.clone().into_iter().map(|b| b.0).collect();
    let data_boxes = data_boxes.0.clone().into_iter().map(|b| b.0).collect();
    let tx_context = ergo_lib::wallet::signing::TransactionContext::new(
        unsigned_tx.0.clone(),
        boxes_to_spend,
        data_boxes,
    )?;
    *transaction_hints_bag_out = Box::into_raw(Box::new(TransactionHintsBag(
        wallet
            .0
            .generate_commitments(tx_context, &state_context.0)?,
    )));
    Ok(())
}

/// Generate Commitments for reduced transaction
pub unsafe fn wallet_generate_commitments_for_reduced_transaction(
    wallet_ptr: ConstWalletPtr,
    reduced_tx_ptr: ConstReducedTransactionPtr,
    transaction_hints_bag_out: *mut TransactionHintsBagPtr,
) -> Result<(), Error> {
    let wallet = const_ptr_as_ref(wallet_ptr, "wallet_ptr")?;
    let reduced_tx = const_ptr_as_ref(reduced_tx_ptr, "reduced_tx_ptr")?;
    let transaction_hints_bag_out = mut_ptr_as_mut(transaction_hints_bag_out, "transaction_out")?;
    *transaction_hints_bag_out = Box::into_raw(Box::new(TransactionHintsBag(
        wallet
            .0
            .generate_commitments_for_reduced_transaction(reduced_tx.0.clone())?,
    )));
    Ok(())
}

/// Represents the signature of a signed message
pub struct SignedMessage(Vec<u8>);
pub type SignedMessagePtr = *mut SignedMessage;
pub type ConstSignedMessagePtr = *const SignedMessage;

/// Sign an arbitrary message using a P2PK address
pub unsafe fn wallet_sign_message_using_p2pk(
    wallet_ptr: ConstWalletPtr,
    address_ptr: ConstAddressPtr,
    message_ptr: *const u8,
    message_length: usize,
    signed_message_out: *mut SignedMessagePtr,
) -> Result<(), Error> {
    let wallet = const_ptr_as_ref(wallet_ptr, "wallet_ptr")?;
    let address = const_ptr_as_ref(address_ptr, "address_ptr")?;
    let msg = std::slice::from_raw_parts(message_ptr, message_length);
    let signed_message_out = mut_ptr_as_mut(signed_message_out, "signed_message_out")?;
    if let Address(ergo_lib::ergotree_ir::chain::address::Address::P2Pk(d)) = address {
        let sb = SigmaBoolean::from(d.clone());
        let sig = wallet.0.sign_message(sb, msg)?;
        *signed_message_out = Box::into_raw(Box::new(SignedMessage(sig)));
        Ok(())
    } else {
        Err(Error::Misc(
            "wallet::sign_message_using_p2pk: Address:P2Pk expected".into(),
        ))
    }
}

/// Verify that the signature is presented to satisfy SigmaProp conditions.
pub unsafe fn verify_signature(
    address_ptr: ConstAddressPtr,
    message_ptr: *const u8,
    message_length: usize,
    signed_message_ptr: ConstSignedMessagePtr,
) -> Result<bool, Error> {
    let address = const_ptr_as_ref(address_ptr, "address_ptr")?;
    let msg = std::slice::from_raw_parts(message_ptr, message_length);
    let signed_message = const_ptr_as_ref(signed_message_ptr, "signed_message_ptr")?;

    if let Address(ergo_lib::ergotree_ir::chain::address::Address::P2Pk(d)) = address {
        let sb = SigmaBoolean::from(d.clone());
        let res = ergo_lib::ergotree_interpreter::sigma_protocol::verifier::verify_signature(
            sb,
            msg,
            signed_message.0.as_slice(),
        )?;
        Ok(res)
    } else {
        Err(Error::Misc(
            "wallet::verify_signature: Address:P2Pk expected".into(),
        ))
    }
}
