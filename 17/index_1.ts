
import { getShape, moveShape, Playfield, Shape } from './17_1';

function main(lines: string[]): void {
  const line = lines[0];
  let charIndex = 0;
  const maxRocks = 2022;
  //const maxRocks = 10;
  const playfield = new Playfield(7);

  console.log('line', line);
  console.log('line.length', line.length);

  for (let i = 0; i < maxRocks; i++) {
    const shape = getShape(i, playfield.getTowerHeight());

    let windDirection = line[charIndex % line.length] === '<' ? -1 : 1;
    while (moveShape(shape, playfield, windDirection)) {
      if(i === 9) {
        printPlayfieldWithShape(playfield, shape);
        console.log('');
      }
      charIndex = (charIndex + 1);
      windDirection = line[charIndex % line.length] === '<' ? -1 : 1;
    }
    // console.log('charIndex', charIndex % line.length);
    charIndex = (charIndex + 1);
  }

  //printPlayfield(playfield);

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

function printPlayfieldWithShape(playfield: Playfield, shape: Shape): void {
  const maxY = Math.max(shape.maxY(), playfield.getTowerHeight());
  for (let y = maxY; y >= 0; y--) {
    let line = '';
    for (let x = 0; x < playfield.width; x++) {
      if (shape.isAt(x, y)) {
        line += 'O';
      } else {
        line += playfield.get(x, y) ? '#' : '.';
      }
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
