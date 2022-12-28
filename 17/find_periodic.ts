
function main(lines: string[]): void {
  const line = lines[0];
  const lineLength = line.length;
  const doubleLine = line + line;

  for(let i = 0; i < lineLength; i++) {
    let substr = doubleLine.substr(0, i + 1);
    let k = 0;
    let isPeriodic = true;
    while (k < lineLength) {
      k += i + 1;
      if (doubleLine.substr(k, i + 1) !== substr) {
        isPeriodic = false;
        break;
      }
    }
    if (isPeriodic) {
      console.log(i + 1);
      return;
    }
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
