//vmdl javascript parser example
const fs = require('fs');
const path = require('path');
const { parse, stringify } = require('./index');

// path to example file (assuming it's in the project's examples directory)
const examplePath = path.join(__dirname, '../../examples/config.vmdl');

// read and parse the file
try {
  const content = fs.readFileSync(examplePath, 'utf8');
  console.log('VMDL Content:');
  console.log('-'.repeat(50));
  console.log(content);
  console.log('-'.repeat(50));
  
  // parse to JavaScript object
  const data = parse(content);
  console.log('\nParsed JavaScript Object:');
  console.log('-'.repeat(50));
  console.log(JSON.stringify(data, null, 2));
  console.log('-'.repeat(50));
  
  // access some values
  console.log('\nAccessing values:');
  console.log(`Project: ${data.Project}`);
  console.log(`Route: ${data.Route}`);
  console.log(`Staging Route: ${data.Environments.Staging.Route}`);
  
  // convert back to VMDL
  const regenerated = stringify(data);
  console.log('\nRegenerated VMDL:');
  console.log('-'.repeat(50));
  console.log(regenerated);
  console.log('-'.repeat(50));
  
  // write to a temporary file
  const tempPath = path.join(__dirname, '../../examples/temp_output.vmdl');
  fs.writeFileSync(tempPath, regenerated);
  console.log(`\nWrote regenerated VMDL to ${tempPath}`);
  
} catch (error) {
  console.error('Error:', error.message);
}
