use std::net::IpAddr;
use std::str::FromStr;

use clap::Parser;

mod arguments;
mod os;

fn main() {
    tracing_subscriber::fmt::init();

    let arguments = arguments::Arguments::parse();

    let server = IpAddr::from_str(&arguments.server.unwrap_or_default()).unwrap_or(
        *os::get_host_dns()
            .expect("get host dns server list")
            .first()
            .expect("get first host dns server"),
    );

    let message: Vec<u8> = rdns_lib::message::Message::new_query(
        true,
        vec![rdns_lib::question::Question::new(
            rdns_lib::domain_name::DomainName::from_str(&arguments.host_name).unwrap(),
            arguments.ty.into(),
            arguments.class.into(),
        )],
    )
    .into();

    let response_message = if arguments.use_tcp {
        let response = tcp_query(message.as_slice(), server);
        rdns_lib::message::Message::try_from(response.as_slice()).expect("message parse")
    } else {
        let response = udp_query(message.as_slice(), server);
        let response_message =
            rdns_lib::message::Message::try_from(response.as_slice()).expect("message parse");
        if response_message.header().truncation() {
            // Message was truncated; Retry with TCP
            let response = tcp_query(message.as_slice(), server);
            rdns_lib::message::Message::try_from(response.as_slice()).expect("message parse")
        } else {
            response_message
        }
    };

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

fn udp_query(message: &[u8], server: IpAddr) -> Vec<u8> {
    let socket = std::net::UdpSocket::bind(("0.0.0.0", 0)).expect("UDP socket bound");
    socket.connect((server, 53)).expect("connected to server");
    socket.send(message).expect("message sent");
    let mut buf = [0; 512];
    socket
        .recv(&mut buf)
        .map(|r| buf[..r].to_vec())
        .expect("response recieved")
}

fn tcp_query(message: &[u8], server: IpAddr) -> Vec<u8> {
    use std::io::prelude::*;

    println!("Connecting to server");
    let mut socket = std::net::TcpStream::connect((server, 53)).expect("tcp connection made");
    socket
        .set_read_timeout(Some(std::time::Duration::from_secs(15)))
        .expect("read timeout set");
    println!("Connected, sending message");
    socket
        .write(&(message.len() as u16).to_be_bytes())
        .expect("length sent");
    socket.write(message).expect("message sent");
    socket.flush().expect("socket flushed");
    println!("Message sent. Awaiting reply...");
    let mut length_buffer = [0_u8; 2];
    socket.read(&mut length_buffer).expect("length read");
    let length = u16::from_be_bytes(length_buffer) as usize;
    let mut buf = [0_u8; 0xFFFF];
    let mut sized_read = socket.take(length as u64);
    let read_bytes = sized_read.read(&mut buf).expect("read failed");
    println!("Reply recieved. Read {read_bytes} bytes.");
    buf[..length].to_vec()
}
