
export class Point {
    x: number;
    y: number;

    constructor(x: number, y: number) {
      this.x = x;
      this.y = y;
    } 

    overlaps(other: Point) {
      return this.x === other.x && this.y === other.y;
    }

    minus(other: Point): Point {
      return this.plus(other.negate());
    }

    negate(): Point {
      return new Point(-this.x, -this.y);
    }

    plus(other: Point): Point {
      return new Point(this.x + other.x, this.y + other.y);
    }

    clone(): Point {
      return new Point(this.x, this.y);
    }
}

function point(x: number, y: number): Point {
  return new Point(x, y);
}


export class Shape {
    points: Point[];

    bottomLeft: Point;

    width: number;
    height: number;

    constructor(bottomLeft: Point, points: Point[]) {
      this.bottomLeft = bottomLeft;
      this.points = points;

      let minX = Infinity;
      let minY = Infinity;
      let maxX = -Infinity;
      let maxY = -Infinity;
      for (let point of points) {
        minX = Math.min(minX, point.x);
        minY = Math.min(minY, point.y);
        maxX = Math.max(maxX, point.x);
        maxY = Math.max(maxY, point.y);
      }
      this.width = maxX - minX + 1;
      this.height = maxY - minY + 1;

    }

    overlaps(other: Shape, offset: Point = point(0, 0)): boolean {
      let finalOffset = this.bottomLeft.minus(other.bottomLeft).plus(offset);
      for (let point of this.points) {
        const translatedPoint = point.plus(finalOffset);
        for (let otherPoint of other.points) {
          if (translatedPoint.overlaps(otherPoint)) {
            return true;
          }
        }
      }
      return false;
    }

    clone(): Shape {
      return new Shape(this.bottomLeft.clone(), this.points.map(p => p.clone()));
    }
}

export class Playfield {
  groundShape: boolean[][] = [];
  width: number;

  maxY: number
  minY: number = 0;

  constructor(width: number) {
    this.width = width;
    this.maxY = 0;
  }

  getTowerHeight() {
    return this.maxY;
  }

  solidify(shape: Shape) {
    if(shape.bottomLeft.y + shape.height > this.maxY) {
      this.maxY = shape.bottomLeft.y + shape.height;
    }
    for (let point of shape.points) {
      const x = point.x + shape.bottomLeft.x;
      const y = point.y + shape.bottomLeft.y;
      const adjustedY = y - this.minY;
      while (adjustedY >= this.groundShape.length) {
        this.groundShape.push(new Array(this.width).fill(false));
      }
      this.groundShape[adjustedY][x] = true;
    }
  }

  isPointSolid(x: number, y: number) {
    const adjustedY = y - this.minY;
    if (adjustedY < 0 || adjustedY >= this.groundShape.length) {
      return false;
    }
    return this.groundShape[adjustedY][x];
  }

  /**
   * Checks if the first row with any solid block seen from the top contains all solid blocks.
   */
  hasTopFlatRow(): boolean {
    if (this.groundShape.length === 0) {
      return false;
    }
    for (let y = this.groundShape.length - 1; y >= 0; y--) {
      if(rowHasAnySolidBlock(this.groundShape[y])) {
        return rowHasAllSolidBlocks(this.groundShape[y]);
      }
    }
    return false;
  }

  /**
   * Checks if there is a bridge of solid rock available above present minY and returns minY in this bridge.
   */
  findMinY(): number | undefined {
    let adjustedHighestYInFirstColumn = this.getFirstColumnMaxY();
    if (adjustedHighestYInFirstColumn !== undefined) {
      let adjustedMinY = this.recursiveMinY(adjustedHighestYInFirstColumn, 0, adjustedHighestYInFirstColumn, 0);
      if (adjustedMinY !== null) {
        return adjustedMinY + this.minY;
      }
    }
    return undefined;
  }

  getFirstColumnMaxY(): number | undefined {
    for (let y = this.groundShape.length - 1; y > 0; y--) {
      if (this.groundShape[y][0]) {
        return y;
      }
    }
  }


  recursiveMinY(adjustedY: number, x: number, minY: number, prevDiffY: number): number | null {
    let newMinY = Math.min(minY, adjustedY);
    if (adjustedY < 0 || adjustedY >= this.groundShape.length || !this.groundShape[adjustedY][x]) {
      return null;
    }
    if(x === this.width - 1) {
      return newMinY;
    }
    let bestMinY = this.recursiveMinY(adjustedY, x + 1, newMinY, 0);
    if(prevDiffY !== -1) {

      let minYAfterYPlus1 = this.recursiveMinY(adjustedY - 1, x, newMinY, 1);
      bestMinY = Math.max(handleNullable(bestMinY, -1), 
                         handleNullable(minYAfterYPlus1, -1));
    }
    if(prevDiffY !== 1) {
      bestMinY = Math.max(handleNullable(bestMinY, -1), 
                          handleNullable(this.recursiveMinY(adjustedY-1, x, newMinY, -1), -1));
    }

    if (bestMinY === null || bestMinY === -1) {
      return null;
    }
    return Math.min(bestMinY ? bestMinY : Infinity, newMinY);
  }

  cleanupHiddenRows() {
    let newMinY = this.findMinY();
    if (newMinY !== undefined) {
      this.minY = newMinY;
      this.groundShape = this.groundShape.slice(this.minY);
    }
  }

  printRows(minY: number, maxY: number, markedRow?: number) {
    for (let y = maxY - this.minY; y >= minY - this.minY; y--) {
      const rowString = this.groundShape[y].map(b => b ? '#' : '.').join('');
      if(y === markedRow) {
        console.log(rowString + ' <-');
      } else {
        console.log(rowString);
      }
    }
  }

  canMoveLeft(shape: Shape) {
    if (shape.bottomLeft.x === 0) {
      return false;
    }
    for (let point of shape.points) {
      if (this.isPointSolid(shape.bottomLeft.x + point.x - 1, shape.bottomLeft.y + point.y)) {
        return false;
      }
    }
    return true;
  }

  canMoveRight(shape: Shape) {
    if (shape.bottomLeft.x + shape.width >= this.width) {
      return false;
    }
    for (let point of shape.points) {
      if (this.isPointSolid(shape.bottomLeft.x + point.x + 1, shape.bottomLeft.y + point.y)) {
        return false;
      }
    }
    return true;
  }

  canMoveDown(shape: Shape) {
    if (shape.bottomLeft.y <= this.minY) {
      return false;
    }
    for (let point of shape.points) {
      if (this.isPointSolid(shape.bottomLeft.x + point.x, shape.bottomLeft.y + point.y - 1)) {
        return false;
      }
    }
    return true;

  }
}

function handleNullable(value: number | null, defaultValue: number): number {
  if (value === null) {
    return defaultValue;
  }
  return value as number;
}

function rowHasAnySolidBlock(row: boolean[]) {
  for (let solid of row) {
    if (solid) {
      return true;
    }
  }
  return false;
}

function rowHasAllSolidBlocks(row: boolean[]) {
  for (let solid of row) {
    if (!solid) {
      return false;
    }
  }
  return true;
}

// there are 5 shapes.
const shapes = [
  // ####
  createShape([ 
    point(0, 0),
    point(1, 0),
    point(2, 0),
    point(3, 0),
  ]),
  //.#.
  //###
  //.#.
  createShape([
    point(1,0),
    point(0,1),
    point(1,1),
    point(2,1),
    point(1,2),
  ]),
  //..#
  //..#
  //###
  createShape([
    point(0,0),
    point(1,0),
    point(2,0),
    point(2,1),
    point(2,2),
  ]),
  //#
  //#
  //#
  //#
  createShape([
    point(0,0),
    point(0,1),
    point(0,2),
    point(0,3),
  ]),
  //##
  //##
  createShape([
    point(0, 0),
    point(1, 0),
    point(0, 1),
    point(1, 1),
  ]),
]

function createShape(points: Point[]): Shape {
  return new Shape(new Point(0,0), points);
}

export function getShape(shapeNumber: number, towerHeight: number): Shape {
  let index = shapeNumber % shapes.length;
  let shape = shapes[index].clone();
  shape.bottomLeft = point(2, towerHeight + 3);
  return shape.clone()
}

/**
 * Returns if the shape was solidified.
 */
export function moveShape(shape: Shape, playfield: Playfield, windDirection: number): Point | undefined {
  if(windDirection > 0) {
    if (playfield.canMoveRight(shape)) {
      shape.bottomLeft = shape.bottomLeft.plus(point(1, 0));
    }
  } else if (windDirection < 0) {
    if (playfield.canMoveLeft(shape)) {
      shape.bottomLeft = shape.bottomLeft.plus(point(-1, 0));
    }
  }

  if (playfield.canMoveDown(shape)) {
    shape.bottomLeft = shape.bottomLeft.plus(point(0, -1));
    return undefined;
  } else {
    playfield.solidify(shape);
    return shape.bottomLeft;
  }
}

