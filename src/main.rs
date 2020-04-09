mod controller;


use crate::controller::{MoterSpeed,Rolling,Action,ProCon,Controller};
use rumqtt::{MqttClient, MqttOptions, QoS, Notification};
use std::thread;
use std::sync::mpsc::{channel,Receiver};

fn main()  {
    let (tx,rx) = channel();
    thread::spawn(move ||  {
        let mut con = ProCon::new().unwrap();
        loop{
            if let Some(action) = con.next_event(){
                let _ = tx.send(action);
            }
        }
    });

//   let mqttoptions = MqttOptions::new("Mac","localhost", 1883).set_keep_alive(100);
   let mqttoptions = create_mqtt_options();

    let (mqtt_client, notifications) = MqttClient::start(mqttoptions).unwrap();
    thread::spawn(move || {
        publish_action(mqtt_client,rx);
    });

    for notification in notifications {
        if let Notification::Publish(publish) = notification {
            let payload:Action = serde_json::from_slice(&publish.payload).unwrap();
            println!("pub = {:?}",payload)

        }
    }

}
fn publish(request: &mut MqttClient, action: &Action){
    let topic = "camrobot/action".to_owned();

    let payload = serde_json::to_vec(action).unwrap();
    let _ = request.publish(&topic, QoS::AtLeastOnce, false,payload);

}
fn publish_action(mut mqtt_client:MqttClient, controller_rev: Receiver<Action>) {
    let topic = "camrobot/action".to_owned();
    let _ = mqtt_client.subscribe(&topic, QoS::AtLeastOnce);

    let mut pre_left_action = Action::MoveLeftCrawler(Rolling::Normal,MoterSpeed::Stop);
    let mut pre_right_action = Action::MoveRightCrawler(Rolling::Normal,MoterSpeed::Stop);
    while let Ok(action) = controller_rev.recv(){
        match action{
            Action::MoveLeftCrawler(_,_) => {
                if pre_left_action != action {
                    publish(&mut mqtt_client, &action);
                    pre_left_action = action;
                }
            },
            Action::MoveRightCrawler(_,_) => {
                if pre_right_action != action {
                    publish(&mut mqtt_client, &action);
                    pre_right_action = action;
                }
            },
            Action::ToggleEye => {
                publish(&mut mqtt_client, &action);
            },
            Action::Stop => {
                publish(&mut mqtt_client, &action);
            }
            Action::End => {
                publish(&mut mqtt_client, &action);
                std::process::exit(0)
            },

        } 
    }

}
fn create_mqtt_options() -> MqttOptions{
    let client_id = "Mac".to_owned();
    let ca = include_bytes!("../certs/AmazonRootCA1.pem").to_vec();
    let client_cert = include_bytes!("../certs/xxx.pem.crt").to_vec();
    let client_key = include_bytes!("../certs/xxx.pem.key").to_vec();

    MqttOptions::new(client_id, "xxx.amazonaws.com", 8883)
    .set_ca(ca)
    .set_client_auth(client_cert, client_key)
    .set_keep_alive(10)
}