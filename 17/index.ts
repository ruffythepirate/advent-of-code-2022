
import { getShape, moveShape, Playfield } from './17_1';

function main(lines: string[]): void {
  const line = lines[0];
  let charIndex = 0;
  const maxRocks = 2022;
  //const maxRocks = 2;
  const playfield = new Playfield(7);

  for (let i = 0; i < maxRocks; i++) {
    console.log(charIndex);
    const shape = getShape(i, playfield.getTowerHeight());

    let windDirection = line[charIndex % line.length] === '<' ? -1 : 1;
    while (moveShape(shape, playfield, windDirection)) {
      charIndex = (charIndex + 1);
      windDirection = line[charIndex] === '<' ? -1 : 1;
    }
    charIndex = (charIndex + 1);
  }

  //printPlayfield(playfield);
  console.log(charIndex);

  console.log(playfield.getTowerHeight());
}

function printPlayfield(playfield: Playfield): void {
  for (let y = playfield.getTowerHeight(); y >= 0; y--) {
    let line = '';
    for (let x = 0; x < playfield.width; x++) {
      line += playfield.get(x, y) ? '#' : '.';
    }
    console.log(line);
  }
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
    main(lines);
  });
}

readStdin();
