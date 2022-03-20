use frost_dalek::{Parameters, Participant, DistributedKeyGeneration};

/*
Comments are mostly summary / comments from the docs.
*/
fn main() -> std::io::Result<()> {
    println!("Czech or Slovakia?");
    let params = Parameters { t:2, n:3 };

    let (david, david_coef) = Participant::new(&params, 1); // The 1 here is called the _participant index_
    let (kristian, kritian_coef) = Participant::new(&params, 2);
    let (suyash, suyash_coef) = Participant::new(&params, 3);

    /*
    These participant indices need to be agree-ed upon out of scope. We could use our UCO's here. like ID numbers.
    As we can also make them public. THat edit will be tried later.
    This also helps saving an exchange step.
    
    - Okay these structs by our names need to be shared : david, kritian..
    The corresponding coeffs are private.
    */

    // Each of us need to verify the other 2 person's zkp by doing this:
    /* EDIT IN YOUR SERVERS */
    let daveproof = match david.proof_of_secret_key.verify(&david.index, &david.public_key().unwrap()) {
        Ok(v) => v,
        Err(e) => panic!("Not David! NOT DAVID")
    };

    

    let krisproof = match kristian.proof_of_secret_key.verify(&kristian.index, &kristian.public_key().unwrap()) {
        Ok(v) => v,
        Err(e) => panic!("Not Kristian! NOT KRISTIAN")
    };

    // Suyash enters round one of the distributed key exchange
    let mut suyash_other_parts: Vec<Participant> = vec![david.clone(),kristian.clone()];

    // I seemingly verify your zkp's again. This time for the purpose of Dist. key gen.
    let suyash_state = DistributedKeyGeneration::<_>::new(&params, &suyash.index, &suyash_coef, &mut suyash_other_parts).unwrap();

    // This is a secret share step. Needs to be done with auth/enc etc. This is what I will share with other participants.
    let suyash_their_secret_shares = suyash_state.their_secret_shares();

    /*
    To be done later:
    send_to_david(suyash_their_secret_shares)
    send_to_kristian(suyash_their_secret_shares)
    
    .. David and Kristian have to do the same.

    I am gonna create your secret shares here, but in reality I am having these
    sent from you: (Remove during production)
    */

    let mut david_other_parts: Vec<Participant> = vec![suyash.clone(), kristian.clone()];

    let david_state = DistributedKeyGeneration::<_>::new(&params, &david.index, &david_coef, &mut david_other_parts).unwrap();

    

    Ok(())

}
