use base64ct::{Base64, Encoding};

use rdns_lib::Message;

fn main() {
    tracing_subscriber::fmt::init();

    let response = Base64::decode_vec("MRiBoAABAAIAAAAABnByb3RvbgJtZQAADwABwAwADwABAAACigAWAAoEbWFpbApwcm90b25tYWlsAmNoAMAMAA8AAQAAAooADAAUB21haWxzZWPALg==").unwrap();

    let response_message = Message::from(&response[..]);

    if response_message.is_answer() {
        println!(
            "Response from {} with id 0x{:04x?}:",
            response_message.header().opcode(),
            response_message.header().id()
        );
        print!("{}, ", response_message.header().response_code());
        if response_message.header().authoritive_answer() {
            print!("Authoritative, ");
        } else {
            print!("Not authoritative, ");
        }
        if response_message.header().truncation() {
            print!("Truncated, ");
        } else {
            print!("Not truncated, ");
        }
        if response_message.header().recursion_available() {
            println!("Recursion is available");
        } else {
            println!("No recursion available");
        }
        println!("\nQuestions:");
        for q in response_message.questions().iter() {
            println!("{q}");
        }
        println!("\nAnswers:");
        for a in response_message.answers().iter() {
            println!("{a}");
        }
        println!("\nAuthorities:");
        for a in response_message.authorities().iter() {
            println!("{a}");
        }
        println!("\nAdditional Records:");
        for a in response_message.additional_records().iter() {
            println!("{a}");
        }
    }
}
