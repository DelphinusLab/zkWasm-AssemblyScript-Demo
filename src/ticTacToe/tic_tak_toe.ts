enum Side {
  None = 0,
  Red = 1,
  Blue = 2,
}

class GridStep {
  public color: Side;
  public x: u8;
  public y: u8;
  public static FromI64(num: i64): GridStep {
    var step = new GridStep();
    step.color = (num & 0xff) as Side;
    step.x = ((num >> 8) & 0xff) as u8;
    step.y = ((num >> 16) & 0xff) as u8;
    return step;
  }
}

function Line(grid: StaticArray<Side>, x: u8, y: u8, x2: u8, y2: u8, x3: u8, y3: u8): Side {
  var i1 = grid[y * 3 + x];
  var i2 = grid[y2 * 3 + x2];
  var i3 = grid[y3 * 3 + x3];

  if (i1 == 0 || i2 == 0 || i3 == 0) return Side.None;
  if (i1 == i2 && i1 == i3) return i1;

  return Side.None;
}

function IsWin(grid: StaticArray<Side>): Side {
  for (var i:u8 = 0; i < 3; i++) {
    // horizontal
    var sideH = Line(grid,0, i, 1, i, 2, i);
    if (sideH != Side.None) return sideH;

    // vertical
    var sideV = Line(grid,i, 0, i, 1, i, 2);
    if (sideV != Side.None) return sideV;
  }

  // diagonal
  var sided1 = Line(grid,0, 0, 1, 1, 2, 2);
  if (sided1 != Side.None) return sided1;

  // diagonal
  var sided2 = Line(grid,2, 0, 1, 1, 0, 2);
  if (sided2 != Side.None) return sided2;

  return Side.None;
}

export function ticTacToe(input: i64[]): i64[] {
  var grid = new StaticArray<Side>(9);
  var step: GridStep;

  for (var i = 0; i < input.length; i++) {
    step = GridStep.FromI64(input[i]);
    grid[step.y * 3 + step.x] = step.color;
  }

  var winner: Side = IsWin(grid);
  var output: i64[] = []
  output.push(winner as i64);
  return output;
}
