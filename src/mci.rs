// lib.rs
#![crate_type = "lib"]
#![crate_name = "mci"]

use std::io;
use std::net::{UdpSocket, SocketAddrV4, Ipv4Addr};
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use std::collections::HashMap;

/// Sends an array of bytes over a UDP connection
///
/// Sends a command sequence over a UDP socket. The socket is created using the
/// SocketAddrV4 information. The connection is hosted on ip address 0.0.0.0 and
/// port 8899.
pub fn send_bytes(connection: &SocketAddrV4, command: [u8; 3]) -> Result<(), io::Error> {
    // Send command to the wifi-bridge
    let ip = Ipv4Addr::new(0, 0, 0, 0);
    let bind_connection = SocketAddrV4::new(ip, 8899);
    let socket = try!(UdpSocket::bind(bind_connection));

    let buf = &command;
    // println!("buffer: {:?}", buf);
    try!(socket.send_to(buf, connection));
    // Err(io::Error::new(io::ErrorKind::NotFound, "Error"))  // Added for testing purposes
    Ok(())
}


pub struct GroupWhite<'a> {
    connection: SocketAddrV4,
    group: u8,
    time: SystemTime,
    sleep_time: u32,
    commands: HashMap<&'a str, u8>,
}

/// Controls a white-group
///
/// ...
impl<'a> GroupWhite<'a> {
    pub fn new(connection: &SocketAddrV4, group: u8) -> GroupWhite {
        // let mut commands: HashMap<&str, u8> = HashMap::new();
        let mut commands = HashMap::new();
        // Standard On/Off
        commands.insert("WHITE_ALL_ON", 0x35);
        commands.insert("WHITE_ALL_OFF", 0x39);
        commands.insert("GROUP_1_ON", 0x38);
        commands.insert("GROUP_1_OFF", 0x3B);
        commands.insert("GROUP_2_ON", 0x3D);
        commands.insert("GROUP_2_OFF", 0x33);
        commands.insert("GROUP_3_ON", 0x37);
        commands.insert("GROUP_3_OFF", 0x3A);
        commands.insert("GROUP_4_ON", 0x32);
        commands.insert("GROUP_4_OFF", 0x36);
        // Standard Brightness/WHITE-COLOR
        commands.insert("BRIGHTNESS_UP", 0x3C);
        commands.insert("BRIGHTNESS_DOWN", 0x34);
        commands.insert("WARM_WHITE_INCREASE", 0x3E);
        commands.insert("COOL_WHITE_INCREASE", 0x3F);
        // Specials FullBrightness/NIGHTMODE
        commands.insert("FULL_BRIGHTNESS_ALL", 0xB5);
        commands.insert("FULL_BRIGHTNESS_GROUP_1", 0xB8);
        commands.insert("FULL_BRIGHTNESS_GROUP_2", 0xBD);
        commands.insert("FULL_BRIGHTNESS_GROUP_3", 0xB7);
        commands.insert("FULL_BRIGHTNESS_GROUP_4", 0xB2);
        // send 100ms after GroupOff
        commands.insert("NIGHT_MODE_ALL", 0xB9);
        commands.insert("NIGHT_MODE_GROUP_1", 0xBB);
        commands.insert("NIGHT_MODE_GROUP_2", 0xB3);
        commands.insert("NIGHT_MODE_GROUP_3", 0xBA);
        commands.insert("NIGHT_MODE_GROUP_4", 0xB);
        GroupWhite {
            connection: connection.clone(),
            group: group,
            time: SystemTime::now(),
            sleep_time: 100*1000,
            commands: commands,
        }
    }

    fn send_command(&mut self, command: u8) -> Result<(), io::Error> {
        // Send the command as send_bytes
        let bytes: [u8; 3] = [command, 0x00, 0x55];
        // Wait the appropriate amount of time
        let current_time = SystemTime::now();
        match current_time.duration_since(self.time) {
            Ok(dtime) => {
                // it prints '2'
                // println!("{}:{}", dtime.as_secs(), dtime.subsec_nanos());
                if dtime.as_secs() < 1 {
                    if dtime.subsec_nanos() < self.sleep_time {
                        // println!("wait: {}", self.sleep_time - dtime.subsec_nanos());
                        sleep(Duration::new(0, self.sleep_time - dtime.subsec_nanos()));
                    }
                }
                self.time = SystemTime::now();
            }
            Err(e) => {
               // an error occured!
               println!("Error: {:?}", e);
            }
        }
        try!(send_bytes(&self.connection, bytes));
        Ok(())
    }

    pub fn on(&mut self) -> Result<(), io::Error> {
        // Switch group on
        let command: u8 = match self.group {
            1 => *self.commands.get("GROUP_1_ON").unwrap(),
            2 => *self.commands.get("GROUP_2_ON").unwrap(),
            3 => *self.commands.get("GROUP_3_ON").unwrap(),
            4 => *self.commands.get("GROUP_4_ON").unwrap(),
            _ => *self.commands.get("WHITE_ALL_ON").unwrap(),
        };
        try!(self.send_command(command));
        Ok(())
    }

    pub fn off(&mut self) -> Result<(), io::Error> {
        // Switch group on
        let command = match self.group {
            1 => *self.commands.get("GROUP_1_OFF").unwrap(),
            2 => *self.commands.get("GROUP_2_OFF").unwrap(),
            3 => *self.commands.get("GROUP_3_OFF").unwrap(),
            4 => *self.commands.get("GROUP_4_OFF").unwrap(),
            _ => *self.commands.get("WHITE_ALL_OFF").unwrap(),
        };
        try!(self.send_command(command));
        Ok(())
    }

    pub fn increase_brightness(&mut self) -> Result<(), io::Error> {
        // Increase brightness
        let steps = 1;  // value should be between 1 and 30
        for _ in 0..steps {
            let command = *self.commands.get("BRIGHTNESS_UP").unwrap();
            try!(self.send_command(command));
        }
        Ok(())
    }

    pub fn decrease_brightness(&mut self) -> Result<(), io::Error> {
        // Decrease brightness
        let steps = 1;  // value should be between 1 and 30
        for _ in 0..steps {
            let command = *self.commands.get("BRIGHTNESS_DOWN").unwrap();
            try!(self.send_command(command));
        }
        Ok(())
    }

    pub fn increase_warmth(&mut self) -> Result<(), io::Error> {
        // Increase warmth
        let steps = 1;  // value should be between 1 and 30
        for _ in 0..steps {
            let command = *self.commands.get("WARM_WHITE_INCREASE").unwrap();
            try!(self.send_command(command));
        }
        Ok(())
    }

    pub fn decrease_warmth(&mut self) -> Result<(), io::Error> {
        // Decrease warmth
        let steps = 1;  // value should be between 1 and 30
        for _ in 0..steps {
            let command = *self.commands.get("COOL_WHITE_INCREASE").unwrap();
            try!(self.send_command(command));
        }
        Ok(())
    }

    pub fn brightmode(&mut self) -> Result<(), io::Error> {
        // Enable full brightness
        try!(self.on());
        let command = match self.group {
            1 => *self.commands.get("FULL_BRIGHTNESS_GROUP_1").unwrap(),
            2 => *self.commands.get("FULL_BRIGHTNESS_GROUP_2").unwrap(),
            3 => *self.commands.get("FULL_BRIGHTNESS_GROUP_3").unwrap(),
            4 => *self.commands.get("FULL_BRIGHTNESS_GROUP_4").unwrap(),
            _ => *self.commands.get("FULL_BRIGHTNESS_ALL").unwrap(),
        };
        try!(self.send_command(command));
        Ok(())
    }

    pub fn nightmode(&mut self) -> Result<(), io::Error> {
        // Enable nightmode
        try!(self.on());
        let command = match self.group {
            1 => *self.commands.get("NIGHT_MODE_GROUP_1").unwrap(),
            2 => *self.commands.get("NIGHT_MODE_GROUP_2").unwrap(),
            3 => *self.commands.get("NIGHT_MODE_GROUP_3").unwrap(),
            4 => *self.commands.get("NIGHT_MODE_GROUP_4").unwrap(),
            _ => *self.commands.get("NIGHT_MODE_ALL").unwrap(),
        };
        try!(self.send_command(command));
        Ok(())
    }
}