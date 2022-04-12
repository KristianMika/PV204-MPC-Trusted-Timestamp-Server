use frost_dalek::signature::ThresholdSignature;
use frost_dalek::GroupKey;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::error::Error;
use std::{thread, time};

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

fn to_array(v: &Vec<u8>) -> [u8; 64] {
    let slice = v.as_slice();
    let array: [u8; 64] = match slice.try_into() {
        Ok(ba) => ba,
        Err(_) => panic!("Expected a Vec of length {} but it was {}", 64, v.len()),
    };
    array
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    // trigger keygen
    let keygen_res = client.post("http://127.0.0.1:8080/keygen").send().await;

    // give some time for keygen

    let ten_millis = time::Duration::from_millis(1000);

    thread::sleep(ten_millis);

    let mut hasher = Sha256::new();
    hasher.update("random text to be hashed, would be file otherwise");
    let fin_hash = hasher.finalize();

    //asking for timestam
    let body = TimestampStruct {
        hashAlgorithm: "SHA2".to_string(),
        hashedMessage: format!("{:x}", fin_hash),
    };

    let timestamp_res = reqwest::Client::new()
        .post("http://127.0.0.1:8080/timestamp")
        .json(&body)
        .send()
        .await?;
    // println!("{:?}", timestamp_res.body());

    let signtoken: TimeStampResp = timestamp_res.json::<TimeStampResp>().await.unwrap();
    if signtoken.status == "Ok?" {
        println!("{}", hex::encode(&signtoken.timeStampToken));
    } else {
        println!("Sign wasn't successfull.");
    }

    let signature = ThresholdSignature::from_bytes(to_array(&signtoken.timeStampToken)).unwrap();

    let slice_final_hash = signtoken.final_hash;
    let array_hash: [u8; 64] = match slice_final_hash.try_into() {
        Ok(ba) => ba,
        Err(_) => panic!(
            "Expected a Vec of length {} but it was {}",
            32,
            signtoken.timeStampToken.len()
        ),
    };

    // getting the pubkey
    let group_key: GroupKey = reqwest::get("http://127.0.0.1:8080/groupkey")
        .await
        .unwrap()
        .json::<GroupKey>()
        .await
        .unwrap();

    match signature.verify(&group_key, &array_hash) {
        Ok(_) => println!("Timestamp verification verified"),
        Err(_) => println!("Verification failed"),
    }

    Ok(())
}
