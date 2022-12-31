
import { getShape, moveShape, Playfield } from './17_2';

function main_find_period(lines: string[]): void {
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

function main(line: string, maxRocks: number): void {
  // Calculate stones involved in period.
  const {heightGainAfterWindIteration, stoneGainAfterWindIteration, charOffsetAfterWindIteration} = findIterationResults(line, 1e6);
  console.log(heightGainAfterWindIteration);
  console.log(stoneGainAfterWindIteration);
  console.log(charOffsetAfterWindIteration);
  const initialStones = stoneGainAfterWindIteration.slice(0, 6).reduce((a, b) => a + b, 0);
  const initialHeight = heightGainAfterWindIteration.slice(0, 6).reduce((a, b) => a + b, 0);

  const periodicStones = stoneGainAfterWindIteration.slice(6, 16).reduce((a, b) => a + b, 0);
  const periodicHeight = heightGainAfterWindIteration.slice(6, 16).reduce((a, b) => a + b, 0);

  const stonePeriods = Math.floor((1e12 - initialStones) / periodicStones);
  const extraStones = (1e12 - initialStones) % periodicStones;

  const playfield = new Playfield(7);
  // we prep the playfield to come into periodic state.
  iterateSolution(line, 0, initialStones, playfield, () => {});
  const prepHeight = playfield.getTowerHeight();
  iterateSolution(line, initialStones, extraStones, playfield, () => {}, -charOffsetAfterWindIteration[6]);
  const lastPieceOfHeight = playfield.getTowerHeight() - prepHeight;

  console.log(initialStones, initialHeight, periodicStones, periodicHeight, stonePeriods, extraStones, prepHeight, lastPieceOfHeight);

  const finalHeight = initialHeight + periodicHeight * stonePeriods + lastPieceOfHeight;
  console.log(finalHeight);
}

function iterateSolution(line: string, rockOffset: number, maxRocks: number, playfield: Playfield, windIterationDone: (rockNumber: number, charOffset: number) => void, initialCharOffset = 0) {
  let charIndex = initialCharOffset;
  for (let i = 0; i < maxRocks; i++) {
    const shape = getShape(rockOffset + i, playfield.getTowerHeight());

    if(i % 1000 === 0) {
      playfield.cleanupHiddenRows();
    }

    let windDirection = getWindDirection(line, charIndex);
    let solidifiedPosition = moveShape(shape, playfield, windDirection);
    let charOffset = 0;
    while (solidifiedPosition === undefined) {
      charIndex = (charIndex + 1);
      charOffset++;
      if(charIndex % line.length === 0) {
        windIterationDone(i, charOffset);
      }
      windDirection = getWindDirection(line, charIndex);
      solidifiedPosition = moveShape(shape, playfield, windDirection);
    }
    charIndex = (charIndex + 1);
    charOffset++;
    if(charIndex % line.length === 0) {
        windIterationDone(i, charOffset);
    }
  }
}

/**
 * Solution: Create a function that gets the top Y of each column in the playfield.
 * Create an iterate function with a hook function every time charOffset has reset. It takes charOffset % line.length / stoneNumber as parameters / shape startY
 * Create a function that iterates the playfield and uses the callback to calculate height diffs. Store all of this in an array. Then store these arrays in another array. Every time this is called the callback returns a number indicating how many stones can be skipped.
 */

function findIterationResults(line: string, maxRocks: number): {heightGainAfterWindIteration: number[], stoneGainAfterWindIteration: number[], charOffsetAfterWindIteration: number[]} {
  const heightGainAfterWindIteration: number[] = [];
  let previousHeightGain = 0;
  let previousStones = 0;
  const stoneGainAfterWindIteration: number[] = [];
  const charOffsetAfterWindIteration: number[] = [];
  const playfield = new Playfield(7);
  iterateSolution(line, 0, maxRocks, playfield, (rockNumber, charOffset) => {
    charOffsetAfterWindIteration.push(charOffset);
    heightGainAfterWindIteration.push(playfield.getTowerHeight() - previousHeightGain);
    stoneGainAfterWindIteration.push(rockNumber - previousStones);
    previousHeightGain = playfield.getTowerHeight();
    previousStones = rockNumber;
  });

  return {heightGainAfterWindIteration, stoneGainAfterWindIteration, charOffsetAfterWindIteration};
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
    main(lines[0], 2022);
  });
}

readStdin();
