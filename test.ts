import { Serial, SerialClearType } from "./mod.ts";

console.log(Serial.availablePorts());

const serial = new Serial("COM7", 9600);
serial.clear(SerialClearType.All);

serial.write(new Uint8Array([0x00, 13, 0]));
serial.write(new Uint8Array([0x01, 13, 1]));
serial.write(new Uint8Array([0x02, 2]));
serial.write(new Uint8Array([0x01, 13, 0]));
const data = serial.read(2);

console.log("Temperature: " + data[0] + " deg C");
console.log("Humidity: " + data[1] + "%");
