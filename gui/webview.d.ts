interface Plugin {
	/**
	 * Registers a callback to receive messages from the plugin host.
	 * @param callback - Function invoked when a message is received.
	 * @returns A function to unsubscribe and remove the listener.
	 */
	listen: (callback: (message: string) => void) => () => void;

	/**
	 * Send a message to the plugin host
	 * @param {string} message - Message to send (must be a string)
	 */
	send: (message: string) => void;
}

declare global {
	interface Window {
		plugin: Plugin;
	}
}

export {};
