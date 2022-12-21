import { parseValue, compareValues } from './13_1';

describe('parseValue', () => {
  it('should parse a number', () => {
    expect(parseValue('1')).toBe(1);
  });
  it('should parse an array', () => {
    expect(parseValue('[1,2,3]')).toEqual([1, 2, 3]);
  });
  it('should parse an array of arrays', () => {
    expect(parseValue('[[1,2],[3,4]]')).toEqual([[1, 2], [3, 4]]);
  });
});

describe('compareValues', () => {
  it('should compare numbers', () => {
    expect(compareValues(1, 2)).toBe(-1);
    expect(compareValues(2, 1)).toBe(1);
    expect(compareValues(1, 1)).toBe(0);
  });

  it('should compare arrays', () => {
    expect(compareValues([1, 2], [1, 3])).toBe(-1);
    expect(compareValues([1, 2], [1, 1])).toBe(1);
    expect(compareValues([1, 2], [1, 2])).toBe(0);
  });

  it('should compare array to number', () => {
    expect(compareValues([1], 1)).toBe(0);
    expect(compareValues(1, [1, 2])).toBe(-1);
  });
});

