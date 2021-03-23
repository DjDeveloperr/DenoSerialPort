Deno.openPlugin("./native/target/debug/native.dll");
const {
  op_available_ports,
  op_serial_new,
  op_serial_close,
  op_serial_read,
  op_serial_write,
  op_serial_read_all,
  op_serial_write_all,
  op_serial_set_baud_rate,
  op_serial_set_break,
  op_serial_clear_break,
  op_serial_bytes_to_read,
  op_serial_bytes_to_write,
  op_serial_write_data_terminal_ready,
  op_serial_write_request_to_send,
  op_serial_read_clear_to_send,
  op_serial_read_data_set_ready,
  op_serial_read_ring_indicator,
  op_serial_read_carrier_detect,
  op_serial_clear,
}: { [name: string]: number } = (Deno as any).core.ops();

const encoder = new TextEncoder();
const decoder = new TextDecoder("utf-8");

function dispatch(id: number, ...args: any[]): Uint8Array {
  return (Deno as any).core.dispatch(
    id,
    ...args
      .map((e) =>
        typeof e === "object"
          ? e instanceof Uint8Array
            ? e
            : JSON.stringify(e)
          : String(e)
      )
      .map((e) => (typeof e == "object" ? e : encoder.encode(e)))
  );
}

function dispatchString(id: number, ...args: any[]) {
  return decoder.decode(dispatch(id, ...args));
}

function dispatchNumber(id: number, ...args: any[]) {
  return Number(dispatchString(id, ...args));
}

function dispatchJSON(id: number, ...args: any[]) {
  return JSON.parse(dispatchString(id, ...args));
}

export function available_ports() {
  return dispatchJSON(op_available_ports);
}

export function serial_new(path: string, baud_rate: number) {
  return dispatchNumber(op_serial_new, path, baud_rate);
}

export function serial_bytes_to_read(id: number) {
  return dispatchNumber(op_serial_bytes_to_read, id);
}

export function serial_bytes_to_write(id: number) {
  return dispatchNumber(op_serial_bytes_to_write, id);
}

export function serial_write_all(id: number, data: string | Uint8Array) {
  return dispatchNumber(op_serial_write_all, id, data);
}

export function serial_write(id: number, data: string | Uint8Array) {
  return dispatchNumber(op_serial_write, id, data);
}

export function serial_read_all(id: number) {
  return dispatch(op_serial_read_all, id);
}

export function serial_read(id: number, len: number) {
  if (len <= 0) return new Uint8Array(0);
  return dispatch(op_serial_read, id, len);
}

export enum SerialClearType {
  Input,
  Output,
  All,
}

export function serial_clear(id: number, type: SerialClearType) {
  return dispatchNumber(op_serial_clear, id, type);
}

export function serial_close(id: number) {
  dispatch(op_serial_close, id);
}

export function serial_set_break(id: number) {
  dispatch(op_serial_set_break, id);
}

export function serial_clear_break(id: number) {
  dispatch(op_serial_clear_break, id);
}

export function serial_set_baud_rate(id: number, rate: number) {
  dispatch(op_serial_set_baud_rate, id, rate);
}
