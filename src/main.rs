use frost_dalek::{Parameters, Participant};

fn main() {
    println!("Czech or Slovakia?");
    let params = Parameters { t:2, n:3 };

    let (david, david_coef) = Participant::new(&params, 1); // The 1 here is called the _participant index_
    let (kristian, kritian_coef) = Participant::new(&params, 2);
    let (suyash, suyash_coef) = Participant::new(&params, 3);

    /*
    These participant indices need to be agree-ed upon out of scope. We could use our UCO's here. like ID numbers.
    As we can also make them public. THat edit will be tried later.
    This also helps saving an exchange step.
    */

}
