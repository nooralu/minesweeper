import boom from '../assets/boom.svg';
import flag from '../assets/flag.svg';
import { Tile as ITile } from 'minsweeper';

type Props = {
  tile: ITile,
  onLeftClick: (index: number) => void,
  onRightClick: (index: number) => void,
}

export default function Tile({tile, onLeftClick, onRightClick} : Props) {
  let content = null;
  if (tile.isFlagged()) {
    content = <img src={flag} width='50%'></img>;
  } else if (tile.isRevealed()) {
    if (tile.hasMine()) {
      content = <img src={boom} width='50%'></img>;
    } else {
      content = <div>{tile.getAdjacentMines()}</div>;
    }
  }

  return (
    <button 
      onClick={() => onLeftClick(tile.getIndex())}
      onContextMenu={(e) => {
        onRightClick(tile.getIndex());
        e.preventDefault();
      }}
      className='w-12 h-12 bg-slate-300/75 rounded flex justify-center items-center select-none hover:bg-slate-300'
      >
      {content}
    </button>
  );
}
