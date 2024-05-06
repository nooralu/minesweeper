import boom from '../assets/boom.svg';
import flag from '../assets/flag.svg';
import ITile from '../types/Tile';

export default function Tile({ tile }: { tile: ITile }) {
  let content = null;
  if (tile.flagged) {
    content = <img src={flag} width='50%'></img>;
  } else if (tile.revealed) {
    if (tile.mine) {
      content = <img src={boom} width='50%'></img>;
    } else {
      content = <div>{tile.adjacentMines}</div>;
    }
  }

  return (
    <button className='w-12 h-12 bg-slate-300/75 rounded flex justify-center items-center select-none hover:bg-slate-300'>
      {content}
    </button>
  );
}
