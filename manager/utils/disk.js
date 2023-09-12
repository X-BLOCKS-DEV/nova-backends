/**
 * Generic disk functions.
 */

const fs = require('fs');
const path = require('path');
const constants = require('utils/const.js');

function readSystemStatusFile(resource) {
  const statusFilePath = path.join(constants.STATUS_DIR, `${resource}-status.json`);
  return readJsonFile(statusFilePath)
    .catch(() => null);
}

function readMetaFile() {
  return readJsonFile(constants.METADATA_FILE)
    .catch(() => null);
}

// Reads a file as a utf8 string. Wraps fs.readFile into a native promise
async function readUtf8File(filePath) {
  return (await readFile(filePath, 'utf8')).trim();
}

async function readJsonFile(filePath) {
  return readUtf8File(filePath).then(JSON.parse);
}

function readFile(filePath, encoding) {
  return new Promise((resolve, reject) => fs.readFile(filePath, encoding, (err, str) => {
    if (err) {
      reject(err);
    } else {
      resolve(str);
    }
  }));
}

function writeFile(filePath, data, encoding) {
  return new Promise((resolve, reject) => fs.writeFile(filePath, data, encoding, err => {
    if (err) {
      reject(err);
    } else {
      resolve();
    }
  }));
}

function deleteFile(filePath) {
  return new Promise((resolve, reject) => fs.unlink(filePath, (err, str) => {
    if (err) {
      reject(err);
    } else {
      resolve(str);
    }
  }));
}

module.exports = {
  readFile,
  writeFile,
  deleteFile,
  readSystemStatusFile,
  readMetaFile
}
