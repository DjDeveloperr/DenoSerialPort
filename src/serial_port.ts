import {
  available_ports,
  SerialClearType,
  serial_bytes_to_read,
  serial_bytes_to_write,
  serial_clear,
  serial_clear_break,
  serial_close,
  serial_new,
  serial_read,
  serial_read_all,
  serial_read_carrier_detect,
  serial_read_clear_to_send,
  serial_read_data_set_ready,
  serial_read_ring_indicator,
  serial_set_baud_rate,
  serial_set_break,
  serial_write,
  serial_write_data_terminal_ready,
  serial_write_request_to_send,
} from "./ops.ts";

export enum SerialPortType {
  Pci = 1,
  Usb,
  Bluetooth,
  Unknown,
}

export interface SerialUsbInfo {
  vendorId: number;
  productId: number;
  serialNumber: string;
  manufacturer: string;
  product: string;
}

export interface SerialPortInfo {
  name: string;
  type: SerialPortType;
  usbInfo?: SerialUsbInfo;
}

export class Serial {
  #rid: number;

  static availablePorts(): SerialPortInfo[] {
    return available_ports().map((e: any) => {
      const data: SerialPortInfo = {
        name: e.name,
        type: e.port_type,
      };

      if (e.usb_info)
        data.usbInfo = {
          vendorId: e.usb_info.vid,
          product: e.usb_info.pid,
          manufacturer: e.usb_info.manufacturer,
          serialNumber: e.usb_info.serial_number,
          productId: e.usb_info.product,
        };

      return data;
    });
  }

  get rid() {
    return this.#rid;
  }

  constructor(name: string, baudRate: number) {
    this.#rid = serial_new(name, baudRate);
  }

  clear(type: SerialClearType) {
    serial_clear(this.rid, type);
  }

  read(len: number) {
    return serial_read(this.#rid, len);
  }

  readToEnd() {
    return serial_read_all(this.rid);
  }

  write(data: Uint8Array | string) {
    serial_write(this.rid, data);
  }

  getBytesToRead() {
    return serial_bytes_to_read(this.rid);
  }

  getBytesToWrite() {
    return serial_bytes_to_write(this.rid);
  }

  setBreak() {
    return serial_set_break(this.rid);
  }

  clearBreak() {
    return serial_clear_break(this.rid);
  }

  setBaudRate(rate: number) {
    return serial_set_baud_rate(this.rid, rate);
  }

  writeDataTerminalReady(level: number) {
    serial_write_data_terminal_ready(this.rid, level);
  }

  writeRequestToSend(level: number) {
    serial_write_request_to_send(this.rid, level);
  }

  readCarrierDetect() {
    return serial_read_carrier_detect(this.rid);
  }

  readClearToSend() {
    return serial_read_clear_to_send(this.rid);
  }

  readDataSetReady() {
    return serial_read_data_set_ready(this.rid);
  }

  readRingIndicator() {
    return serial_read_ring_indicator(this.rid);
  }

  close() {
    return serial_close(this.rid);
  }
}
