
import { getShape, moveShape, Playfield } from './17_2';

function main(lines: string[]): void {
  const line = lines[0];
  let charIndex = 0;
  const maxRocks = 2022;
  //const maxRocks = 2;
  const playfield = new Playfield(7);

  const firstRocks = []

  for (let i = 0; i < maxRocks; i++) {
    const shape = getShape(i, playfield.getTowerHeight());


    let windDirection = line[charIndex % line.length] === '<' ? -1 : 1;
    let solidifiedPosition = moveShape(shape, playfield, windDirection);
    while (solidifiedPosition === undefined) {
      if(charIndex % line.length === 0) {
        //console.log('char index reset, shape index = ', i % 5, 'first rocks: ', firstRocks);
      }
      charIndex = (charIndex + 1);
      windDirection = line[charIndex] === '<' ? -1 : 1;
      solidifiedPosition = moveShape(shape, playfield, windDirection);
    }
    if (firstRocks.length < 10) {
      firstRocks.push(solidifiedPosition);
    }
    if(charIndex % line.length === 0) {
      console.log('char index reset after solid, shape index = ', i % 5);//, 'first rocks: ', firstRocks);
    }
    charIndex = (charIndex + 1);
  }

  console.log(charIndex);
  console.log(playfield.minY, playfield.getTowerHeight());

  console.log(playfield.getTowerHeight());
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
