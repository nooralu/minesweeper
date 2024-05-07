import { useEffect, useState } from 'react';
import TileComponent from './components/Tile';
import { Board, Tile, Difficulty, GameState } from 'minsweeper';
import ToolBar from './components/ToolBar';
import StateBar from './components/StateBar';

function App() {
  const [board, setBoard] = useState<Board | null>(null);
  const [tiles, setTiles] = useState<Tile[]>([]);
  const [difficulty, setDifficulty] = useState(Difficulty.Easy);
  const [width, setWidth] = useState(0);
  const [state, setState] = useState<GameState>(GameState.Ready);
  const [duration, setDuration] = useState(0);

  useEffect(() => {
    newGame();
  }, [difficulty]);

  function newGame() {
    const board = new Board(difficulty);
    setBoard(board);
    setTiles(board?.getTiles() ?? []);
    setWidth(board.getWidth());
    setState(board?.getState() ?? GameState.Ready);
  }

  useEffect(() => {
    if (state === GameState.Ready) {
      setDuration(0);
    } else if (state === GameState.Playing) {
      const interval = setInterval(() => {
        setDuration((prev) => prev + 1);
      }, 1000);
      return () => clearInterval(interval);
    }
  }, [state]);

  function handleClik(index: number, left: boolean = true) {
    board?.onClick(index, left);
    setTiles(board?.getTiles() ?? []);
    setState(board?.getState() ?? GameState.Ready);
  }

  function handleChangeDifficulty(target: Difficulty) {
    if (difficulty === target) {
      newGame();
    } else {
      setDifficulty(target);
    }
  }

  return (
    <>
      <div className='w-full pt-10 pb-20 flex flex-col justify-start items-center'>
        <ToolBar setDifficulty={handleChangeDifficulty} />
        <StateBar duration={duration} difficulty={difficulty} state={state} />
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
