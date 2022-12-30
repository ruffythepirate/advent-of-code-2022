
import { getShape, moveShape, Playfield } from './17_2';

function main(lines: string[]): void {
  const line = lines[0];
  let charIndex = 0;
  const maxRocks = 1e6;
  //const maxRocks = 2;
  const playfield = new Playfield(7);

  const heightGainAfterWindIteration = [];
  let previousHeightGain = 0;
  let previousStones = 0;
  const stoneGainAfterWindIteration = [];

  for (let i = 0; i < maxRocks; i++) {
    const shape = getShape(i, playfield.getTowerHeight());

    if(i % 1000 === 0) {
      playfield.cleanupHiddenRows();
    }


    let windDirection = getWindDirection(line, charIndex);
    let solidifiedPosition = moveShape(shape, playfield, windDirection);
    while (solidifiedPosition === undefined) {
      if(charIndex % line.length === 0) {
        //console.log('char index reset, shape index = ', i % 5, 'first rocks: ', firstRocks);
      }
      charIndex = (charIndex + 1);
      if(charIndex % line.length === 0) {
        heightGainAfterWindIteration.push(playfield.getTowerHeight() - previousHeightGain);
        stoneGainAfterWindIteration.push(i - previousStones);
        previousHeightGain = playfield.getTowerHeight();
        previousStones = i;
      }
      windDirection = getWindDirection(line, charIndex);
      solidifiedPosition = moveShape(shape, playfield, windDirection);
    }
    charIndex = (charIndex + 1);
    if(charIndex % line.length === 0) {
        heightGainAfterWindIteration.push(playfield.getTowerHeight() - previousHeightGain);
        stoneGainAfterWindIteration.push(i - previousStones);
        previousHeightGain = playfield.getTowerHeight();
        previousStones = i;
    }
  }

  console.log(heightGainAfterWindIteration);
  console.log(stoneGainAfterWindIteration);


  console.log(playfield.getTowerHeight());
}

function getWindDirection(line: string, charIndex: number): number {
  return line[charIndex % line.length] === '<' ? -1 : 1;
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
