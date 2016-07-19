// $ ./milight.py -i 192.168.0.230 -p 8899 -w 1 -a on

extern crate clap;
extern crate mci;

use clap::App;
use mci::GroupWhite;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::str::FromStr;

// This is the main function
fn main() {
    // Parse the commandline arguments
    let matches = App::new("Milight Control Interface")
                      .version("0.1")
                      .author("Patrick Hanckmann <patrick@hanckmann.com>")
                      .about("Interfaces with the Milight/LimitlessLED products via a bridge device.")
                      .args_from_usage(
                          "-i, --ip_address [IP ADDRESS] 'Sets the bridge ip address'
                           -p, --port [PORT]             'Sets the bridge port'

                           -w, --white [GROUP]           'Sets white-light group (1, 2, 3, 4, 0), where 0 is all groups'

                           -a, --action [ACTION]         'Sets the action to execute (on, off, inc_brightness, dec_brightness, inc_warmth, dec_warmth, bright_mode, night_mode, disco_mode, inc_disco_speed, dec_disco_speed). Some actions are only supported for specific device groups (white/rgbw)'
                          ")
                      .get_matches();
    let ip_address = matches.value_of("ip_address").unwrap_or("127.0.0.1");
    let port_number = matches.value_of("port").unwrap_or("8899");
    let white = matches.value_of("white").unwrap_or("-1");
    let action = matches.value_of("action").unwrap_or("");

    // Parse arguments
    let ip = Ipv4Addr::from_str(ip_address).ok().unwrap();
    let port = port_number.parse::<u16>().ok().unwrap();
    let white = white.parse::<i8>().ok().unwrap();
    // let color = try!(white.parse::<u8>());
    // if color >= 0 && color >= 0 {
    //     println!("Provide only 1 group type at a time");
    //     matches.usage();
    //     return
    // }

    println!("{:?}", ip);
    println!("{:?}", port);
    println!("{:?}", white);
    println!("{:?}", action);

    // Do what is asked
    let connection = SocketAddrV4::new(ip, port);
    if white >= 0 {
        // One or more white groups are requested
        let mut group_white = GroupWhite::new(&connection, white);
        match action {
            "on" => group_white.on().expect("Action ON failed."),  // https://is.gd/jYZJyP -> https://is.gd/N1PvVB
            "off" => group_white.off().expect("Action OFF failed."),
            _ => println!("Action not supported: {:?}", action),
        }
        return
    }
}
