//vmdl javascript parser module

/**
 * parse VMDL content into a javascript object
 * @param {string} content - VMDL content as a string
 * @returns {object} - parsed javascript object
 */
function parse(content) {
  const lines = content.split('\n');
  const root = {};
  const stack = [{ level: 0, object: root }];

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    
    // skip empty lines
    if (line.trim() === '') continue;
    
    // skip comments
    if (line.trim().startsWith('#')) continue;

    // calculate indentation level (count leading spaces/tabs)
    const indentLevel = line.length - line.trimStart().length;
    const lineContent = line.trim();
    
    // pop the stack until we find a parent with lower indentation
    while (stack.length > 1 && stack[stack.length - 1].level >= indentLevel) {
      stack.pop();
    }
    
    const parent = stack[stack.length - 1].object;
    
    if (lineContent.endsWith(':')) {
      // this is an object declaration
      const key = lineContent.slice(0, -1).trim();
      parent[key] = {};
      stack.push({ level: indentLevel, object: parent[key] });
    } else if (lineContent.includes('=')) {
      // this is a key-value pair
      const [key, ...valueParts] = lineContent.split('=');
      const value = valueParts.join('=').trim();
      parent[key.trim()] = value;
    }
  }
  
  return root;
}

/**
 * Convert a javascript object to VMDL format
 * @param {object} data - javascript object to convert
 * @param {number} indent - current indentation level (internal use)
 * @returns {string} - VMDL formatted string
 */
function stringify(data, indent = 0) {
  let result = '';
  const spacing = ' '.repeat(indent);
  
  for (const [key, value] of Object.entries(data)) {
    if (typeof value === 'object' && value !== null) {
      result += `${spacing}${key}:\n`;
      result += stringify(value, indent + 4);
    } else {
      result += `${spacing}${key} = ${value}\n`;
    }
  }
  
  return result;
}

module.exports = { parse, stringify };
