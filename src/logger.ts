import chalk from 'chalk';

const error = chalk.red;
const valid= chalk.green;
const dateColor = chalk.blue;
const timeColor = chalk.blue.bold;

export function log(msg: string, tabs = 1) {
  const date = new Date();
  console.log(`${"\t".repeat(tabs)}${dateColor(date.toLocaleDateString())} ${timeColor(date.toLocaleTimeString("fr-FR"))} ${valid(msg)}`);
}

export function logError(msg: string) {
  console.log(error(msg));

}

export function newLine() {
  console.log();
}

