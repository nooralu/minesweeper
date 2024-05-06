import { useState } from 'react'

const WIDTH = 4;
const HEIGHT = 4;

function App() {
  const tiles = [];
  for (let i = 0; i < HEIGHT; i += 1) {
    for (let j = 0; j < WIDTH; j += 1) {
      tiles.push(<button className='bg-slate-300/75 rounded w-12 h-12' key={i * WIDTH + j}></button>)
    }
  }

  return (
    <>
      <div className='w-screen h-screen flex justify-center items-center'>
        <div className={`grid grid-cols-${WIDTH} gap-1`}>
          {
            tiles
          }
        </div>
      </div>
    </>
  )
}

export default App
