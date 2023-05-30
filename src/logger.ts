export function log(msg: string, tabs = 1) {
  console.log(`${"\t".repeat(tabs)}${new Date().toISOString()} - ${msg}`);
}
