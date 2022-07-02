const hosts = require("./hosts");
const server = require("./server");

const hosts_file = "hosts.txt";
const hostname = "0.0.0.0";
const ports = [ 80, 443 ];

hosts.run(hosts_file);
server.run(hostname, ports);
