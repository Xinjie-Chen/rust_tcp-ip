use std::io;

fn main() -> io::Result<()> {
    // Create a new TUN interface named "tun0" n TUN mode.
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;

    // Define a buffer of size 1504 bytes (maximum Ethernet frame size without CRC) to store received data.
    let mut buf = [0u8; 1504];

    // main loop to continuously receive data from the interface
    loop {
        let nbytes = nic.recv(&mut buf[..])?;

        eprintln!("read {} bytes: {:x?}", nbytes, &buf[..nbytes]);
    }
}
