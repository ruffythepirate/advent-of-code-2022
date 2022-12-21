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
    const values = [];
    for (let i = 0; i < lines.length; i += 3) {
      values.push(parseValue(lines[i]));
      values.push(parseValue(lines[i + 1]));
    }
    values.push([[2]]);
    values.push([[6]]);

    const sorted = values.sort((a, b) => compareValues(a, b));

    sorted.forEach((value) => {
      console.log(JSON.stringify(value));
    });
    const lowIndex = sorted.findIndex((value) => { return compareValues(value, [[2]]) === 0; });
    const highIndex = sorted.findIndex((value) => { return compareValues(value, [[6]]) === 0; });

    console.log(lowIndex, highIndex);

    console.log((lowIndex + 1) * (highIndex + 1));
  });
}

solveProblem();




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
