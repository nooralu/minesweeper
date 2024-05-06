import { useEffect, useState } from 'react';
import TileComponent from './components/Tile';
import { Board, Tile } from 'minsweeper';

const WIDTH = 4;
const HEIGHT = 4;

function App() {
  const [board, setBoard] = useState<Board | null>(null);
  const [tiles, setTiles] = useState<Tile[]>([]);

  useEffect(() => {
    const board = new Board(WIDTH, HEIGHT);
    board.genrateMines(3);
    setBoard(board);
    setTiles(board?.getTiles() ?? []);
  }, []);

  function handleClik(index: number, left: boolean = true) {
    board?.onClick(index, left);
    setTiles(board?.getTiles() ?? []);
  }

  return (
    <>
      <div className='w-screen h-screen flex justify-center items-center'>
        <div className={`grid grid-cols-4 gap-1`}>
          {
            tiles
            .map((tile, index) => (
              <TileComponent
                tile={tile}
                key={index}
                onLeftClick={(index) => handleClik(index)}
                onRightClick={(index) =>handleClik(index, false)}
              />
            ))
          }
        </div>
      </div>
    </>
  )
}

export default App
