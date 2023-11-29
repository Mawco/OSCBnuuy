use std::io::Error;

use async_std::net::UdpSocket;
use openvr::{init, ApplicationType, TrackingUniverseOrigin};
use rosc::{OscMessage, encoder, OscPacket, OscType};

mod utils;

#[async_std::main]
async fn main() -> Result<(), Error> {
    // let args: Vec<String> = std::env::args().collect();
    // TODO: add args so that you can set custom ports for listening and sending
    let listen_port: u16 = 9001;
    let send_port: u16 = 9000;

    println!("OSCBnuuy will listen on port {} and send on port {}", listen_port, send_port);
    
    let context = unsafe { init(ApplicationType::Background) }.unwrap();
    let system = context.system().unwrap();
    let sock = UdpSocket::bind("127.0.0.1:9001").await?;

    loop {
        let poses = system
        .device_to_absolute_tracking_pose(TrackingUniverseOrigin::RawAndUncalibrated, 0.0);

        for pose in poses.iter() {
            let (_yaw, _pitch, _roll) = utils::math::decompose_rotation(utils::math::matrix34_to_matrix3(pose.device_to_absolute_tracking()));
            println!("( x: {:.2}, z: {:.2} )", _pitch.to_degrees(), _roll.to_degrees());

            let rot_x_msg = encoder::encode(&OscPacket::Message(OscMessage {
                addr: "/avatar/parameters/hmd_rot_x".to_string(),
                args: vec![OscType::Float(
                    ((_pitch.to_degrees() + 90.0) / 180.0).clamp(0.0, 1.0)
                )]
            })).unwrap();
            sock.send_to(&rot_x_msg, "127.0.0.1:9000").await?;

            let rot_z_msg = encoder::encode(&OscPacket::Message(OscMessage {
                addr: "/avatar/parameters/hmd_rot_z".to_string(),
                args: vec![OscType::Float(
                    ((_roll.to_degrees() + 180.0) / 360.0).clamp(0.0, 1.0)
                )]
            })).unwrap();
            sock.send_to(&rot_z_msg, "127.0.0.1:9000").await?;
            break;
        }
    }
}