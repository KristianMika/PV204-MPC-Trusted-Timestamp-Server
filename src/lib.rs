use frost_dalek::signature::PartialThresholdSignature;
use serde::{ Serialize, Deserialize };
use curve25519_dalek::scalar::Scalar;

#[derive(Serialize, Deserialize, Debug)]
pub struct PartSignSerded {
    index : u32,
    z : [u8; 32]
}
pub fn serde_partsign(ps : PartialThresholdSignature) -> PartSignSerded {         // Notice Partial Signature is being moved here.

    /*
    Converts PartialThresholdSignature to an equivalent Structure that implements SER and DE.
    Never use it elsewhere.
    */

    PartSignSerded {
        index: ps.index,
        z: ps.z.bytes
    }
    
}

pub fn deser_partsign(pss : PartSignSerded) -> PartialThresholdSignature {
    PartialThresholdSignature { index: pss.index, z: Scalar { bytes: pss.z } }
}