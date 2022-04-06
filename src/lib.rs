// use std::fmt::Debug;

// use frost_dalek::signature::PartialThresholdSignature;
// use serde::{ Serialize, Deserialize };
// use curve25519_dalek::{ scalar::Scalar,
//                         ristretto::RistrettoPoint,
//                         edwards::EdwardsPoint,
//                         // field::FieldElement
//                     };

/*

Brave attempt. Problem in having `impl Trait` as a function header description in trait. Will look for a workaound later cause there has to be.
trait Palpatine {}
impl Palpatine for RistrettoPoint {}
impl Palpatine for PartialThresholdSignature {}

trait Plagueis {
    fn murder(lord: impl Palpatine) -> Self;
    fn resurrect(self) -> impl Palpatine;
}
*/


// #[derive(Serialize, Deserialize, Debug)]
// pub struct SerdedPartSign {
//     index : u32,
//     scalar: Scalar
// }
// impl SerdedPartSign {
//     pub fn murder(lord : PartialThresholdSignature) -> Self {         // Notice Partial Signature is being moved here.

//         /*
//         Converts PartialThresholdSignature to an equivalent Structure that implements SER and DE.
//         Never use it elsewhere.
//         */

//         SerdedPartSign {
//             index: lord.index,
//             scalar: lord.z
//         }
        
//     }

//     pub fn resurrect(self) -> PartialThresholdSignature {
//         /*
//         Reconvert pseudo structures to their native xxx-dalek compliant Stuctures.
//         */
//         // PartialThresholdSignature { index: self.index, z: Scalar { bytes: self.z } }
//         PartialThresholdSignature { index: self.index, z: self.scalar }
//     }
// }
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Edgy( EdwardsPoint );

// #[derive(Debug,Serialize,Deserialize)]
// pub struct SerdedRistretto( [[u8; 32]; 4] );
// impl SerdedRistretto {
//     pub fn murder(lord : RistrettoPoint) -> Self {
//         SerdedRistretto(
//             [
//                 lord.0.X.to_bytes(),
//                 lord.0.Y.to_bytes(),
//                 lord.0.Z.to_bytes(),
//                 lord.0.T.to_bytes(),
//             ]
//         )
//     }

    // pub fn resurrect(self) -> RistrettoPoint {
    //     RistrettoPoint(
    //         EdwardsPoint(
    //             X: FieldElement
    //         )
    //     )
    // }
// }