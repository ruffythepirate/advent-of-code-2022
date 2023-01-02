
import { getShape, moveShape, Playfield, solveProblem } from './17_2';

function main(line: string, maxRocks: number): void {
  const solution = solveProblem(line, maxRocks);
  console.log(solution);
}

/**
 * Reads all input from stdin and returns it as a string.
 */
function readStdin() {
  const readline = require('readline');
  const lines: string[] = [];
  const rl = readline.createInterface({
    input: process.stdin,
  }).on('line', (line: string) => {
    lines.push(line);
  }).on('close', () => {
    main(lines[0], 1e12);
  });
}

readStdin();
