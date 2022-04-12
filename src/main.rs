use clap::{arg, Command};
use chrono::offset::Utc;
use chrono::DateTime;
use std::time::SystemTime;

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
                    .arg(arg!(--XXX).help("Triggers the key generation phase. Requires the -p passphrase argument and Config mode."))
                    .arg(arg!(-x --passphrase <passphrase>).help("Passphrase to trigger keygen"))
        )
        .subcommand(
            Command::new("display")
                    .about("Displays the relevant information like socket address, Context, group key, parameters, etc.")
                    .arg(arg!(-a --sockaddrs).help("Displays the various socket addresses of the servers."))
                    .arg(arg!(-c --context).help("Displays the Context string"))
                    .arg(arg!(-g --groupkey).help("Displays the public group key"))
                    .arg(arg!(-k --serverkey).help("Displays the server's public key. Use with -s flag to supply the socket address"))
                    .arg(arg!(-s --servaddr <addr>).help("The server address to send the request to").required(false))
                    .arg(arg!(-p --params).help("Displays the number of signers and the minimum threshold required to sign."))/* */
                    .arg(arg!(-t --timeout).help("Displays the current timestamp in the format used for signing."))/* */
        )
        .subcommand(
            Command::new("input")
                    .about("Input operations like signing, validation, etc.")
                    .arg(arg!(-m --msg <msg>).help("Input the message that needs to be hashed. Input will be parsed as a string."))
                    .arg(arg!(-s --server <server>).help("Enter the socketaddr of the server that you want to visit."))
                    .arg(arg!(-v --verify <signfile>).help("Enter the json file for the signature. Should have 2 more arguments: -t timestamp, -m message"))
                    .arg(arg!(-t --timein <timein>).help("Enter the time stamp for signature verification."))
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

                    println!("{:?}", body);
                }
                else if sub_matches.is_present("params"){
                    println!("Signers: 3\nThreshold: 2");
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
            _ => {
                println!("TODO");
                Ok(())
            }
        }
        
                    
}
