
use serialport::SerialPort;

use crate::estruturas::Tecla;

pub fn enviar_dados(tecla: Tecla, port: &mut Box<dyn SerialPort>) {
    let mut buf = [0; 3];
    buf[0] = tecla.pressed as u8;
    buf[1] = tecla.special as u8;
    buf[2] = tecla.key;

    port.write(&buf).unwrap();
    println!("{:#?}", buf);

    let mut serial_buf: Vec<u8> = vec![0; 8];
    match port.read(serial_buf.as_mut_slice()) {
        Ok(t) => println!("{}", String::from_utf8_lossy(&serial_buf[..t])),
        _ => println!("a")
    }
}