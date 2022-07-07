use std::io::{Read, Write};
use std::net::TcpStream;
use std::{io, str};
use serde::{Serialize, Deserialize};

mod challenge_md5;

fn main() {
    let stream = std::net::TcpStream::connect("127.0.0.1:7878");
    match stream {
        Ok(mut stream) => {
            let hello = Message::Hello;
            send(&mut stream, hello);

            let subscribe = Message::Subscribe(Subscribe { name: "player".parse().unwrap() });
            send(&mut stream, subscribe);

            // welcome
            let welcome_array = [0; 4];
            receive(&mut stream, welcome_array);

            // subscribeResult
            let subscribe_array = [0; 4];
            receive(&mut stream, subscribe_array);

            // leaderBoard
            let leaderboard_array = [0; 4];
            receive(&mut stream, leaderboard_array);

            // challenge md5
            let challenge_array = [0; 4];
            receive(&mut stream, challenge_array);
            loop {
                match challenge {
                    Challenge::MD5HashCash(hashcash) => {
                        let complexity = hashcash.complexity;
                        let message = hashcash.message;
                        println!("ll {:?} {:?}", complexity, message);

                        let md5answer = challenge_md5::md5_hash(complexity, message);

                        println!("ll {:?}", md5answer);

                        let challenge_result = Message::ChallengeResult(ChallengeResult { answer: ChallengeAnswer::MD5HashCash(md5answer), next_target: "player2".parse().unwrap() });
                        send(&mut stream, challenge_result);

                        break;
                    }
                }
            }

            //RoundSummary if one player
            let summary_array = [0; 4];
            receive(&mut stream, summary_array);

            //End of game
            let end_array = [0; 4];
            receive(&mut stream, end_array);
        }
        Err(err) => panic!("Cannot connect: {err}")
    }
}

fn receive(stream: &mut TcpStream, mut array: [u8; 4]) {
    stream.read( &mut array).unwrap();

    let size_message: u32 = u32::from_be_bytes(array);
    let size_message = size_message as usize;
    let mut vector = vec![0; size_message];

    stream.read(&mut vector).unwrap();

    let message_received = std::str::from_utf8(&*vector).unwrap();

    let welcome_serialized = serde_json::to_string(&message_received).unwrap();
    let a = welcome_serialized.replace("\\", "");

    let first_last_off: &str = &a[1..a.len() - 1];
    let message: Result<Message, _> = serde_json::from_str(&first_last_off);

    match message {
        Ok(m) => println!("message={m:?}"),
        Err(err) => println!("error={err:?}")
    }
}

fn send(stream: &mut TcpStream, message_to_send: Message) {
    let message_to_serialized = serde_json::to_string(&message_to_send);
    let message_to_serialized = message_to_serialized.unwrap();
    let serialized_message_length_to_u32 = (message_to_serialized.len()) as u32;

    stream.write_all(&serialized_message_length_to_u32.to_be_bytes()).unwrap();

    stream.write_all(&message_to_serialized.as_bytes()).unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
struct Welcome{
    version: i32
}

#[derive(Debug, Serialize, Deserialize)]
struct Subscribe {
    name: String
}

#[derive(Debug, Serialize, Deserialize)]
enum SubscribeError {
    AlreadyRegistered,
    InvalidName
}

#[derive(Debug, Serialize, Deserialize)]
enum SubscribeResult {
    Ok,
    Err(SubscribeError)
}

#[derive(Debug, Serialize, Deserialize)]
enum Message {
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(PublicLeaderBoard),
    Challenge(Challenge),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame)
}

#[derive(Debug, Serialize, Deserialize)]
struct PublicLeaderBoard(Vec<PublicPlayer>);

#[derive(Debug, Serialize, Deserialize)]
struct PublicPlayer {
    name: String,
    stream_id: String,
    score: i32,
    steps: u32,
    is_active: bool,
    total_used_time: f64
}

#[derive(Debug, Serialize, Deserialize)]
enum Challenge {
    //ChallengeName(ChallengeInput),
    MD5HashCash(ChallengeInput),
}

#[derive(Debug, Serialize, Deserialize)]
struct ChallengeInput {
    complexity: u32,
    message: String
}

#[derive(Debug, Serialize, Deserialize)]
struct ChallengeOutput {
    seed: u64,
    hashcode: String
}

#[derive(Debug, Serialize, Deserialize)]
enum ChallengeAnswer {
    ChallengeName(ChallengeOutput)
}

#[derive(Debug, Serialize, Deserialize)]
struct ChallengeResult {
    answer: ChallengeAnswer,
    next_target: String
}

#[derive(Debug, Serialize, Deserialize)]
enum ChallengeValue {
    Unreachable,
    Timeout,
    BadResult(BadResult),
    OK(Ok)
}

#[derive(Debug, Serialize, Deserialize)]
struct BadResult {
    used_time: f64,
    next_target: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Ok {
    used_time: f64,
    next_target: String
}

#[derive(Debug, Serialize, Deserialize)]
struct MD5HashCashInput {
    complexity: u32,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MD5HashCashOutput {
    seed: u64,
    hashcode: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RoundSummary {
    challenge: String,
    chain: Vec<ReportedChallengeResult>
}

#[derive(Debug, Serialize, Deserialize)]
struct ReportedChallengeResult {
    name: String,
    value: ChallengeValue
}

#[derive(Debug, Serialize, Deserialize)]
struct EndOfGame {
    leader_board: PublicLeaderBoard
}
