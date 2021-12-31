use std::io::Read;

#[derive(serde::Serialize)]
struct Request {
    title: String,
    text: String,
}

impl Request {
    fn new(title:&str, text:&str) -> Request {
        Request{title:title.to_string(), text:text.to_string()}
    }
}

fn main(){
    let incoming_webhook_url = std::env::var("INCOMING_WEBHOOK_URL").expect("INCOMING_WEBHOOK_URL environment variable must be defined");

    let mut f = std::fs::File::open("test/demo.md").expect("open file");
    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("read to string");

    let req = Request::new("demo", &buf);

    let client = reqwest::blocking::Client::new();
    let resp = client.post(incoming_webhook_url).json(&req).send().expect("POST request");
    println!("{:?}", resp.text().expect("response as text"));
}
