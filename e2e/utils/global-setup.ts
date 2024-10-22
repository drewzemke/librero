import { execSync } from "child_process";

async function setupTests() {
  execSync(
    "deno task --cwd ../client build --outDir=../server/assets/ --emptyOutDir=false"
  );
}

export default setupTests;
