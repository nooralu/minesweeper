interface ITile {
  revealed: boolean;
  adjacentMines: number;
  mine?: boolean;
  flagged?: boolean;
}

export default ITile;
