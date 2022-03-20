use std::vec;

use frost_dalek::{Parameters, Participant, DistributedKeyGeneration};

/*
Comments are mostly summary / comments from the docs.
*/
fn main() -> std::io::Result<()> {
    println!("Czech or Slovakia?");
    let params = Parameters { t:2, n:3 };

    let (david, david_coef) = Participant::new(&params, 1); // The 1 here is called the _participant index_
    let (kristian, kristian_coef) = Participant::new(&params, 2);
    let (suyash, suyash_coef) = Participant::new(&params, 3);

    /*
    These participant indices need to be agree-ed upon out of scope. We could use our UCO's here. like ID numbers.
    As we can also make them public. THat edit will be tried later.
    
    - Okay these structs by our names need to be shared : david, kritian..
    The corresponding coeffs are private.
    */

    // Each of us need to verify the other 2 person's zkp by doing this:
    /* EDIT IN YOUR SERVERS */
    david.proof_of_secret_key.verify(&david.index, &david.public_key().unwrap());

    kristian.proof_of_secret_key.verify(&kristian.index, &kristian.public_key().unwrap());

    // Suyash enters round one of the distributed key exchange
    let mut suyash_other_parts: Vec<Participant> = vec![david.clone(),kristian.clone()];

    // I seemingly verify your zkp's again. This time for the purpose of Dist. key gen.
    let suyash_state = DistributedKeyGeneration::<_>::new(&params, &suyash.index, &suyash_coef, &mut suyash_other_parts).unwrap();

    // This is a secret share step. Needs to be done with auth/enc etc. This is what I will share with other participants.
    // From docs: Retrieve a secret share for each other participant, to be given to them at the end of DistributedKeyGeneration::<RoundOne>
    let suyash_their_secret_shares = match suyash_state.their_secret_shares() {
        Ok(v) => v,
        Err(e) => panic!(" Error producing secret to share: {:?}", e)
        
    };
    dbg!(&suyash_their_secret_shares);

    /*
    To be done later:
    send_to_david(suyash_their_secret_shares)
    send_to_kristian(suyash_their_secret_shares)
    
    .. David and Kristian have to do the same.

    Each of us will then have a vector of secret shares from the other participant.

    I am gonna create your secret shares here, but in reality I am having these
    sent from you: (Remove during production)
    */

    let mut david_other_parts: Vec<Participant> = vec![suyash.clone(), kristian.clone()];
    let david_state = DistributedKeyGeneration::<_>::new(&params, &david.index, &david_coef, &mut david_other_parts).unwrap();
    let david_their_secret_shares = david_state.their_secret_shares().unwrap(); // You would handle the error in your code and return me the unwrapped value if no error.
    dbg!(&david_their_secret_shares);

    let mut kristian_other_parts: Vec<Participant> = vec![david.clone(), suyash.clone()];
    let kristian_state = DistributedKeyGeneration::<_>::new(&params, &kristian.index, &kristian_coef, &mut kristian_other_parts).unwrap();
    let kristian_their_secret_shares = kristian_state.their_secret_shares().unwrap();
    dbg!(&kristian_their_secret_shares);
    /* Foreign code ends. Main code starts */

    //This is what I have gotten from you two
    let suyash_my_secret_shares = vec![david_their_secret_shares[0].clone(), kristian_their_secret_shares[1].clone()];

    /* Foreign code */
    let kristian_my_secret_shares = vec![david_their_secret_shares[1].clone(), suyash_their_secret_shares[1].clone()];
    let david_my_secret_shares = vec![kristian_their_secret_shares[0].clone(), suyash_their_secret_shares[0].clone()];

    //State updates. Advancing to round 2:

    let suyash_state = match suyash_state.to_round_two(suyash_my_secret_shares) {
        Ok(v) => v,
        Err(()) => panic!(" Suyash can't move to round 2")
    };

    /*Foreign code */
    let david_state = david_state.to_round_two(david_my_secret_shares).unwrap();
    let kristian_state = kristian_state.to_round_two(kristian_my_secret_shares).unwrap();

    let (suyash_group_key, suyash_secret_key) = suyash_state.finish(suyash.public_key().unwrap()).unwrap(); //Too tired to make a panic code.

    /*Foreign code */

    let (david_group_key, david_secret_key) = david_state.finish(david.public_key().unwrap()).unwrap();
    let (kristian_group_key, kristian_secret_key) = kristian_state.finish(kristian.public_key().unwrap()).unwrap();

    assert!(suyash_group_key == kristian_group_key);
    assert!(suyash_group_key == david_group_key);

    let suyash_public_key = suyash_secret_key.to_public();
    let david_public_key = david_secret_key.to_public();
    let kristian_public_key = kristian_secret_key.to_public();

    println!("Suyash's public key: {:?}", suyash_public_key);
    println!("Dave's public key: {:?}", david_public_key);
    println!("Kristi's public key: {:?}", kristian_public_key);

    Ok(())

}
