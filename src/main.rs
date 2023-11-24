//zector_detector_pub_zcashd Block Explorer
//https://github.com/autotunafish/zector_detector_pub_zcashd
//
//This tool calls a getblock.io public Zcash node for tx and block data.
//The format is: The node IP endpoint address and API key.
//Example: https://zec.getblock.io/e7a92777-136d-4d0b-9eea-1ad3aea31b37/mainnet
//                                               /\
//The API key is the included code string here:  |
//Example:
//e7a92777-136d-4d0b-9eea-1ad3aea31b37
//
//The public Zcash node of this website truncates certain data from the calls.
//This is a best-effort project but provides a large amount of information.
//The full raw json of all the contained information is available at every instance.
//
//The generalized main function starts 1 of 3 scripts in the background running inotifywait.
//User input data here (hash's or numbers),
//are written to a curl command in a file being watched by one of the background inotifywait scripts.
//A rust instance of inotify is started here and watches a '3rd file'.
//The background inotifywait script copies the command to a '2nd file' and executes
//with a pipe option to receive the returned data into said '3rd file'.
//When the data is received, it is read into a String and parsed iteratively.
//Useful data is matched upon and printed.

use chrono::{TimeZone, Utc};
use execute::shell;
use inotify::{Inotify, WatchMask};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Stdio;
use std::{fs, io, thread, time};

fn main() {
    //Get the canonicalized file paths to the needed files for the commands as a PathBuf.
    let acdir = PathBuf::from("./action2.txt")
        .into_os_string()
        .into_string()
        .unwrap();
    let abdir = PathBuf::from("./action5.txt")
        .into_os_string()
        .into_string()
        .unwrap();
    let chdir = PathBuf::from("./chaininfo2.txt")
        .into_os_string()
        .into_string()
        .unwrap();

    //Convert them to usable Strings.
    let acdir1 = fs::canonicalize(&acdir)
        .expect("No Such File")
        .display()
        .to_string();
    let abdir1 = fs::canonicalize(&abdir)
        .expect("No Such File")
        .display()
        .to_string();
    let chdir1 = fs::canonicalize(&chdir)
        .expect("No Such File")
        .display()
        .to_string();

    //The user is prompted to enter their node endpoint info.
    println!("\x1b[48;5;232m\x1b[38;5;191m* * * * * * * * * * * * * * * * * * * * * * * *\x1b[0m");
    println!("\x1b[48;5;232m\x1b[38;5;154m* Enter Your Public Node IP Endpoint Address  *\x1b[0m");
    println!("\x1b[48;5;232m\x1b[38;5;155m* Example: https://zec.getblock.io/e7a92777-136d-4d0b-9eea-1ad3aea31b37/mainnet *\x1b[0m");

    //This file stores the node IP address and is assessed for prior usage.
    let tokfile = String::from("tokfile.txt");
    let mut pb_ip = fs::read_to_string(&tokfile).unwrap();

    //The IP, if any, is cloned and any line breaks removed.
    //Required for proper formatting of the terminal output.
    let pbbinding = pb_ip.clone();
    let pb_ip1 = pbbinding.trim();

    //If the clone of the file is not empty then the user is prompted to
    //confirm the contents, or input a new address.
    if pb_ip1.len() != 0 {
        println!(
            "\x1b[48;5;232m\x1b[38;5;157m* Or Hit Enter To Use Previous Input Recorded *\x1b[0m"
        );
        println!("\x1b[48;5;232m\x1b[38;5;159m* {} *\x1b[0m", &pb_ip);

        //Create a variable for the input.
        let mut usrinput = String::new();

        //Get the User input.
        io::stdin().read_line(&mut usrinput).unwrap();

        //Pop the newline \n
        usrinput.pop();

        // !! If the user input ANYTHING, it is written to the tokfile.        !!
        // !! THIS IS NOT CHECKED FOR PROPER FORMAT for compatibility reasons. !!
        // !! AND IS WHY THE USER ALWAYS CONFIRMS THE CONTENTS ABOVE.          !!
        if usrinput.len() >= 1 {
            //User input is now the IP address.
            pb_ip = usrinput.clone();
            //Create file and write out.
            let mut pib_1 = File::create(&tokfile).expect("no");
            pib_1.write_all(&usrinput.as_bytes()).expect("no");
        }
    }

    //If the file is empty (First Run)
    if pb_ip1.len() == 0 {
        println!("\x1b[48;5;232m\x1b[38;5;157m* No Previous Input Address Recorded *\x1b[0m");

        //Create a variable for the input
        let mut usrinput = String::new();

        //Get the User input.
        io::stdin().read_line(&mut usrinput).unwrap();

        //Pop the newline \n
        usrinput.pop();

        //User input is now the IP address
        pb_ip = usrinput.clone();

        //Create file and write out.
        let mut pib_1 = File::create(&tokfile).expect("no");
        pib_1.write_all(&usrinput.as_bytes()).expect("no");
    }

    println!("\x1b[48;5;232m\x1b[38;5;191m* * * * * * * * * * * * * * * * *\x1b[0m");
    println!("\x1b[48;5;232m\x1b[38;5;154m* Enter Your Public Node IP API *\x1b[0m");
    println!("\x1b[48;5;232m\x1b[38;5;155m* Tip: It's In The IP Address * *\x1b[0m");
    print!("\x1b[48;5;232m\x1b[38;5;156m* Example: https://zec.getblock.io/\x1b[0m");
    print!("\x1b[48;5;47m\x1b[38;5;198me7a92777-136d-4d0b-9eea-1ad3aea31b37\x1b[0m");
    println!("\x1b[48;5;232m\x1b[38;5;156m/mainnet *\x1b[0m");

    //Begin a similar process for the API key.
    //This file stores the node API key and is assessed for prior usage.
    let tikfile = String::from("tikfile.txt");
    let mut pb_api = fs::read_to_string(&tikfile).unwrap();

    //The API Key, if any, is cloned and any line breaks removed.
    //Required for proper formatting of the terminal output.
    let apbinding = pb_api.clone();
    let pb_api1 = apbinding.trim();

    //If the clone of the file is not empty then the user is prompted to
    //confirm the contents, or input a new API Key.
    if pb_api1.len() != 0 {
        println!(
            "\x1b[48;5;232m\x1b[38;5;157m* Or Hit Enter To Use Previous Input Recorded *\x1b[0m"
        );
        println!("\x1b[48;5;232m\x1b[38;5;159m* {} *\x1b[0m", &pb_api);

        //Create a variable for the input
        let mut usrinput = String::new();

        //Get the User input.
        io::stdin().read_line(&mut usrinput).unwrap();

        //Pop the newline \n
        usrinput.pop();

        // !! If the user input ANYTHING, it is written to the tikfile.        !!
        // !! THIS IS NOT CHECKED FOR PROPER FORMAT for compatibility reasons. !!
        // !! AND IS WHY THE USER ALWAYS CONFIRMS THE CONTENTS ABOVE.          !!
        if usrinput.len() >= 1 {
            //User input is now the API key.
            pb_api = usrinput.clone();

            //Create file and write out.
            let mut pib_1 = File::create(&tikfile).expect("no");
            pib_1.write_all(&usrinput.as_bytes()).expect("no");
        }
    }

    //If the file is empty (First Run), print out some info and prompt for input.
    if pb_api1.len() == 0 {
        println!("\x1b[48;5;232m\x1b[38;5;157m* No Previous Input API Recorded *\x1b[0m");

        //Create a variable for the input
        let mut usrinput = String::new();

        //Get the User input.
        io::stdin().read_line(&mut usrinput).unwrap();

        //Pop the newline \n
        usrinput.pop();

        //User input is now the IP address
        pb_api = usrinput.clone();

        let mut pib_1 = File::create(&tikfile).expect("no");
        pib_1.write_all(&usrinput.as_bytes()).expect("no");
    }

    //Create the command strings.
    //These are the tails or close to the tail of the commands.
    //The shorter, second two contain params options and so are finished during their fn call.
    let getblockchaininfo = "\"getblockchaininfo\",\"params\": []}' | json_pp > ";

    let getrawtransaction = "\"getrawtransaction\",\"params\": [\"";

    let getblock = "\"getblock\",\"params\": [\"";

    /////////////////////////////////////////////////////////////
    //getinfo is slated for getinfo but does not seem worth doing
    //let getinfo = "\"getinfo\",\"params\": []}' | json_pp > ";

    //The leading portion of the command.
    let mut chaininfo = String::from("curl --location --request POST '");

    //Goes between the IP and API key in the command string.
    let chaininfo1 = "' --header '";

    //Goes after the API key.
    let chaininfo2 = ";' --header 'Content-Type: application/json' --data-raw '{
			\"jsonrpc\": \"2.0\",
			\"id\": \"getblock.io\",
			\"method\": ";

    //Construct the entire leading portion of the command string by pushing the str's onto the leading String.
    chaininfo.push_str(&pb_ip);
    chaininfo.push_str(&chaininfo1);
    chaininfo.push_str(&pb_api);
    chaininfo.push_str(&chaininfo2);
    //
    //curl --location --request POST 'https://zec.getblock.io/e7a92777-136d-4d0b-9eea-1ad3aea31b37/mainnet' --header '7a92777-136d-4d0b-9eea-1ad3aea31b37;' --header 'Content-Type: application/json' --data-raw '{\"jsonrpc\": \"2.0\",\"id\": \"healthcheck\",\"method\":
    //

    //The main program loop.
    loop {
        //Create the specific commands for getblockchaininfo, getrawtransaction and getblock.
        //Clone the leading portion above x3
        let mut gtbcinf = chaininfo.clone();
        let mut gtrwtxin = chaininfo.clone();
        let mut gtblck = chaininfo.clone();

        //gtinf is slated for getinfo but does not seem worth doing
        //let mut gtinf = chaininfo.clone();
        //gtinf.push_str(&getinfo);

        //Push the specific commands into the respective String.
        gtbcinf.push_str(&getblockchaininfo);
        gtrwtxin.push_str(&getrawtransaction);
        gtblck.push_str(&getblock);

        //This defines a sleep function necessary for the outside inotifywait scripts to start.
        //This is actually 100ms.
        let ten_millis = time::Duration::from_millis(100);
        //The best alternative for performance is manually starting and stopping the scripts.
        //However, automatic constant background runtime from code start results in zombies.
        //So the current "start, use once and exit, repeat" method was chosen, this is ideal.
        //This behavior is obtained in the scripts which exit after a single event.

        //These are variables for the getrawtransaction input/output/shieldedoutput/action data, as best as I could derive it.
        let mut innies = 0;
        let mut outties = 0;
        let mut orchies = 0;
        let mut shoutties = 0;

        //This value dictates whether the second command getblock is run.
        //getrawtransaction is first trial run. If success, return.
        //Else, try getblock.
        let mut erval = 0;

        //Initial prompt for general User input.
        //This will either
        //A: fail if input is bad, in which case the loop just continues and we return here.
        //B: Empty input will call getblockchaininfo and display a summary of info, ask to display raw json, return here.
        //C: Run one or both commands on the Hash or Height. This will yield one error and one good result.
        //The Error is skipped and the good result is matched upon and info printed, return here.

        println!("\x1b[48;5;232m\x1b[38;5;191m* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\x1b[0m");
        println!("\x1b[48;5;232m\x1b[38;5;154m* Hit Enter To Refresh Info, Input TX Hash Or Block Hash/Height *\x1b[0m");

        //Create a variable for the input
        let mut usrinput = String::new();

        //Get the User input.
        io::stdin().read_line(&mut usrinput).unwrap();

        //////////////////////////////////////
        //This is the getblockchaininfo call//
        //////////////////////////////////////

        //If the input is 'None', then the general getblockchaininfo data will be refreshed.
        //Improper input will return a bad curl command and improper data, skip it!
        //This 'if' statement checks for any length.
        if usrinput.len() <= 1 {
            //This script runs getblockchaininfo commands.
            //Pipe the output of the cmd below to collection.txt
            let mut ccommand = shell("./chain_notify > collection.txt 2>&1");

            //Create and spawn the new thread process.
            ccommand.stdout(Stdio::piped());
            let mut _output3 = ccommand.spawn();

            //Define 'now' to properly count the sleep.
            let _now = time::Instant::now();
            //Sleep 100ms.
            thread::sleep(ten_millis);
            //Zzzzzzzz

            //This is a str for a filename that will accept the received data from the curl request response.
            let ch2 = "chaininfo2.txt";

            //Push the canonicalized file path string to the command.
            gtbcinf.push_str(&chdir1);

            //This bracket seperates the file write from the 'event' below and seems necessary.
            {
                //This is a filename that will receive the curl command and then copy to '2nd file'.
                let ci0 = "chaininfo.txt";

                //This creates the file (perhaps unnecessary) and writes the curl cmd as the contents.
                let mut var_1 = File::create(&ci0).expect("no");
                var_1.write_all(&gtbcinf.as_bytes()).expect("no");
                //This write_out (above) triggers the chain_notify script and runs the curl command.

                //This bracket breaks out of the above function.
                //Technically this is a datarace but the distance from here to the next inotify event is very short.
            }

            //This creates an inotify instance add_watch on the filename that receives the curl response.
            let mut inotify = Inotify::init().expect("Error while initializing inotify instance");
            inotify
                .add_watch(&ch2, WatchMask::CLOSE_WRITE)
                .expect("Failed to add file watch");

            //Create a buffer for the read event.
            let mut buffer = [0; 512];

            //Define the read_events_blocking instance.
            let events = inotify
                .read_events_blocking(&mut buffer)
                .expect("Error while reading events");

            //When the curl response is received it will trigger the 'event'.
            //The filename's contents are read into a string.
            #[allow(unused_variables)]
            for event in events {
                let k = fs::read_to_string(&ch2).unwrap();

                //The string is split at all whitespaces.
                //It passes into an iterative match and the useful data is parsed and printed.
                let mut klines = k.split_whitespace();
                loop {
                    //Get first line, if any, and all other lines through the iterator.
                    let kv = klines.next().clone();

                    //Perform assert EQ for 'Some' line.
                    if kv != None {
                        //IF YES - Remove the 'Result' wrapper.
                        //IF NO, then break out of the loop (below).
                        let kvc = kv.unwrap();

                        //Match the data to a common json char or a desired header.
                        //Either skip or print the data.
                        //Most will print the matched item and the following two items as given by .next(), .next().
                        match kvc {
                            "\"initial_block_download_complete\"" => {
                                println!(
                                    "\x1b[48;5;232m\x1b[38;5;141m{}{}{}\x1b[0m",
                                    &kvc,
                                    &klines.next().unwrap(),
                                    &klines.next().unwrap()
                                )
                            }
                            "\"consensus\"" => {
                                let errwast = klines.next().unwrap();
                                let errwasta = klines.next().unwrap();
                                println!(
                                    "\x1b[48;5;232m\x1b[38;5;10m{}{}{}{}{}{}\x1b[0m",
                                    &klines.next().unwrap(),
                                    &klines.next().unwrap(),
                                    &klines.next().unwrap(),
                                    &klines.next().unwrap(),
                                    &klines.next().unwrap(),
                                    &klines.next().unwrap()
                                )
                            }
                            "\"chainwork\"" | "\"blocks\"" | "\"size_on_disk\""
                            | "\"bestblockhash\"" => {
                                println!(
                                    "\x1b[48;5;232m\x1b[38;5;228m{}{}{}\x1b[0m",
                                    &kvc,
                                    &klines.next().unwrap(),
                                    &klines.next().unwrap()
                                )
                            }
                            "\"chainValue\"" => {
                                println!();
                                print!(
                                    "\x1b[48;5;232m\x1b[38;5;159m{}{}{}\x1b[0m",
                                    &kvc,
                                    &klines.next().unwrap(),
                                    &klines.next().unwrap()
                                )
                            }
                            "\"activationheight\"" => {
                                print!(
                                    "\x1b[48;5;232m\x1b[38;5;162m{}{}{}\x1b[0m",
                                    &kvc,
                                    &klines.next().unwrap(),
                                    &klines.next().unwrap()
                                )
                            }
                            "\"name\"" | "\"difficulty\"" => {
                                println!(
                                    "\x1b[48;5;232m\x1b[38;5;162m{}{}{}\x1b[0m",
                                    &kvc,
                                    &klines.next().unwrap(),
                                    &klines.next().unwrap()
                                )
                            }
                            "\"sapling\"," | "\"orchard\"," | "\"sprout\"," => {
                                println!("\x1b[48;5;232m\x1b[38;5;159m\"pool\":{}\x1b[0m", &kvc);
                            }
                            _ => {} //End of match kvc.
                        }

                        //Returns to inner loop to check for more line data to parse.
                        continue;
                    }

                    //If the assert EQ fails and '== None', break.
                    break;
                }

                //Prompt for the raw json output.
                println!();
                println!("\x1b[48;5;232m\x1b[38;5;220m* * * * * * * * * * * * * * * * * * * * * * * *\x1b[0m");
                println!("\x1b[48;5;232m\x1b[38;5;220m* Hit Enter To Return, Input 'R' For Raw Json *\x1b[0m");

                //Create a string for the user input.
                let mut usrinput = String::new();

                //Get the User input, match on 'R'
                io::stdin().read_line(&mut usrinput).unwrap();
                let r = "R\n";
                if usrinput.as_str() == r {
                    //Print the raw json.
                    println!("{}", &k);
                }
                //End of 'event'
            }

            //End of length check 'if' statement, return to the main loop.
            continue;
        }

        ///////////////////////////////////////////////////////////////////////////
        ///////////////////////////////////////////////////////////////////////////

        //If the User input is not 'None' then run that data with the getrawtransaction and getblock calls.

        if usrinput.len() >= 2 {
            //Pop the newline of the input.
            usrinput.pop();
            println!();

            //Create filenames for the returned data.
            let resend2 = String::from("action2.txt");
            let resend3 = String::from("action5.txt");

            //Clone those filename Strings.
            //This is 'action2'
            let sp4 = resend2.clone();
            //This is 'action5'
            let sb4 = resend3.clone();

            //Create the tail of the command which pipes the data into 'action2' or 'action5'.
            let resend1 = String::from("\", 1]}' |json_pp > ");

            ///////////////////////////////////////
            //This is the getrawtransaction call.//
            ///////////////////////////////////////
            {
                //This script runs getrawtransaction commands.
                //Pipe all the output of the cmd below to collection.txt
                let mut acommand = shell("./action_notify > collection.txt 2>&1");
                acommand.stdout(Stdio::piped());

                //Create and spawn the new thread process.
                let _output1 = acommand.spawn();

                //Define 'now' to properly count the sleep time.
                let _now = time::Instant::now();
                //Sleep 100ms.
                thread::sleep(ten_millis);
                //Zzzzzzzz

                //Clone the input and tailing portions of the command.
                let sp2 = usrinput.clone();
                let sp3 = resend1.clone();

                //Push them onto the leading portion of the cmd.
                //the user input,
                gtrwtxin.push_str(&sp2);

                //the tail of the command,
                gtrwtxin.push_str(&sp3);

                //and the canonicalized filepath for the recieved curl data to be piped to.
                gtrwtxin.push_str(&acdir1);

                //Create a filename to contain the command just written.
                let action = "action.txt";

                //Create that named file and write the command into it.
                let mut var_1 = File::create(&action).expect("no");
                var_1.write_all(&gtrwtxin.as_bytes()).expect("no");
            }

            //This creates an inotify instance add_watch on the filename that receives the curl response.
            let mut inotify = Inotify::init().expect("Error while initializing inotify instance");
            inotify
                .add_watch(&sp4, WatchMask::CLOSE_WRITE)
                .expect("Failed to add file watch");

            //Create a buffer for the read event.
            let mut buffer = [0; 512];

            //Define the read_events_blocking instance.
            let events = inotify
                .read_events_blocking(&mut buffer)
                .expect("Error while reading events");

            //When the curl response is received it will trigger the 'event'.
            //The filename's contents are read into a string.
            #[allow(unused_variables)]
            for event in events {
                let k = fs::read_to_string(&sp4).unwrap();

                //The string is split at all whitespaces.
                //It passes into an iterative match and the useful data is parsed and printed.
                let mut klines = k.split_whitespace();
                loop {
                    //Get first line, if any, and all other lines through the iterator.
                    let kv = klines.next().clone();

                    //Perform assert EQ for 'Some' line.
                    if kv != None {
                        //IF YES - Remove the 'Result' wrapper.
                        //IF NO, then break out of the loop.
                        let kvc = kv.unwrap();

                        //Match the data to a common json char or a desired header.
                        //Either skip or print the data.
                        match kvc {
                            "\"time\"" => {
                                //This retrieves the Unix Timestamp and converts into normal time.
                                let timewast = klines.next();
                                let timeget = klines.next();
                                if timeget != None {
                                    let mut gottime = timeget.clone().expect("").to_string();
                                    gottime.pop();
                                    let goime = gottime.parse().expect("");

                                    #[allow(deprecated)]
                                    let dt = Utc.timestamp(goime, 0);
                                    println!();
                                    println!("\x1b[48;5;232m\x1b[38;5;50m{}{}\x1b[0m", &kvc, &dt);
                                    println!();
                                }
                            }

                            "\"error\"" => {
                                //If the call failed it will return the header "code", which is matched on.
                                //This sets erval high and will allow passage beyond the check (below).
                                //If this call does not fail then it will simply prevent the next call from occuring.
                                let errwast = klines.next().unwrap();
                                let errwast2 = klines.next().unwrap();
                                let errchk = klines.next().unwrap();
                                if errchk == "\"code\"" {
                                    erval += 1;

                                    break;
                                }
                            }
                            "\"chainValue\"" | "\"difficulty\"" => {
                                println!();
                                print!(
                                    "\x1b[48;5;232m\x1b[38;5;159m{}{}{}\x1b[0m",
                                    &kvc,
                                    &klines.next().unwrap(),
                                    &klines.next().unwrap()
                                )
                            }
                            "\"activationheight\""
                            | "\"confirmations\""
                            | "\"hash\""
                            | "\"nextblockhash\""
                            | "\"previousblockhash\"" => {
                                println!(
                                    "\x1b[48;5;232m\x1b[38;5;228m{}{}{}\x1b[0m",
                                    &kvc,
                                    &klines.next().unwrap(),
                                    &klines.next().unwrap()
                                )
                            }
                            "\"name\"" | "\"height\"" | "\"version\"" | "\"expiryheiht\""
                            | "\"anchor\"" | "\"overwintered\"" => {
                                println!(
                                    "\x1b[48;5;232m\x1b[38;5;156m{}{}{}\x1b[0m",
                                    &kvc,
                                    &klines.next().unwrap(),
                                    &klines.next().unwrap()
                                )
                            }
                            "\"valueBalance\"" | "\"value\"" | "\"blockhash\"" | "\"vpub_old\""
                            | "\"vpub_new\"" => {
                                println!(
                                    "\x1b[48;5;232m\x1b[38;5;169m{}{}{}\x1b[0m",
                                    &kvc,
                                    &klines.next().unwrap(),
                                    &klines.next().unwrap()
                                );
                            }
                            "\"addresses\"" => {
                                print!(
                                    "\x1b[48;5;232m\x1b[38;5;224m{}{}\x1b[0m",
                                    &kvc,
                                    &klines.next().unwrap()
                                );
                                let errwast = klines.next().unwrap();
                                print!(
                                    "\x1b[48;5;232m\x1b[38;5;224m{}\x1b[0m",
                                    &klines.next().unwrap()
                                );
                            }
                            "\"txid\"" => {
                                println!(
                                    "\x1b[48;5;232m\x1b[38;5;231m{}{}{}\x1b[0m",
                                    &kvc,
                                    &klines.next().unwrap(),
                                    &klines.next().unwrap()
                                );
                            }
                            "\"vShieldedOutput\"" => loop {
                                let jlines = klines.next().unwrap();
                                if jlines == "\"cmu\"" {
                                    shoutties += 1;
                                    continue;
                                }
                                if jlines == "\"vShieldedSpend\"" {
                                    break;
                                }
                                continue;
                            },

                            "\"sapling\"," | "\"orchard\"," | "\"sprout\"," => {
                                println!("\x1b[48;5;232m\x1b[38;5;159m\"pool\":{}\x1b[0m", &kvc)
                            }
                            "\"transparent\"" => {
                                println!(
                                    "\x1b[48;5;232m\x1b[38;5;160m\"pool\":{} Not Monitored\x1b[0m",
                                    &kvc
                                );
                                println!();
                            }
                            "\"coinbase\"" => {
                                println!("\x1b[48;5;232m\x1b[38;5;210m**{}**\x1b[0m", &kvc);
                            }
                            "\"scriptSig\"" | "\"spendAuthSig\"" => {
                                innies += 1;
                            }
                            "\"n\"" => {
                                outties += 1;
                                print!(
                                    "\x1b[48;5;232m\x1b[38;5;159m{}{}{}\x1b[0m",
                                    &kvc,
                                    &klines.next().unwrap(),
                                    &klines.next().unwrap()
                                );
                            }
                            "\"cmx\"" => {
                                orchies += 1;
                            }
                            _ => {} //End of match kvc.
                        }

                        //Returns to inner loop to check for more line data to parse.
                        continue;
                    }

                    //If the assert EQ fails and '== None', break.
                    break;
                }

                //If the call succeded.
                if erval == 0 {
                    //This prints the input and output data matched upon above.
                    println!("\x1b[48;5;232m\x1b[38;5;157m\"All Inputs\":{} \"Public Outputs\":{}\x1b[0m", &innies, &outties);
                    println!("\x1b[48;5;232m\x1b[38;5;157m\"Shielded Outputs\":{} \"Actions\":{} \x1b[0m", &shoutties, &orchies);

                    innies = 0;
                    outties = 0;
                    orchies = 0;
                    shoutties = 0;

                    println!();
                    println!("\x1b[48;5;232m\x1b[38;5;220m* * * * * * * * * * * * * * * * * * * * * * * *\x1b[0m");
                    println!("\x1b[48;5;232m\x1b[38;5;220m* Hit Enter To Return, Input 'R' For Raw Json *\x1b[0m");

                    let mut usrinput = String::new();

                    //Get the User input.
                    io::stdin().read_line(&mut usrinput).unwrap();
                    let r = "R\n";

                    if usrinput.as_str() == r {
                        println!("{}", &k);
                    }
                    //End of if erval == 0 check.
                }

                //End of 'event'
            }

            //If the call succeded, return to the loop.
            if erval == 0 {
                continue;
            };

            // println!("past");

            //////////////////////////////
            //This is the getblock call.//
            //////////////////////////////
            {
                //This script runs getblock commands.
                //Pipe all the output of the cmd below to collection.txt
                let mut bcommand = shell("./action_notify_b > collection.txt 2>&1");
                bcommand.stdout(Stdio::piped());

                //Create and spawn the new thread process.
                let mut _output2 = bcommand.spawn();

                //Define 'now' to properly count the sleep time.
                let _now = time::Instant::now();
                thread::sleep(ten_millis);

                //Clone the user input.
                let sb2 = usrinput.clone();

                //Clone the tail of the command which pipes the data into 'action5'.
                let sb3 = resend1.clone();

                //Push them both onto the leading portion of the cmd.
                //the user input
                gtblck.push_str(&sb2);

                //the tail of the command
                gtblck.push_str(&sb3);

                //and the canonicalized filepath for the recieved curl data to be piped to.
                gtblck.push_str(&abdir1);

                //Create a file name str.
                let action = "action3.txt";

                //Create that named file and write the command into it.
                let mut var_1 = File::create(&action).expect("no");
                var_1.write_all(&gtblck.as_bytes()).expect("no");
            }

            //This creates an inotify instance add_watch on the filename that receives the curl response.
            let mut inotify = Inotify::init().expect("Error while initializing inotify instance");
            inotify
                .add_watch(&sb4, WatchMask::CLOSE_WRITE)
                .expect("Failed to add file watch");

            //Create a buffer for the read event.
            let mut buffer = [0; 512];

            //Define the read_events_blocking instance.
            let events = inotify
                .read_events_blocking(&mut buffer)
                .expect("Error while reading events");

            //When the curl response is received it will trigger the 'event'.
            //The filename's contents are read into a string.
            #[allow(unused_variables)]
            for event in events {
                let k = fs::read_to_string(&sb4).unwrap();

                //Redundant, can probably be removed.
                let mut erval = 0;

                //The string is split at all whitespaces (perhaps unnecessary).
                //It passes into an iterative match and the useful data is parsed and printed.
                let mut klines = k.split_whitespace();
                loop {
                    //Get first line, if any.
                    let kv = klines.next().clone();

                    //Perform assert EQ for 'Some' line.
                    if kv != None {
                        //IF YES - Remove the 'Result' wrapper.
                        //IF NO, then break out of the loop.
                        let kvc = kv.unwrap();

                        //Match the data to a common json char or a desired header.
                        //Either skip or print the data.
                        match kvc {
                            "\"time\"" => {
                                //This retrieves the Unix Timestamp and converts into normal time.
                                let timewast = klines.next();
                                let timeget = klines.next();
                                if timeget != None {
                                    let mut gottime = timeget.clone().expect("").to_string();
                                    gottime.pop();
                                    let goime = gottime.parse().expect("");

                                    #[allow(deprecated)]
                                    let dt = Utc.timestamp(goime, 0);
                                    println!();
                                    println!("\x1b[48;5;232m\x1b[38;5;50m{}{}\x1b[0m", &kvc, &dt);
                                    println!();
                                }
                            }
                            "\"error\"" => {
                                //Because if the linearity of the program, this match is currently redundant.
                                //It is of no consequence and should remain in case of future use cases or operational bugs.
                                let errwast = klines.next().unwrap();
                                let errwast2 = klines.next().unwrap();
                                let errchk = klines.next().unwrap();

                                if errchk == "\"code\"" {
                                    erval += 1;

                                    break;
                                }
                            }
                            "\"difficulty\"" | "\"merkleroot\"" | "\"nonce\"" | "\"size\""
                            | "\"version\"" | "\"blocks\"" => {
                                println!(
                                    "\x1b[48;5;232m\x1b[38;5;10m{}{}{}\x1b[0m",
                                    &kvc,
                                    &klines.next().unwrap(),
                                    &klines.next().unwrap()
                                )
                            }
                            "\"chainValue\"" => {
                                println!();
                                print!(
                                    "\x1b[48;5;232m\x1b[38;5;159m{}{}{}\x1b[0m",
                                    &kvc,
                                    &klines.next().unwrap(),
                                    &klines.next().unwrap()
                                )
                            }
                            "\"activationheight\""
                            | "\"confirmations\""
                            | "\"hash\""
                            | "\"nextblockhash\""
                            | "\"previousblockhash\"" => {
                                println!(
                                    "\x1b[48;5;232m\x1b[38;5;228m{}{}{}\x1b[0m",
                                    &kvc,
                                    &klines.next().unwrap(),
                                    &klines.next().unwrap()
                                )
                            }
                            "\"chainwork\"" | "\"bits\"" | "\"anchor\"" | "\"name\""
                            | "\"height\"" => {
                                println!(
                                    "\x1b[48;5;232m\x1b[38;5;136m{}{}{}\x1b[0m",
                                    &kvc,
                                    &klines.next().unwrap(),
                                    &klines.next().unwrap()
                                )
                            }
                            "\"valueDelta\"" => {
                                println!(
                                    "\x1b[48;5;232m\x1b[38;5;162m{}{}{}\x1b[0m",
                                    &kvc,
                                    &klines.next().unwrap(),
                                    &klines.next().unwrap()
                                );
                            }

                            "\"trees\"" => {
                                print!("\x1b[48;5;232m\x1b[38;5;141m{}\x1b[0m", &kvc);

                                loop {
                                    let jlines = klines.next().unwrap();
                                    if jlines != "\"tx\"" {
                                        print!("\x1b[48;5;232m\x1b[38;5;141m{}\x1b[0m", &jlines);
                                        continue;
                                    }
                                    println!();
                                    println!();
                                    print!("\x1b[48;5;232m\x1b[38;5;231m{}\x1b[0m", &jlines);

                                    loop {
                                        let jlines = klines.next().unwrap();
                                        if jlines != "\"valuePools\"" {
                                            println!(
                                                "\x1b[48;5;232m\x1b[38;5;231m{}\x1b[0m",
                                                &jlines
                                            );
                                            continue;
                                        }
                                        break;
                                    }

                                    break;
                                }
                            }
                            "\"sapling\"," | "\"orchard\"," | "\"sprout\"," => {
                                print!("\x1b[48;5;232m\x1b[38;5;159m\"pool\":{}\x1b[0m", &kvc)
                            }
                            "\"transparent\"," => {
                                print!(
                                    "\x1b[48;5;232m\x1b[38;5;160m\"pool\":{}\"Not Monitored\",\x1b[0m",
                                    &kvc);
                            }
                            _ => {} //End of match kvc.
                        }

                        //Returns to inner loop to check for more line data to parse.
                        continue;
                    }

                    //If the assert EQ fails and '== None', break.
                    break;
                }

                //Because if the linearity of the program, this if is currently redundant.
                //It is of no consequence and should remain in case of future use cases or operational bugs.
                if erval == 0 {
                    println!();
                    println!("\x1b[48;5;232m\x1b[38;5;220m* * * * * * * * * * * * * * * * * * * * * * * *\x1b[0m");
                    println!("\x1b[48;5;232m\x1b[38;5;220m* Hit Enter To Return, Input 'R' For Raw Json *\x1b[0m");

                    let mut usrinput = String::new();

                    //Get the User input.
                    io::stdin().read_line(&mut usrinput).unwrap();
                    let r = "R\n";

                    if usrinput.as_str() == r {
                        println!("{}", &k);
                    }
                    //End of erval == 0 check.
                }

                //End of 'event'
            }
            //End of length >= 2 check.
        }
        //End of main loop.
    }
    //End of fn main()
}
