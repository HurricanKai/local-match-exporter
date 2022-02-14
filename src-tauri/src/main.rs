use league_client_connector::LeagueClientConnector;
use std::thread;
use websocket::{OwnedMessage};
use serde_json::{Map, Value};


#[derive(Clone, serde::Serialize)]
struct LeagueEvent {
  uri: String,
  event_type: String,
  data: Value,
}


#[tauri::command]
async fn connect(window: tauri::Window) -> () {
  // TODO: I don't understand Rust error handling.
  // I *think* you are supposed to define like a custom error enum (https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html)
  // But I can't figure out how tauri wants this to work

  let lockfile = LeagueClientConnector::parse_lockfile().unwrap();

  println!("Lockfile: {:?}", lockfile);

  let url = "wss://".to_owned() + &lockfile.username + ":" + &lockfile.password + "@" + &lockfile.address + ":" + &lockfile.port.to_string() + "/";
  println!("Connecting to {:?}", url);


  let headers = websocket::header::Headers::new();

  println!("Headers: {:?}", headers);

  let mut client = websocket::ClientBuilder::new(&url)
    .unwrap()
    .add_protocol("wamp")
    .custom_headers(&headers)
    .connect_secure(Some(websocket::native_tls::TlsConnector::builder()
      .danger_accept_invalid_certs(true)
      .min_protocol_version(Some(websocket::native_tls::Protocol::Tlsv12))
      .build().unwrap()))
    .unwrap();

    

  println!("Connected");
    
  client.send_message(&OwnedMessage::Text("[5,\"OnJsonApiEvent\"]".to_owned())).unwrap();
  thread::spawn(move || {
    loop {
      let message = client.recv_message().unwrap();
      match message {
        OwnedMessage::Text(ref t) => {
          let parsed : Value = serde_json::from_str(&t).unwrap();
          if parsed[0].as_i64().unwrap() == 8
          {
            let body = parsed[2].as_object().unwrap();
            let data = body["data"].to_owned();
            let event_type = body["eventType"].as_str().unwrap().to_owned();
            let uri = body["uri"].as_str().unwrap().to_owned();
            window.emit("league-event", LeagueEvent { data, event_type, uri }).unwrap();
          }
          else
          {
            println!("Non-Event message: {:?}", message);
          }
        },
        _ => {
          println!("Weird message {:?}", message);
        },
      }
    }
  });


  // TODO: Figure this out
  // There is no way to split a TlsStream, so the below isn't posssible.
  // I suspect we want to somehow do this https://github.com/websockets-rs/rust-websocket/blob/master/examples/ssl-client.rs
  // but without async (yada yada commands can be async but I don't want to deal with that)
  //
  // The end goal is to raise an event whenever the LCU raises something, while also forwarding messages from some channel
  // (that we then store as state).

    /*
  let (reader, writer) = client.split();
  let mut reader = client.reader_mut();
  let mut writer = client.writer_mut();

  println!("Connected");
  
  let (tx, rx) = std::sync::mpsc::channel();
  let tx_1 = tx.clone();
  
  println!("Creating threads");
  let send_loop = std::thread::spawn(move || {
    println!("sending...");
		loop {
			// Send loop
			let message = match rx.recv() {
				Ok(m) => m,
				Err(e) => {
					println!("Send Loop: {:?}", e);
					return;
				}
			};
			match message {
				OwnedMessage::Close(_) => {
					let _ = writer.send_message(&message);
					// If it's a close message, just send it and then return.
					return;
				}
				_ => (),
			}
			// Send the message
      println!("Sending {:?}", message);
			match writer.send_message(&message) {
				Ok(()) => (),
				Err(e) => {
					println!("Send Loop: {:?}", e);
					let _ = writer.send_message(&Message::close());
					return;
				}
			}
		}
	});

  let receive_loop = std::thread::spawn(move || {
    println!("receiving...");
    for message in reader.incoming_messages() {
      let message = match message {
        Ok(m) => m,
        Err(e) => {
          println!("Receive Loop: {:?}", e);
          return;
        }
      };
      println!("Test: {:?}", message);
      match message {
        OwnedMessage::Close(_) => {
          return;
        }
        OwnedMessage::Ping(data) => {
          match tx_1.send(OwnedMessage::Pong(data)) {
            Ok(()) => (),
            Err(e) => {
              println!("Receive Loop: {:?}", e);
              return;
            }
          }
        }
        // Say what we received
        _ => println!("Receive Loop: {:?}", message),
      }
    }
  });

  tx.send(websocket::OwnedMessage::Text("[5,\"OnJsonApiEvent\"]".to_owned())).unwrap();*/
}


fn main() {
  tauri::Builder::default() 
    .invoke_handler(tauri::generate_handler![connect])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
