import { Point, Shape, getShape, Playfield, moveShape, iteratePlayfield, solveProblem, IterationState, findMatchingIterationState } from './17_2';

describe("Point", () => {
  it("should have x and y coordinates", () => {
    const point = new Point(1, 2);
    expect(point.x).toBe(1);
    expect(point.y).toBe(2);
  });

  it('should overlap when same', () => {
    const point = new Point(1, 2);
    expect(point.overlaps(new Point(1, 2))).toBe(true);
    expect(point.overlaps(new Point(2, 2))).toBe(false);
  });
});

describe("Shape", () => {
  it("should have a number of points", () => {
    const shape = new Shape(new Point(0,0), [new Point(1, 2), new Point(3, 4)]);
    expect(shape.points.length).toBe(2);
  });

  it("should have a bottom left corner", () => {
    const shape = new Shape(new Point(1,2), [new Point(1, 2), new Point(3, 4)]);
    expect(shape.bottomLeft.x).toBe(1);
    expect(shape.bottomLeft.y).toBe(2);
  });

  it("should have an overlap", () => {
    const shape = new Shape(new Point(1,2), [new Point(1, 2), new Point(3, 4)]);
    expect(shape.overlaps(new Shape(new Point(1,2), [new Point(1, 2)]))).toBe(true);
    expect(shape.overlaps(new Shape(new Point(2,2), [new Point(1, 2), new Point(3, 4)]))).toBe(false);
  });

  it('should calculate width', () => {
    const shape = new Shape(new Point(1,2), [new Point(1, 2), new Point(3, 4)]);
    expect(shape.width).toBe(3);
  });

  it('should calculate height', () => {
    const shape = new Shape(new Point(1,2), [new Point(1, 2), new Point(3, 4)]);
    expect(shape.height).toBe(3);
  });
});

describe("getShape", () => {
  it("should return a shape", () => {
    const shape = getShape(0, 0);
    expect(shape).toBeInstanceOf(Shape);
  });

  it('should calculate bottomLeft.y as towerHeight + 3', () => {
    const shape = getShape(0, 0);
    expect(shape.bottomLeft.y).toBe(3);
  });

  it('should calculate bottomLeft.x as 2', () => {
    const shape = getShape(0, 0);
    expect(shape.bottomLeft.x).toBe(2);
  });
});

describe("iteratePlayfield", () => {
  it("should call callback when new rock happens after exhausisting wind directions", () => {
    const playfield = new Playfield(7);
    const callback = jest.fn();

    callback.mockReturnValue(0);

    iteratePlayfield(playfield, "<<<", 3, callback);

    expect(callback).toBeCalledTimes(2);
  });
});

describe('Playfield', () => {

  it('should get top y for each column', () => {
    const playfield = new Playfield(7);
    playfield.solidify(new Shape(new Point(0, 0), [new Point(0, 0), new Point(1, 0)]));
    const topY = playfield.getTopYForEachColumn();
    expect(topY).toEqual([1, 1, 0, 0, 0, 0, 0]);
  });

  it('should have a width', () => {
    const playfield = new Playfield(7);
    expect(playfield.width).toBe(7);
  });

  it('should find minY', () => {
    const playfield = new Playfield(4);
    playfield.solidify(new Shape(new Point(0, 2), [new Point(0, 0), new Point(1, 0), new Point(2, 0), new Point(3, 0)]));
    expect(playfield.findMinY()).toBe(2);
  });

  it('should have minY undefined when not found', () => {
    const playfield = new Playfield(4);
    expect(playfield.findMinY()).toBeUndefined();
  });

  it('should have minY undefined when no shape', () => {
    const playfield = new Playfield(4);
    playfield.solidify(new Shape(new Point(0, 2), [new Point(0, 0), new Point(1, 0), new Point(2, 0), new Point(3, 1)]));
    let minY = playfield.findMinY();
    expect(minY).toBeUndefined();
  });

  it('should find first column maxY', () => {
    const playfield = new Playfield(4);
    playfield.solidify(new Shape(new Point(0, 2), [new Point(0, 0), new Point(1, 0), new Point(2, 0), new Point(3, 0)]));
    expect(playfield.getFirstColumnMaxY()).toBe(2);
  });

  it('should return undefined when no firstColumnMaxY', () => {
    const playfield = new Playfield(4);
    expect(playfield.getFirstColumnMaxY()).toBeUndefined();
  });

  it('should cleanupHiddenRows', () => {
    const playfield = new Playfield(4);
    playfield.solidify(new Shape(new Point(0, 2), [new Point(0, 0), new Point(1, 0), new Point(2, 0), new Point(3, 0)]));
    playfield.solidify(new Shape(new Point(0, 3), [new Point(0, 0), new Point(1, 0), new Point(2, 0), new Point(3, 0)]));
    playfield.cleanupHiddenRows();
    expect(playfield.minY).toBe(3);
  });

  it('should be able to check hasTopFlatRow', () => {
    const playfield = new Playfield(4);
    playfield.solidify(new Shape(new Point(0, 2), [new Point(0, 0), new Point(1, 0), new Point(2, 0), new Point(3, 0)]));
    expect(playfield.hasTopFlatRow()).toBe(true);
  });

  it('should say false to hasTopFlatRow when no top flat row', () => {
    const playfield = new Playfield(4);
    playfield.solidify(new Shape(new Point(0, 2), [new Point(0, 0), new Point(1, 0), new Point(2, 0), new Point(3, 0)]));
    playfield.solidify(new Shape(new Point(0, 3), [new Point(0, 0), new Point(1, 0), new Point(2, 0)]));
    expect(playfield.hasTopFlatRow()).toBe(false);
  });

  it('should handle cleanup if minY > 0', () => {
    const playfield = new Playfield(4);
    playfield.solidify(new Shape(new Point(0, 2), [new Point(0, 0), new Point(1, 0), new Point(2, 0), new Point(3, 0)]));
    playfield.cleanupHiddenRows();
    expect(playfield.minY).toBe(2);
    playfield.solidify(new Shape(new Point(0, 3), [new Point(0, 0), new Point(1, 0), new Point(2, 0), new Point(3, 0)]));
    playfield.cleanupHiddenRows();
    expect(playfield.minY).toBe(3);
  });

  it('should return undefined when no minY above lowest row', () => {
    const playfield = new Playfield(4);
    expect(playfield.findMinY()).toBeUndefined();
  });

  it('should see if shape can move left', () => {
    const playfield = new Playfield(7);
    const shape = getShape(0, 0);
    shape.bottomLeft.y = 0;
    shape.bottomLeft.x = 1;
    expect(playfield.canMoveLeft(shape)).toBe(true);
    shape.bottomLeft.x = 0;
    expect(playfield.canMoveLeft(shape)).toBe(false);
    shape.bottomLeft.x = 1;
    playfield.solidify(new Shape(new Point(0, 0), [new Point(0, 0)]));
    expect(playfield.canMoveLeft(shape)).toBe(false);
  });

  it('should see if shape can move right', () => {
    const playfield = new Playfield(7);
    const shape = new Shape(new Point(0, 0), [new Point(0, 0)]);
    shape.bottomLeft.y = 0;
    shape.bottomLeft.x = 2;
    expect(playfield.canMoveRight(shape)).toBe(true);
    shape.bottomLeft.x = 6;
    expect(playfield.canMoveRight(shape)).toBe(false);
    shape.bottomLeft.x = 5;
    playfield.solidify(new Shape(new Point(6, 0), [new Point(0, 0)]));
    expect(playfield.canMoveRight(shape)).toBe(false);
  });

  it('should see if shape can move down', () => {
    const playfield = new Playfield(7);
    const shape = new Shape(new Point(0, 0), [new Point(0, 0)]);
    shape.bottomLeft.y = 1;
    shape.bottomLeft.x = 2;
    expect(playfield.canMoveDown(shape)).toBe(true);
    shape.bottomLeft.y = 0;
    expect(playfield.canMoveDown(shape)).toBe(false);
    shape.bottomLeft.y = 19;
    playfield.solidify(new Shape(new Point(2, 18), [new Point(0, 0)]));
    expect(playfield.canMoveDown(shape)).toBe(false);
  });
});

describe('moveShape', () => {
  it('should iterate shape location', () => {
    const shape = new Shape(new Point(0, 3), [new Point(0, 0)]);
    const playfield = new Playfield(7);
    const points = [];
    const solidified = moveShape(shape, playfield, 1) !== undefined;

    expect(solidified).toBe(false);
    expect(shape.bottomLeft.x).toBe(1);
    expect(shape.bottomLeft.y).toBe(2);
  });

  it('should solidify shape', () => {
    const shape = new Shape(new Point(0, 0), [new Point(0, 0)]);
    const playfield = new Playfield(7);
    const points = [];
    const solidified = moveShape(shape, playfield, 0) !== undefined;

    expect(solidified).toBe(true);
    expect(shape.bottomLeft.x).toBe(0);
    expect(shape.bottomLeft.y).toBe(0);
  });

  it('should consider solidified shapes', () => {
    let shape = new Shape(new Point(0, 0), [new Point(0, 0)]);
    const playfield = new Playfield(7);
    const points = [];
    const solidified = moveShape(shape, playfield, 0) !== undefined;
    expect(solidified).toBe(true);

    shape = new Shape(new Point(0, 3), [new Point(0, 0)]);
    expect(moveShape(shape, playfield, 0) === undefined).toBe(true);
    expect(moveShape(shape, playfield, 0) === undefined).toBe(true);
    expect(moveShape(shape, playfield, 0) === undefined).toBe(false);
    expect(playfield.getTowerHeight()).toBe(2);

  });
});

describe('solveProblem', () => {
  it('should handle example input problem', () => {
    const solution = solveProblem('>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>', 2022);

    expect(solution).toBe(3068);
  });

  it('should handle big input by using periodicity', () => {
    const solution = solveProblem('>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>', 1e12);

    expect(solution).toBe(1514285714288);
  });
});

describe('findMatchingIterationState', () => {
  it('should find most recent matching iteration state', () => {
    const iterationsStates = [
      new IterationState(0, 2, 3, [1, 4, 3]),
      new IterationState(10, 2, 3, [1, 2, 3]),
      new IterationState(20, 2, 3, [1, 4, 3]),
      new IterationState(30, 2, 3, [1, 2, 3]),
    ];

    const matchingIterationState = findMatchingIterationState(iterationsStates, 
                                                              new IterationState(50, 2, 3, [1, 4, 3]));

    expect(matchingIterationState).toEqual(new IterationState(20, 2, 3, [1, 4, 3]));
  });

  it('should return undefined if no matching iteration state', () => {
    const iterationsStates = [
      new IterationState(0, 2, 3, [1, 4, 3]),
      new IterationState(10, 2, 3, [1, 2, 3]),
      new IterationState(20, 2, 3, [1, 4, 3]),
      new IterationState(30, 2, 3, [1, 2, 3]),
    ];

    const matchingIterationState = findMatchingIterationState(iterationsStates, 
                                                              new IterationState(50, 2, 3, [1, 4, 4]));

    expect(matchingIterationState).toBeUndefined();
  });
});

