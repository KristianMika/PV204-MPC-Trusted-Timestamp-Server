use frost_dalek::signature::PartialThresholdSignature;
use serde::{ Serialize, Deserialize };
use curve25519_dalek::{scalar::Scalar, ristretto::RistrettoPoint};


#[derive(Serialize, Deserialize, Debug)]
pub struct SerdedPartSign {
    index : u32,
    z : [u8; 32]
}
impl SerdedPartSign {
    pub fn murder(lord : PartialThresholdSignature) -> Self {         // Notice Partial Signature is being moved here.

        /*
        Converts PartialThresholdSignature to an equivalent Structure that implements SER and DE.
        Never use it elsewhere.
        */

        SerdedPartSign {
            index: lord.index,
            z: lord.z.bytes
        }
        
    }

    pub fn resurrect(self) -> PartialThresholdSignature {
        /*
        Reconvert pseudo structures to their native xxx-dalek compliant Stuctures.
        */
        PartialThresholdSignature { index: self.index, z: Scalar { bytes: self.z } }
    }
}
#[derive(Debug,Serialize,Deserialize)]
pub struct RistrettoSerded( [u64; 5] );

pub fn serde_ristretto(ris : RistrettoPoint) -> RistrettoSerded {
    RistrettoSerded([0; 5])
}