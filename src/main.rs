extern crate ws;
extern crate openssl_probe;

use ws::{connect, CloseCode};


fn main() {
    openssl_probe::init_ssl_cert_env_vars();
    let url = std::env::var("WSS_ENDPOINT").unwrap();

    connect(url, |out| {
        let start_message = "{\"signal\": \"start\"}";
        out.send(start_message).unwrap();

        move |msg| {
            match msg {
                ws::Message::Text(data) => {
                    let number = parse_number(&data);
                    let answer = match number {
                        Some(number) => get_answer(number),
                        None => {
                            println!("{}", data);
                            out.close(CloseCode::Normal).unwrap();
                            panic!("couldn't parse number")
                        }
                    };
                    return out.send(answer);
                },
                ws::Message::Binary(_) => {
                    println!("invalid message");
                    return out.close(CloseCode::Normal);
                },
            }
        }
    }).unwrap();
}

fn get_answer(number: u64) -> String {
    let mut answer = String::new();
    if number % 3 == 0 {
        answer = answer + "Fizz";
    }
    if number % 5 == 0 {
        answer = answer + "Buzz";
    }
    if number % 7 == 0 {
        answer = answer + "Moi";
    }

    if answer.len() == 0 {
        return format!("{{\"answer\": \"{}\"}}", number.to_string());
    } else {
        return format!("{{\"answer\": \"{}\"}}", answer);
    }
}

fn parse_number(data: &str) -> Option<u64> {
    let prop_idx = data.find("number\":");
    let comma_idx = data.find(",");
    match (prop_idx, comma_idx) {
        (Some(prop_idx), Some(comma_idx)) => {
            data[prop_idx + 8..comma_idx].parse().ok()
        },
        _ => None,
    }
}
