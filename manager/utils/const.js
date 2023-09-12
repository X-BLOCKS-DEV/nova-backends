function readFromEnvOrTerminate(key) {
	const value = process.env[key];

	if(typeof(value) !== "string" || value.trim().length === 0) {
		console.error(`The env. variable '${key}' is not set. Terminating...`);

		process.exit(0);
	}

	return value;
}

module.exports = {
  PORT: process.env.PORT || 3006,
  SHUTDOWN_SIGNAL_FILE: process.env.SHUTDOWN_SIGNAL_FILE || '/signals/shutdown',
  REBOOT_SIGNAL_FILE: process.env.REBOOT_SIGNAL_FILE || '/signals/reboot',
};
