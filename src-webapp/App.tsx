import React, { useEffect, useState } from 'react'
import './App.css'
import Test from './components/Test'
import { WASMModule } from './model/wasm-component-props'

const App = () => {
  const [wasmModule, setWasmModule] = useState<WASMModule>(undefined)

  useEffect(() => {
    import('../pkg/native_stock').then((module: WASMModule) => {
      setWasmModule(module)
    })
  }, [])
  console.log(wasmModule)
  return wasmModule ? (
    <div className={'main-container'}>
      <Test wasmModule={wasmModule} />
    </div>
  ) : (
    <div>Loading WASM module...</div>
  )
}

export default App
