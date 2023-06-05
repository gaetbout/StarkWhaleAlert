export function log(msg: string, tabs = 1) {
  const date = new Date();
  console.log(`${"\t".repeat(tabs)}${date.toLocaleDateString()}T${date.toLocaleTimeString("fr-FR")} - ${msg}`);
}
