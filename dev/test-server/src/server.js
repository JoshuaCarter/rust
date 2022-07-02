const http = require("http");

const server = http.createServer(options, (req, res) => {
    // buffer data
    var data = "";
    req.on("data", (chunk) => {
        data += chunk;
    });

    // log and reflect data back to requestor
    req.on("end", () => {
        console.log(data);
        console.log(`\n${req.method} ${req.url}`);
        console.log(req.headers);
        console.log("BODY: " + data);

        res.statusCode = 200;
        res.setHeader("Content-Type", "text/plain");
        res.end(data);
    });
});

module.exports = {
    run: (hostname, ports) => {
        // listen for http
        try {
            ports.forEach((port) => {
                server.listen(port, hostname, () => {
                    console.log(`Listening on ${hostname}:${port}`);
                });
            });
        }
        catch (e) {
            console.error(e);
            console.warn(`Tip(1): You should run this as admin so we can bind to ports under 1024 and modify /etc/hosts`);
            console.warn(`Tip(2): You may need to stop any services using on ports ${ports} (e.g. apache2)`);
        }
    }
};
