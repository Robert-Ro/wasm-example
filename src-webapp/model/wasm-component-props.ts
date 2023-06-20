import * as WasmModule from '../../pkg/native_stock'

export type WASMModule = typeof WasmModule
export interface WasmComponentProps {
  wasmModule: WASMModule
}
