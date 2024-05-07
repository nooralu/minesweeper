import { Difficulty, GameState } from "minsweeper";

export default function StateBar({
  duration,
  difficulty,
  state,
}: {
  duration: number;
  difficulty: Difficulty;
  state: GameState;
}) {
  let content = null;
  if (state === GameState.Playing || state === GameState.Ready) {
    content = <>
      <div className="text-xl font-bold">Duration</div>
      <div className="text-xl font-bold">{duration}s</div>
      <div className="text-xl font-bold">Difficulty</div>
      <div className="text-xl font-bold">{difficulty}</div>
    </>;
  } else {
    content = <div className="text-xl font-bold">{state === GameState.Won ? "You won!" : "You lost!"}</div>;
  }
  return (
    <div className="flex items-center justify-between gap-4">
      {content}
    </div>
  )
}
