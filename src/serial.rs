use serialport::SerialPort;

use crate::estruturas::Tecla;

pub fn enviar_dados(tecla: Tecla, port: &mut Box<dyn SerialPort>) {
    let mut buf = [0; 3];
    buf[0] = tecla.pressed as u8;
    buf[1] = tecla.special as u8;
    buf[2] = tecla.key;

    port.write(&buf).unwrap();
    println!("{:?}", buf);
}