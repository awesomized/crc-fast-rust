// Verifies the WebAssembly module built from this directory: that it calculates correct
// checksums, and that it stays small.
//
// Usage: node check.mjs [path/to/crc64_nvme_wasm.wasm]

import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';

// the module is under 1 KiB; exceeding this limit means something got linked in which shouldn't
// have been (most likely the precomputed lookup tables, which are 32 KiB for a single algorithm)
const MAX_SIZE = 2048;

const defaultPath = fileURLToPath(
  new URL('target/wasm32-unknown-unknown/release/crc64_nvme_wasm.wasm', import.meta.url),
);

const bytes = readFileSync(process.argv[2] ?? defaultPath);
const { instance } = await WebAssembly.instantiate(bytes, {});
const { memory, crc64_nvme } = instance.exports;

// the module has no allocator, so grow its memory and use the new pages as the input buffer
const input = memory.grow(1) * 65536;

function checksum(crc, data) {
  new Uint8Array(memory.buffer, input, data.length).set(data);

  return BigInt.asUintN(64, crc64_nvme(crc, input, data.length));
}

function assertEqual(actual, expected, what) {
  if (actual !== expected) {
    throw new Error(`${what}: expected 0x${expected.toString(16)}, got 0x${actual.toString(16)}`);
  }
}

const encode = (text) => new TextEncoder().encode(text);

assertEqual(checksum(0n, encode('123456789')), 0xae8b14860a799888n, 'check value');
assertEqual(checksum(0n, encode('')), 0n, 'empty input');
assertEqual(checksum(0n, encode('hello, world!')), 0xf8046e40c403f1d0n, 'hello, world!');
assertEqual(checksum(0n, new Uint8Array(4096)), 0x6482d367eb22b64en, '4096 zero bytes');
assertEqual(checksum(0n, new Uint8Array(4096).fill(255)), 0xc0ddba7302eca3acn, '4096 0xff bytes');

// streaming in pieces must match a single pass
assertEqual(
  checksum(checksum(0n, encode('1234')), encode('56789')),
  0xae8b14860a799888n,
  'streaming',
);

if (bytes.length > MAX_SIZE) {
  throw new Error(`module is ${bytes.length} bytes, which exceeds the ${MAX_SIZE} byte limit`);
}

console.log(`ok: ${bytes.length} bytes`);
