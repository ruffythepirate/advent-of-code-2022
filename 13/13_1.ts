console.log('hello');

/**
 * Parses a number or an array containing numbers or arrays.
 */
export function parseValue(value: string): any {
  return JSON.parse(value);
}

export function solveProblem() {

  const readline = require('readline');

  const lines: string[] = [];

  readline.createInterface({
    input: process.stdin,
    terminal: false
  }).on('line', (line: string) => {
    lines.push(line);
  }).on('close', () => {
    const valuePairs = [];
    for (let i = 0; i < lines.length; i += 3) {
      const valueTuple = [parseValue(lines[i]), parseValue(lines[i + 1])];
      valuePairs.push(valueTuple);
    }

    const correctPairs = valuePairs
        .map(([a, b]) => compareValues(a, b) < 0);
    let sum = 0;
    correctPairs.forEach((correct, index) => {
      if (correct) {
        sum += 1 + index;
      }
    });
    console.log(sum);
  });
}





/**
 * Compares two values. If both are numbers it compares them and returns positive if the first is bigger.
 * If both are arrays it compares the first elements and if they are equal compares the second elements. 
 * If both are arrays and the first array is shorter than the second it returns positive.
 */
export function compareValues(value1: any, value2: any): number {
  const isFirstNumber = typeof value1 === 'number';
  const isSecondNumber = typeof value2 === 'number';
  if (isFirstNumber && isSecondNumber) {
    return value1 - value2;
  }

  if (isFirstNumber) {
    return compareValues([value1], value2);
  } else if(isSecondNumber) {
    return compareValues(value1, [value2]);
  }

  const firstAsArray = value1 as any[];
  const secondAsArray = value2 as any[];
  const minLength = Math.min(firstAsArray.length, secondAsArray.length);
  for(let i = 0; i < minLength; i++) {
    const result = compareValues(firstAsArray[i], secondAsArray[i]);
    if (result !== 0) {
      return result;
    }
  }
  return value1.length - value2.length;
  
}
