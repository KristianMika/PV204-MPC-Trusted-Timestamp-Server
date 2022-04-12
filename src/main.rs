use clap::{arg, Command};
use chrono::offset::Utc;
use chrono::DateTime;
use std::time::SystemTime;
use serde::{Serialize, Deserialize};
use frost_dalek::signature::ThresholdSignature;
// use std::process;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let matches = Command::new("DiKS TiTS Client")
        .version("0.1.0")
        .author("Kristian Mika\nDavid Rypar\nSuyash Shandilya")
        .about("End user CL App for DiKS TiTS")
        .propagate_version(false)
        .subcommand_required(true)
        .subcommand(
            Command::new("config")
                    .about("Configuration mode. Authentication required (for some commands)")
                    .arg(arg!(-i --keygen).help("Triggers the key generation phase. Requires the -p passphrase argument and Config mode."))
                    .arg(arg!(-p --passphrase <passphrase>).help("Passphrase to trigger keygen.
                                            JUST A DEMO TO SHOW THAT THESE OPERATIONS ARE PERFORMED ONLY BY A TRUSTED ADMIN.
                                            These api's and corresponding endpoints itself can be assumed to be inaccessible to attacker,
                                            as they are only to be used during setup or teardown of the server."))
                    .arg(arg!(-r --reset).help("Reset the server"))
        )
        .subcommand(
            Command::new("display")
                    .about("Displays the relevant information like socket address, Context, group key, parameters, etc.")
                    // .arg(arg!(-a --sockaddrs).help("Displays the various socket addresses of the servers."))
                    .arg(arg!(-c --context).help("Displays the Context string"))/* */
                    .arg(arg!(-g --groupkey).help("Displays the public group key"))/* */
                    .arg(arg!(-k --serverkey).help("Displays the server's public key. Use with -s flag to supply the socket address"))/*text -> json */
                    .arg(arg!(-s --servaddr <addr>).help("The server address to send the request to").required(false))/* */
                    .arg(arg!(-p --params).help("Displays the number of signers and the minimum threshold required to sign."))/* */
                    .arg(arg!(-t --timeout).help("Displays the current timestamp in the format used for signing."))/* */
        )
        .subcommand(
            Command::new("input")
                    .about("Input operations like signing, validation, etc.")
                    .arg(arg!(-m --msg <msg>).help("Input the message that needs to be hashed. Input will be parsed as a string."))
                    .arg(arg!(-a --server <server>).help("Enter the socketaddr of the server that you want to visit.").required(false))
                    .arg(arg!(-v --verify).help("Verify a signature. Should have 2 more arguments: -t timestamp, -m message").required(false))
                    .arg(arg!(-s --sign).help("Sign the msg. Should have 2 more arguments: -t timestamp, -m message").required(false))
                    .arg(arg!(-t --timein <timein>).help("Enter the time stamp for signature verification.").required(false))
                    .arg(arg!(-n --token <token>).help("Hex token of the signature.").required(false))
        )
        .get_matches();

        match matches.subcommand() {
            Some(("display", sub_matches)) => {
                if sub_matches.is_present("timeout") {
                    let timenow = SystemTime::now();
                    let datetime: DateTime<Utc> = timenow.into();
                    let timestr = datetime.format("%Y%m%d%H%M%SZ").to_string();
                    println!("RFC 3161 compliant timestamp: {}",timestr);
                } else if sub_matches.is_present("groupkey") {
                    let body = reqwest::get("http://127.0.0.1:8081/groupkey")
                                .await?
                                .text()
                                .await?;

                    println!("Raw response:\n{:?}", body);

                    let kgb: GroupKey = body.json().unwrap();
                    dbg!(kgb);
                }
                else if sub_matches.is_present("params"){
                    println!("Signers: 3\nThreshold: 2");
                }
                else if sub_matches.is_present("context"){
                    println!("diks-tits");
                }
                else if sub_matches.is_present("serverkey") {
                    if sub_matches.is_present("servaddr") {
                        // println!("serverkey flag found");
                        let body = reqwest::get( format!( "{}/pubkey",sub_matches.value_of("servaddr").unwrap() ) )
                                    .await?
                                    .text()
                                    .await?;
                        println!("{:?}", body);
                    }
                }

                Ok(())
            },

            Some(("config", sub_matches)) =>{
                
                if sub_matches.is_present("passphrase"){
                    if sub_matches.value_of("passphrase").unwrap() == "security100" {
                        println!("You are an admin. You can't be an attacker.");
                        
                        if sub_matches.is_present("keygen") {
                            let client = reqwest::Client::new();
                            let res = client.post("http://127.0.0.1:8080/keygen")
                                .body("")
                                .send()
                                .await?;
                            println!("{:?}",res);
                        }

                        if sub_matches.is_present("reset") {
                            let client = reqwest::Client::new();
                            let res = client.post("http://127.0.0.1:8080/reset")
                                .body("")
                                .send()
                                .await?;
                            println!("{:?}",res);
                        }
                    } else {
                        println!(" Password is: security100. Try again.");
                    }
                } else {
                    println!("Send -p as security100. Try again.");
                    return Ok(());
                };
                Ok(())
                
            }

            Some(("input", sub_matches)) => {

                if sub_matches.is_present("sign") {
                    #[derive(Serialize)]
                    struct TimestampStruct {
                        hashAlgorithm: String,
                        hashedMessage: String,
                    }
                    #[derive(Deserialize)]
                    pub struct TimeStampResp {
                        status: String, // for now, later PKIStatusInfo. 🤣
                        timeStampToken: Vec<u8>, // TODO: read the RFC 3161 🤣
                    }

                    let body = TimestampStruct{ hashAlgorithm: "SHA2".to_string(), hashedMessage: sub_matches.value_of("msg").unwrap().to_string() };
                    let kgb = serde_json::to_string(&body).unwrap();
                    let client = reqwest::Client::new();
                    let res = client.post("http://127.0.0.1:8080/timestamp")
                        .json(&kgb)
                        .send()
                        .await?;
                    println!("{:?}",res);
                    
                    let signtoken : TimeStampResp = res.json().unwrap();
                    if signtoken.status = "Ok?" {
                        println!(hex::encode(signtoken.timeStampToken));
                    } else {
                        println!("Sign wasn't successfull.");
                    }

                } else if sub_matches.is_present("verify"){
                    if sub_matches.is_present.("timein") {
                        if sub_matches.is_present("token"){
                            match hex::decode(sub_matches.value_of("token").unwrap()){
                                Ok(v) => {
                                    let slice = v.as_slice();
                                    let array: [u8; 64] = match slice.try_into() {
                                        Ok(ba) => ba,
                                        Err(_) => panic!("Expected a Vec of length {} but it was {}", 32, v.len()),
                                    };
                                    match ThresholdSignature::from_bytes(array) {
                                        Ok(u) => {
                                            println!("Fetching the group key");

                                            // u.verify();
                                        },
                                        Err(_) => println!("Invalid group signature");
                                    },
                                },
                                Err(_) => println!("Invalid hex string.")
                            }
                        } else {
                            println!("Please enter the hex signed token")
                        }
                    } else {
                        println!("Time stamp not provided")
                    }
                }else {
                    println!("Invalid input in input mode.");
                }
                Ok(())
                
            }

            _ => {
                println!("invalid mode of operation");
                Ok(())
            }
        }
        
                    
}
