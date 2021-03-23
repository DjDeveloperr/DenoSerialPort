use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::ZeroCopyBuf;
use serde::Serialize;
use serialport::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::str::FromStr;

thread_local! {
    static PORTS: RefCell<HashMap<u32, Box<dyn SerialPort>>> = RefCell::new(HashMap::new());
}

#[derive(Serialize)]
struct OpSerialPort {
    name: String,
    port_type: i32,
    usb_info: Option<OpUsbPortInfo>,
}

#[derive(Serialize)]
struct OpUsbPortInfo {
    vid: u16,
    pid: u16,
    serial_number: Option<String>,
    manufacturer: Option<String>,
    product: Option<String>,
}

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    interface.register_op("op_available_ports", op_available_ports);
    interface.register_op("op_serial_new", op_new_serial);
    interface.register_op("op_serial_close", op_close_serial);
    interface.register_op("op_serial_set_baud_rate", op_serial_set_baud_rate);
    interface.register_op("op_serial_set_break", op_serial_set_break);
    interface.register_op("op_serial_clear_break", op_serial_clear_break);
    interface.register_op("op_serial_bytes_to_read", op_serial_bytes_to_read);
    interface.register_op("op_serial_bytes_to_write", op_serial_bytes_to_write);
    interface.register_op(
        "op_serial_write_data_terminal_ready",
        op_serial_write_data_terminal_ready,
    );
    interface.register_op(
        "op_serial_write_request_to_send",
        op_serial_write_request_to_send,
    );
    interface.register_op("op_serial_write", op_serial_write);
    interface.register_op("op_serial_write_all", op_serial_write_all);
    interface.register_op("op_serial_read", op_serial_read);
    interface.register_op("op_serial_read_clear_to_send", op_serial_read_clear_to_send);
    interface.register_op(
        "op_serial_read_data_set_ready",
        op_serial_read_data_set_ready,
    );
    interface.register_op(
        "op_serial_read_ring_indicator",
        op_serial_read_ring_indicator,
    );
    interface.register_op(
        "op_serial_read_carrier_detect",
        op_serial_read_carrier_detect,
    );
    interface.register_op("op_serial_read_all", op_serial_read_all);
    interface.register_op("op_serial_clear", op_serial_clear);
}

fn op_available_ports(_interface: &mut dyn Interface, _args: &mut [ZeroCopyBuf]) -> Op {
    let ports = available_ports().expect("failed to retrieve available ports");
    let mut op_ports = Vec::with_capacity(ports.len());
    for port in ports {
        let port_type;
        let mut usb_info: Option<OpUsbPortInfo> = Option::default();
        match port.port_type {
            SerialPortType::BluetoothPort => {
                port_type = 3;
            }
            SerialPortType::PciPort => {
                port_type = 1;
            }
            SerialPortType::UsbPort(u) => {
                port_type = 2;
                usb_info = Option::from(OpUsbPortInfo {
                    vid: u.vid,
                    pid: u.pid,
                    serial_number: u.serial_number,
                    manufacturer: u.manufacturer,
                    product: u.product,
                })
            }
            SerialPortType::Unknown => {
                port_type = 4;
            }
        }

        let op_port = OpSerialPort {
            name: port.port_name,
            port_type,
            usb_info,
        };

        op_ports.push(op_port)
    }

    Op::Sync(
        deno_core::serde_json::to_vec(&op_ports)
            .unwrap()
            .into_boxed_slice(),
    )
}

fn has_id(id: u32) -> Result<bool> {
    PORTS.with(|map| {
        let hm = map.borrow_mut();
        if hm.contains_key(&id) {
            Ok(true)
        } else {
            Ok(false)
        }
    })
}

fn get_next_id() -> Result<u32> {
    let mut id: u32 = 0;
    while has_id(id).unwrap() {
        id += 1;
    }
    Ok(id)
}

fn op_new_serial(_interface: &mut dyn Interface, args: &mut [ZeroCopyBuf]) -> Op {
    let path = std::str::from_utf8(args.get(0).unwrap()).unwrap();
    let baud_rate = u32::from_str(std::str::from_utf8(args.get(1).unwrap()).unwrap()).unwrap();
    let port = serialport::new(path, baud_rate)
        .open()
        .expect("failed to open serialport");
    let id = get_next_id().unwrap();
    PORTS.with(|map| {
        let mut mymap = map.borrow_mut();
        if let Some(_target) = mymap.get_mut(&id) {
            let res = b"n";
            Op::Sync(res.to_vec().into_boxed_slice())
        } else {
            mymap.insert(id, Box::from(port));
            let res = id.to_string();
            Op::Sync(res.as_bytes().to_vec().into_boxed_slice())
        }
    })
}

fn op_close_serial(_interface: &mut dyn Interface, args: &mut [ZeroCopyBuf]) -> Op {
    let id = u32::from_str(std::str::from_utf8(args.get(0).unwrap()).unwrap()).unwrap();
    PORTS.with(|map| {
        if let Some(_target) = map.borrow_mut().get_mut(&id) {
            map.borrow_mut().remove(&id);
            let res = b"0";
            Op::Sync(res.to_vec().into_boxed_slice())
        } else {
            let res = b"1";
            Op::Sync(res.to_vec().into_boxed_slice())
        }
    })
}

fn op_serial_set_baud_rate(_interface: &mut dyn Interface, args: &mut [ZeroCopyBuf]) -> Op {
    let id = u32::from_str(std::str::from_utf8(args.get(0).unwrap()).unwrap()).unwrap();
    let baud_rate = u32::from_str(std::str::from_utf8(args.get(1).unwrap()).unwrap()).unwrap();
    PORTS.with(|map| {
        if let Some(target) = map.borrow_mut().get_mut(&id) {
            target.set_baud_rate(baud_rate).unwrap();
            let res = b"0";
            Op::Sync(res.to_vec().into_boxed_slice())
        } else {
            let res = b"1";
            Op::Sync(res.to_vec().into_boxed_slice())
        }
    })
}

fn op_serial_set_break(_interface: &mut dyn Interface, args: &mut [ZeroCopyBuf]) -> Op {
    let id = u32::from_str(std::str::from_utf8(args.get(0).unwrap()).unwrap()).unwrap();
    PORTS.with(|map| {
        if let Some(target) = map.borrow_mut().get_mut(&id) {
            target.set_break().unwrap();
            let res = b"0";
            Op::Sync(res.to_vec().into_boxed_slice())
        } else {
            let res = b"1";
            Op::Sync(res.to_vec().into_boxed_slice())
        }
    })
}

fn op_serial_clear_break(_interface: &mut dyn Interface, args: &mut [ZeroCopyBuf]) -> Op {
    let id = u32::from_str(std::str::from_utf8(args.get(0).unwrap()).unwrap()).unwrap();
    PORTS.with(|map| {
        if let Some(target) = map.borrow_mut().get_mut(&id) {
            target.clear_break().unwrap();
            let res = b"0";
            Op::Sync(res.to_vec().into_boxed_slice())
        } else {
            let res = b"1";
            Op::Sync(res.to_vec().into_boxed_slice())
        }
    })
}

fn op_serial_write_request_to_send(_interface: &mut dyn Interface, args: &mut [ZeroCopyBuf]) -> Op {
    let id = u32::from_str(std::str::from_utf8(args.get(0).unwrap()).unwrap()).unwrap();
    let level = u32::from_str(std::str::from_utf8(args.get(1).unwrap()).unwrap()).unwrap() == 1;
    PORTS.with(|map| {
        if let Some(target) = map.borrow_mut().get_mut(&id) {
            let mut res = b"1";
            let r = target.write_request_to_send(level);
            if r.is_err() {
                res = b"0";
            }
            Op::Sync(res.to_vec().into_boxed_slice())
        } else {
            let res = b"1";
            Op::Sync(res.to_vec().into_boxed_slice())
        }
    })
}

fn op_serial_write_data_terminal_ready(
    _interface: &mut dyn Interface,
    args: &mut [ZeroCopyBuf],
) -> Op {
    let id = u32::from_str(std::str::from_utf8(args.get(0).unwrap()).unwrap()).unwrap();
    let level = u32::from_str(std::str::from_utf8(args.get(1).unwrap()).unwrap()).unwrap() == 1;
    PORTS.with(|map| {
        if let Some(target) = map.borrow_mut().get_mut(&id) {
            let mut res = b"1";
            let r = target.write_data_terminal_ready(level);
            if r.is_err() {
                res = b"0";
            }
            Op::Sync(res.to_vec().into_boxed_slice())
        } else {
            let res = b"1";
            Op::Sync(res.to_vec().into_boxed_slice())
        }
    })
}

fn op_serial_bytes_to_read(_interface: &mut dyn Interface, args: &mut [ZeroCopyBuf]) -> Op {
    let id = u32::from_str(std::str::from_utf8(args.get(0).unwrap()).unwrap()).unwrap();
    PORTS.with(|map| {
        if let Some(target) = map.borrow_mut().get_mut(&id) {
            let res = target
                .bytes_to_read()
                .unwrap()
                .to_string()
                .as_bytes()
                .to_vec()
                .into_boxed_slice();
            Op::Sync(res)
        } else {
            let res = b"n";
            Op::Sync(res.to_vec().into_boxed_slice())
        }
    })
}

fn op_serial_bytes_to_write(_interface: &mut dyn Interface, args: &mut [ZeroCopyBuf]) -> Op {
    let id = u32::from_str(std::str::from_utf8(args.get(0).unwrap()).unwrap()).unwrap();
    PORTS.with(|map| {
        if let Some(target) = map.borrow_mut().get_mut(&id) {
            let res = target
                .bytes_to_write()
                .unwrap()
                .to_string()
                .as_bytes()
                .to_vec()
                .into_boxed_slice();
            Op::Sync(res)
        } else {
            let res = b"n";
            Op::Sync(res.to_vec().into_boxed_slice())
        }
    })
}

fn op_serial_write_all(_interface: &mut dyn Interface, args: &mut [ZeroCopyBuf]) -> Op {
    let id = u32::from_str(std::str::from_utf8(args.get(0).unwrap()).unwrap()).unwrap();
    let data = args.get(1).unwrap().to_vec();
    PORTS.with(|map| {
        if let Some(target) = map.borrow_mut().get_mut(&id) {
            target.write_all(data.as_slice()).unwrap();
            let res = b"1";
            Op::Sync(res.to_vec().into_boxed_slice())
        } else {
            let res = b"0";
            Op::Sync(res.to_vec().into_boxed_slice())
        }
    })
}

fn op_serial_write(_interface: &mut dyn Interface, args: &mut [ZeroCopyBuf]) -> Op {
    let id = u32::from_str(std::str::from_utf8(args.get(0).unwrap()).unwrap()).unwrap();
    let data = args.get(1).unwrap().to_vec();
    PORTS.with(|map| {
        if let Some(target) = map.borrow_mut().get_mut(&id) {
            target.write(data.as_slice()).unwrap();
            let res = b"1";
            Op::Sync(res.to_vec().into_boxed_slice())
        } else {
            let res = b"0";
            Op::Sync(res.to_vec().into_boxed_slice())
        }
    })
}

fn op_serial_read(_interface: &mut dyn Interface, args: &mut [ZeroCopyBuf]) -> Op {
    let id = u32::from_str(std::str::from_utf8(args.get(0).unwrap()).unwrap()).unwrap();
    let len = u32::from_str(std::str::from_utf8(args.get(1).unwrap()).unwrap()).unwrap();
    let mut data = vec![0; len as usize];
    PORTS.with(|map| {
        if let Some(target) = map.borrow_mut().get_mut(&id) {
            target.read(data.as_mut_slice()).unwrap();
            Op::Sync(data.into_boxed_slice())
        } else {
            let res = b"n";
            Op::Sync(res.to_vec().into_boxed_slice())
        }
    })
}

fn op_serial_clear(_interface: &mut dyn Interface, args: &mut [ZeroCopyBuf]) -> Op {
    let id = u32::from_str(std::str::from_utf8(args.get(0).unwrap()).unwrap()).unwrap();
    let buf = u32::from_str(std::str::from_utf8(args.get(1).unwrap()).unwrap()).unwrap();
    let buffer = if buf == 0 {
        ClearBuffer::Input
    } else if buf == 1 {
        ClearBuffer::Output
    } else {
        ClearBuffer::All
    };
    PORTS.with(|map| {
        if let Some(target) = map.borrow_mut().get_mut(&id) {
            target.clear(buffer).unwrap();
            let res = b"1";
            Op::Sync(res.to_vec().into_boxed_slice())
        } else {
            let res = b"0";
            Op::Sync(res.to_vec().into_boxed_slice())
        }
    })
}

fn op_serial_read_all(_interface: &mut dyn Interface, args: &mut [ZeroCopyBuf]) -> Op {
    let id = u32::from_str(std::str::from_utf8(args.get(0).unwrap()).unwrap()).unwrap();
    PORTS.with(|map| {
        if let Some(target) = map.borrow_mut().get_mut(&id) {
            let mut data = Vec::<u8>::new();
            target.read_to_end(&mut data).unwrap();
            Op::Sync(data.into_boxed_slice())
        } else {
            let res = b"n";
            Op::Sync(res.to_vec().into_boxed_slice())
        }
    })
}

fn op_serial_read_clear_to_send(_interface: &mut dyn Interface, args: &mut [ZeroCopyBuf]) -> Op {
    let id = u32::from_str(std::str::from_utf8(args.get(0).unwrap()).unwrap()).unwrap();
    PORTS.with(|map| {
        if let Some(target) = map.borrow_mut().get_mut(&id) {
            #[allow(unused_assignments)]
            let mut res = b"1";
            let r = target.read_clear_to_send();
            if r.is_err() {
                res = b"0";
            } else {
                res = if r.unwrap() == true { b"0" } else { b"1" };
            }
            Op::Sync(res.to_vec().into_boxed_slice())
        } else {
            let res = b"1";
            Op::Sync(res.to_vec().into_boxed_slice())
        }
    })
}

fn op_serial_read_data_set_ready(_interface: &mut dyn Interface, args: &mut [ZeroCopyBuf]) -> Op {
    let id = u32::from_str(std::str::from_utf8(args.get(0).unwrap()).unwrap()).unwrap();
    PORTS.with(|map| {
        if let Some(target) = map.borrow_mut().get_mut(&id) {
            #[allow(unused_assignments)]
            let mut res = b"1";
            let r = target.read_data_set_ready();
            if r.is_err() {
                res = b"0";
            } else {
                res = if r.unwrap() == true { b"0" } else { b"1" };
            }
            Op::Sync(res.to_vec().into_boxed_slice())
        } else {
            let res = b"1";
            Op::Sync(res.to_vec().into_boxed_slice())
        }
    })
}

fn op_serial_read_ring_indicator(_interface: &mut dyn Interface, args: &mut [ZeroCopyBuf]) -> Op {
    let id = u32::from_str(std::str::from_utf8(args.get(0).unwrap()).unwrap()).unwrap();
    PORTS.with(|map| {
        if let Some(target) = map.borrow_mut().get_mut(&id) {
            #[allow(unused_assignments)]
            let mut res = b"1";
            let r = target.read_ring_indicator();
            if r.is_err() {
                res = b"0";
            } else {
                res = if r.unwrap() == true { b"0" } else { b"1" };
            }
            Op::Sync(res.to_vec().into_boxed_slice())
        } else {
            let res = b"1";
            Op::Sync(res.to_vec().into_boxed_slice())
        }
    })
}

fn op_serial_read_carrier_detect(_interface: &mut dyn Interface, args: &mut [ZeroCopyBuf]) -> Op {
    let id = u32::from_str(std::str::from_utf8(args.get(0).unwrap()).unwrap()).unwrap();
    PORTS.with(|map| {
        if let Some(target) = map.borrow_mut().get_mut(&id) {
            #[allow(unused_assignments)]
            let mut res = b"1";
            let r = target.read_carrier_detect();
            if r.is_err() {
                res = b"0";
            } else {
                res = if r.unwrap() == true { b"0" } else { b"1" };
            }
            Op::Sync(res.to_vec().into_boxed_slice())
        } else {
            let res = b"1";
            Op::Sync(res.to_vec().into_boxed_slice())
        }
    })
}
