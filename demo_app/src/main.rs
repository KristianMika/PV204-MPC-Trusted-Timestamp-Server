use serde::{Serialize, Deserialize};
use frost_dalek::signature::ThresholdSignature;

use sha2::{Digest, Sha256};


async fn main() {
    // needed structures
    #[derive(Serialize)]
    struct TimestampStruct {
        hashAlgorithm: String,
        hashedMessage: String,
    }
    #[derive(Deserialize)]
    pub struct TimeStampResp {
        status: String,
        timeStampToken: Vec<u8>,
        final_hash: Vec<u8>,
    }


    let client = reqwest::Client::new();

    // trigger keygen
    let keygen_res = client.post("http://127.0.0.1:8080/keygen");

    let mut hasher = Sha256::new();
    hasher.update("random text to be hashed, would be file otherwise");
    let fin_hash = hasher.finalize();

    //asking for timestam
    let body = TimestampStruct{ hashAlgorithm: "SHA2".to_string(), hashedMessage: fin_hash  };
    let kgb = serde_json::to_string(&body).unwrap();

    let timestamp_res = client.post("http://127.0.0.1:8080/timestamp")
        .json(&kgb)
        .send()
        .await?;
    println!("{:?}",timestamp_res);

    let signtoken : TimeStampResp = timestamp_res.json().unwrap();
    if signtoken.status = "Ok?" {
        println!(hex::encode(signtoken.timeStampToken));
    } else {
        println!("Sign wasn't successfull.");
    }

    let slice = signtoken.timeStampToken.as_slice();
    let array: [u8; 64] = match slice.try_into() {
        Ok(ba) => ba,
        Err(_) => panic!("Expected a Vec of length {} but it was {}", 32, signtoken.timeStampToken.len()),
    };
    let signature = ThresholdSignature::from_bytes(array.unwrap());

    let slice_final_hash = signtoken.final_hash.as_slice();
    let array_hash: [u8; 64] = match slice_final_hash.try_into() {
        Ok(ba) => ba,
        Err(_) => panic!("Expected a Vec of length {} but it was {}", 32, signtoken.timeStampToken.len()),
    };


    // getting the pubkey
    let group_key_res = reqwest::get("http://127.0.0.1:8080/groupkey")
                                .await?
                                .text()
                                .await?;

    println!("Raw response:\n{:?}", group_key_res);
    let group_key: GroupKey  = group_key_res.json().unwrap();
    

    let verified = threshold_signature.verify(&group_key, &array_hash.unwrap()).unwrap();



}
