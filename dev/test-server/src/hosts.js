const hostile = require('hostile');

module.exports = {
    run: (hosts_file) => {
        // load host overrides
        hostile.getFile(hosts_file, false, async(err, lines) => {
            if (err) { console.error(err.message); }

            // modify hosts
            console.log("\nREDIRECTING:");

            // generate file-write closures
            const redirect_ops = [];
            lines.forEach((line) => {
                redirect_ops.push((resolve) => {
                    hostile.set(line[0], line[1], (err) => {
                        if (err) {
                            console.error(err.message);
                            console.warn("Note: the user running this script must have perms to modify the /etc/hosts file, or modify it manually and run this with an empty hosts.txt file.");
                            // don't run without redirects
                            process.exit();
                        }
                        console.log(`  ${line[1]} -> ${line[0]}`);
                        resolve();
                    })
                });
            });

            // run and await each
            for (i = 0; i < redirect_ops.length; i++) {
                await new Promise(redirect_ops[i]);
            }

            // revert hosts
            process.on("SIGINT", async() => {
                process.stdout.clearLine();
                console.log("\nREVERTING:");

                const revert_ops = [];
                lines.forEach((line) => {
                    revert_ops.push((resolve) => {
                        hostile.remove(line[0], line[1], (err) => {
                            if (err) { console.error(err.message); }
                            console.log(`  ${line[1]}`);
                            resolve();
                        });
                    })
                });

                // run and await each
                for (i = 0; i < revert_ops.length; i++) {
                    await new Promise(revert_ops[i]);
                }

                // exit
                process.stdout.clearLine();
                process.exit();
            });
        });
    }
}
