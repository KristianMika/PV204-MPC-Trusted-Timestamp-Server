use frost_dalek::{Parameters,
                Participant,
                DistributedKeyGeneration,
                compute_message_hash,
                generate_commitment_share_lists,
                SignatureAggregator};

use rand::rngs::OsRng;

/*
Comments are mostly summary / comments from the docs.
*/
fn main() {
    let params = Parameters { t:2, n:3 };

    let (david, david_coef) = Participant::new(&params, 1);
    let (kristian, kristian_coef) = Participant::new(&params, 2);
    let (suyash, suyash_coef) = Participant::new(&params, 3);

    /*
    These participant indices need to be agree-ed upon out of scope. I did it alphabetically. But it also makes for a cool abbreviation: DIKS.
    Which stands for DIstributed Key Signing.
    They are kinda public knowledge, but ironically, not public fields of the structs. So in the code we need to know what index to use for what participant.
    
    These structs by our names need to be shared : david, kristian..
    The corresponding coeffs are private.

    Each of us need to verify the other 2 person's zkp by doing this:

    !! EDIT IN YOUR SERVERS */
    david.proof_of_secret_key.verify(&david.index, &david.public_key().unwrap()).expect("Not David! NOT DAVID!!!!!");

    kristian.proof_of_secret_key.verify(&kristian.index, &kristian.public_key().unwrap()).expect("Not Kristian! NOT KRISTIAN!!!");

    // Round 1 of establishing keys.
    let mut suyash_other_parts: Vec<Participant> = vec![david.clone(),kristian.clone()];
    let suyash_state = DistributedKeyGeneration::<_>::new(&params, &suyash.index, &suyash_coef, &mut suyash_other_parts).unwrap();

    // This is the secret I will share at the end of my Round 1.
    let suyash_gives_secrets = suyash_state.their_secret_shares().expect(" Suyash can't create the secretes to share");

    /*
    To be done later:
    send_to_david(suyash_gives_secrets)
    send_to_kristian(suyash_gives_secrets)
    
    .. David and Kristian have to do the same.

    Each of us will then have a vector of secret shares from the other participants.

    I am gonna create your secret shares here, but in reality I am having these
    sent from you: (Remove during production)
    */

    let mut david_other_parts: Vec<Participant> = vec![suyash.clone(), kristian.clone()];
    let david_state = DistributedKeyGeneration::<_>::new(&params, &david.index, &david_coef, &mut david_other_parts).unwrap();
    let david_gives_secrets = david_state.their_secret_shares().unwrap(); // You would handle the error in your code and return me the unwrapped value if no error.

    let mut kristian_other_parts: Vec<Participant> = vec![david.clone(), suyash.clone()];
    let kristian_state = DistributedKeyGeneration::<_>::new(&params, &kristian.index, &kristian_coef, &mut kristian_other_parts).unwrap();
    let kristian_gives_secrets = kristian_state.their_secret_shares().unwrap();
    /* Foreign code ends. Main code starts */

    //I collate the secrets I received from you 2 into a vector.
    /* WATCH THE INDEXES. Each get a secret made FOR them, BY the other parties. */
    let suyash_gets_secrets = vec![david_gives_secrets[0].clone(), kristian_gives_secrets[1].clone()];

    /* Foreign code */
    let kristian_gets_secrets = vec![david_gives_secrets[1].clone(), suyash_gives_secrets[1].clone()];
    let david_gets_secrets = vec![kristian_gives_secrets[0].clone(), suyash_gives_secrets[0].clone()];

    // ---------------------------------------------------------------
    
    //Round 2 begins. Update the states!

    let suyash_state = match suyash_state.to_round_two(suyash_gets_secrets) {
        Ok(v) => v,
        Err(()) => panic!(" Suyash can't move to round 2")
    };

    /*Foreign code */
    let david_state = david_state.to_round_two(david_gets_secrets).unwrap();
    let kristian_state = kristian_state.to_round_two(kristian_gets_secrets).unwrap();

    // Finishing the 2nd round and deriving the group key and secret key from the latest updated state.
    let (suyash_group_key, suyash_secret_key) = suyash_state.finish(suyash.public_key().expect("Suyash public key access error")).expect("Suyash pooped deriving his group and secret keys");

    /*Foreign code */
    let (david_group_key, david_secret_key) = david_state.finish(david.public_key().unwrap()).unwrap();
    let (kristian_group_key, kristian_secret_key) = kristian_state.finish(kristian.public_key().unwrap()).unwrap();

    // Checking if we all got the same group keys. Since GROUP KEY IS OUR PUBLIC KEY we can virtually "shout it out loud"
    assert!(suyash_group_key == kristian_group_key);
    assert!(suyash_group_key == david_group_key);

    let suyash_public_key = suyash_secret_key.to_public();
    let david_public_key = david_secret_key.to_public();
    let kristian_public_key = kristian_secret_key.to_public();

    println!("Suyash's public key: {:?}", suyash_public_key);
    println!("Davi's public key: {:?}", david_public_key);
    println!("Kristi's public key: {:?}", kristian_public_key);

    /*========= KEY ESTABLISHMENT OVER ============= */
    /*========= Signing ============= */

    let (ash_public_comshares, mut ash_secret_comshares) = generate_commitment_share_lists(&mut OsRng, 3, 1);
    let (kris_public_comshares, mut kris_secret_comshares) = generate_commitment_share_lists(&mut OsRng, 2, 1);
    let (dave_public_comshares, mut dave_secret_comshares) = generate_commitment_share_lists(&mut OsRng, 1, 1);

    /* CONTEXT = A byte string, kinda public, pertinent to this application. So this will be a constant for the group. */
    const CONTEXT: &[u8] = b"PV204_PETR_SVENDA_ANTONIN_DUFKA";
    let message = b"MV013 sucks. I don't wanna study this course";

    let message_hash = compute_message_hash(&CONTEXT[..], &message[..]);

    /* This aggregator will assign signers, pull in their signatures, and finalise theri sign.
    The best part is, we don't trust the aggregator. THE AGGREGATOR IS UNTRUSTED. It could be our user, one of us, a standalone app, Dufka, Jennifer Lawrence, anyone we want.*/
    let mut aggregator = SignatureAggregator::new(params, suyash_group_key.clone(), &CONTEXT[..], &message[..]);

    /*Aggregator nominates Kristian and I */
    // Don't know where the 0 index in the published_commitment_share is coming from. Right now.
    aggregator.include_signer(2, kris_public_comshares.commitments[0], kristian_public_key);
    aggregator.include_signer(3, ash_public_comshares.commitments[0], suyash_public_key);

    /*The aggregator should then publicly announce which participants are expected to be signers. */
    let signers = aggregator.get_signers();
    // println!("The signers are: {:?}",signers);

    // No idea what how we get commitment share index
    let kris_partial = match kristian_secret_key.sign(&message_hash, &kristian_group_key, &mut kris_secret_comshares, 0, signers) {
        Ok(v) => v,
        Err(e) => panic!("Kristian is corrupt!!!\n{}",e)
    };
    let ash_partial = match suyash_secret_key.sign(&message_hash, &suyash_group_key, &mut ash_secret_comshares, 0, signers) {
        Ok(v) => v,
        Err(e) => panic!("Suyash is having some trouble signing:\n{}",e)
    };

    aggregator.include_partial_signature(kris_partial);
    aggregator.include_partial_signature(ash_partial);

    /* Aggregation begins */

    let aggregator = match aggregator.finalize() {
        Ok(v) => v,
        Err(e) => panic!("Aggregator pooped!\n{:?}",e)
    };

    let threshold_sign= match aggregator.aggregate() {
        Ok(v) => v,
        Err(e) => panic!("Bad signing. Likely corrupted signees or signatures!\n{:?}",e)
    };

    println!("The message: {:?}\nThe Context: {:?}\nThe Signature: {:?}", &message, &CONTEXT, threshold_sign);
}
