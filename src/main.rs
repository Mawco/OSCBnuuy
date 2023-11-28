use std::io::Error;

// use async_std::net::UdpSocket;
use openvr::{init, ApplicationType, TrackingUniverseOrigin};

mod utils;

#[async_std::main]
async fn main() -> Result<(), Error> {
    let context = unsafe { init(ApplicationType::Background) }.unwrap();
    let system = context.system().unwrap();

    // let _listener = UdpSocket::bind("127.0.0.1:9001").await?;

    let poses = system
        .device_to_absolute_tracking_pose(TrackingUniverseOrigin::RawAndUncalibrated, 0.0);
    
    for pose in poses.iter() {
        let (yaw, pitch, roll) = utils::math::decompose_rotation(utils::math::matrix34_to_matrix3(pose.device_to_absolute_tracking()));
        println!("Yaw: {:.2} degrees", yaw.to_degrees());
        println!("Pitch: {:.2} degrees", pitch.to_degrees());
        println!("Roll: {:.2} degrees", roll.to_degrees());
        break;
    }

    Ok(())
}