import { useEffect, useState } from 'react';
import TileComponent from './components/Tile';
import { Board, Tile, Difficulty } from 'minsweeper';

function App() {
  const [board, setBoard] = useState<Board | null>(null);
  const [tiles, setTiles] = useState<Tile[]>([]);
  const [difficulty, setDifficulty] = useState(Difficulty.Easy);
  const [width, setWidth] = useState(0);

  useEffect(() => {
    const board = new Board(difficulty);
    setBoard(board);
    setTiles(board?.getTiles() ?? []);
    setWidth(board.get_width());
  }, [width, difficulty]);

  function handleClik(index: number, left: boolean = true) {
    board?.onClick(index, left);
    setTiles(board?.getTiles() ?? []);
  }

  return (
    <>
      <div className='w-screen h-screen pt-10 pb-20 flex flex-col justify-start items-center'>
        <div className='flex justify-between gap-1 mb-2 text-white'>
          <button
            className='w-15 p-2 rounded bg-sky-500/75 hover:bg-sky-500'
            onClick={() => setDifficulty(Difficulty.Easy)}
          >
            简单
          </button>
          <button
            className='w-15 p-2 rounded bg-sky-500/75 hover:bg-sky-500'
            onClick={() => setDifficulty(Difficulty.Medium)}
          >
            普通
          </button>
          <button
            className='w-15 p-2 rounded bg-sky-500/75 hover:bg-sky-500'
            onClick={() => setDifficulty(Difficulty.Hard)}
          >
            困难
          </button>
        </div>
        <div className='grid gap-1' style={{ gridTemplateColumns: `repeat(${width}, minmax(0, 1fr))` }}>
          {
            tiles
              .map((tile, index) => (
                <TileComponent
                  tile={tile}
                  key={`${difficulty}-${index}`}
                  onLeftClick={(index) => handleClik(index)}
                  onRightClick={(index) => handleClik(index, false)}
                />
              ))
          }
        </div>
      </div>
    </>
  )
}

export default App
