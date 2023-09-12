require('module-alias/register');
require('module-alias').addPath('.');

const express = require('express');
const app = express();
const diskUtils = require('utils/disk.js');
const asyncHandler = require('utils/asyncHandler.js');
const constants = require('utils/const.js');

app.get('/', (req, res) => {
  res.send('Welcome to Nova!');
});

app.get('/v1/info', asyncHandler(async (req, res) => {
  const j = await diskUtils.readMetaFile();
  res.send(j);
}));

app.get('/v1/uptime', asyncHandler(async (req, res) => {
    const memm = await diskUtils.readSystemStatusFile('uptime');
    return res.send(memm);
}));

app.get('/v1/memory', asyncHandler(async (req, res) => {
    const memm = await diskUtils.readSystemStatusFile('memory');
    return res.send(memm);
}));

app.get('/v1/temperature', asyncHandler(async (req, res) => {
    const temp = await diskUtils.readFile('temperature');
    return res.send(temp);
}));

app.post('/v1/shutdown', asyncHandler(async (req, res) => {
    await diskUtils.writeFile(constants.SHUTDOWN_SIGNAL_FILE, 'true', null);
    return res.send('shutdown ...');
}));

app.post('/v1/reboot', asyncHandler(async (req, res) => {
    await diskUtils.writeFile(constants.REBOOT_SIGNAL_FILE, 'true', null);
    return res.send('rebooting ...');
}));

app.listen(constants.PORT, () => {
  console.log(`Server listening on port ${constants.PORT}`);
});
