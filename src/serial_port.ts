import { available_ports, SerialClearType, serial_bytes_to_read, serial_bytes_to_write, serial_clear, serial_new, serial_read, serial_read_all, serial_write } from "./ops.ts";

export interface SerialUsbInfo {
    vendorId: number;
    productId: number;
    serialNumber: string;
    manufacturer: string;
    product: string;
}

export interface SerialPortInfo {
    name: string;
    type: number;
    usbInfo?: SerialUsbInfo;
}

export class Serial {
    #rid: number;

    static availablePorts(): SerialPortInfo[] {
        return available_ports().map((e: any) => {
            const data: SerialPortInfo = {
                name: e.name,
                type: e.type,
            };

            if (e.usb_info) data.usbInfo = {
                vendorId: e.usb_info.vid,
                product: e.usb_info.pid,
                manufacturer: e.usb_info.manufacturer,
                serialNumber: e.usb_info.serial_number,
                productId: e.usb_info.product,
            };

            return data;
        })
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
}