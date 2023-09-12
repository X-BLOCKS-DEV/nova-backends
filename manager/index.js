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

app.post('/v1/shutdown', asyncHandler(async (req, res) => {
    await diskUtils.writeFile(constants.SHUTDOWN_SIGNAL_FILE, 'true', null);
    return res.send('shutdown ...')
}));

app.post('/v1/reboot', asyncHandler(async (req, res) => {
    await diskUtils.writeFile(constants.REBOOT_SIGNAL_FILE, 'true', null);
    return res.send('rebooting ...')
}));

app.listen(constants.PORT, () => {
  console.log(`Server listening on port ${constants.PORT}`);
});
