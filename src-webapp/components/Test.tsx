import React, { FC, useEffect, useState } from 'react'
import { WasmComponentProps } from '../model/wasm-component-props'

const Home: FC<WasmComponentProps> = ({ wasmModule }) => {
  const [num, setNum] = useState<number>()
  useEffect(() => {
    wasmModule.console_log()
    setNum(wasmModule.add(1, 2))
    wasmModule.fetch_data().then(console.log).catch(console.error)
  }, [])
  return <div className={'navigate-buttons-container'}>{num}</div>
}

export default Home
