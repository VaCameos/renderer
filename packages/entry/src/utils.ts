import localforage from 'localforage';
import { initSync } from '@renderer/core';
import wasmUrl from '@renderer/core/wasm?url';

const getInstance = async () => {
  let buf:BufferSource | null = await localforage.getItem('wasm');
  if(!buf) {
      const rep = await fetch(wasmUrl);
      buf = await rep.arrayBuffer();
  };
  
  const module = await WebAssembly.compile(buf);
  const ctx = initSync(module);
  return ctx;
}
export const useRenderer = async () => {
  try {
    const ctx = await getInstance()
    ctx.greet();
  } catch (err) {
    console.error('Failed to initialize renderer:', err);
  }
};