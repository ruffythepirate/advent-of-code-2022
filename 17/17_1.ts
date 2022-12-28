
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
  solidifiedShapes: Shape[];
  width: number;

  maxY: number

  constructor(width: number) {
    this.solidifiedShapes = [];
    this.width = width;
    this.maxY = 0;
  }

  getTowerHeight() {
    return this.maxY;
  }

  get(x: number, y: number): boolean {
    for (let shape of this.solidifiedShapes) {
      if (shape.overlaps(new Shape(point(x, y), [point(0, 0)]))) {
        return true;
      }
    }
    return false;
  }

  solidify(shape: Shape) {
    if(shape.bottomLeft.y + shape.height > this.maxY) {
      this.maxY = shape.bottomLeft.y + shape.height;
    }
    this.solidifiedShapes.push(shape);
  }

  canMoveLeft(shape: Shape) {
    if (shape.bottomLeft.x === 0) {
      return false;
    }
    for (let solidifiedShape of this.solidifiedShapes) {
      if (shape.overlaps(solidifiedShape, point(-1, 0))) {
        return false;
      }
    }
    return true;
  }

  canMoveRight(shape: Shape) {
    if (shape.bottomLeft.x + shape.width >= this.width) {
      return false;
    }
    for (let solidifiedShape of this.solidifiedShapes) {
      if (shape.overlaps(solidifiedShape, point(1, 0))) {
        return false;
      }
    }
    return true;
  }

  canMoveDown(shape: Shape) {
    if (shape.bottomLeft.y <= 0) {
      return false;
    }
    for (let solidifiedShape of this.solidifiedShapes) {
      if (shape.overlaps(solidifiedShape, point(0, -1))) {
        return false;
      }
    }
    return true;

  }
}

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
export function moveShape(shape: Shape, playfield: Playfield, windDirection: number): boolean {
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
    return true;
  } else {
    playfield.solidify(shape);
    return false;
  }
}

