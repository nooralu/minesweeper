import { Difficulty } from "minsweeper"

type Props = {
  setDifficulty: (difficulty: Difficulty) => void,
};

const difficultyButtons = [
  {
    name: '简单',
    value: Difficulty.Easy,
  },
  {
    name: '普通',
    value: Difficulty.Medium,
  },
  {
    name: '困难',
    value: Difficulty.Hard,
  }
]


export default function ToolBar({ setDifficulty }: Props) {
  return (
    <div className='flex justify-between gap-1 mb-2 text-white'>
      {
        difficultyButtons.map((button) => (
          <button
            key={button.value}
            className='w-15 p-2 rounded bg-sky-500/75 hover:bg-sky-500'
            onClick={() => setDifficulty(button.value)}
          >
            {button.name}
          </button>
        ))
      }
    </div>
  );
}
