use std::io;
use log::debug;

fn main() -> io::Result<()> {
  env_logger::init();
  // Create a new TUN interface named "tun0" n TUN mode.
  let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;

  // Define a buffer of size 1504 bytes (maximum Ethernet frame size without CRC) to store received data.
  let mut buf = [0u8; 1504];

  // main loop to continuously receive data from the interface
  loop {
    let nbytes = nic.recv(&mut buf[..])?;
    let flags = u16::from_be_bytes([buf[0], buf[1]]);
    let proto = u16::from_be_bytes([buf[2], buf[3]]);
    
    if proto != 0x0800 {
      continue;
    }
    
    match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..]) {
      Ok(iph) => {
        let src = iph.source_addr();

        let dst = iph.destination_addr();

        let proto = iph.protocol();

        if proto != etherparse::IpNumber(0x06) {
          continue;
        }
        
        println!("slice length is {}", iph.slice().len());
        match etherparse::TcpHeaderSlice::from_slice(&buf[4+iph.slice().len()..]) {
          Ok(tcph) => {
            eprintln!("{}->{}: TCP to port {}", src, dst, tcph.destination_port());
          },
          Err(e) => {
            eprintln!("error parsing TCP header: {:?}", e);
          }
        }
      },
      Err(e) => {
        eprintln!("error parsing IPv4 header: {:?}", e);
      }
    }

    eprintln!("flags={:x}, proto={:x}", flags, proto);
    eprintln!("read {} bytes: {:x?}", nbytes-4, &buf[4..nbytes]);
  }
}
