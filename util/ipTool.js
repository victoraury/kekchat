const { networkInterfaces} = require('os');
const fs = require('fs');

const nets = networkInterfaces();

for (const name of Object.keys(nets)) {
	if (!name.toLowerCase().includes('wi-fi')) continue;

	for (const net of nets[name]) {
		if (net.internal || net.family != 'IPv4') continue;

		fs.writeFileSync(__dirname + '/ip.json', JSON.stringify({ ip: net.address }));
		break;
    }
}