# mljboard
A leader board for Maloja servers.

## `HOS` (HTTP-Over-Socket)
**NOTE**: If you're looking for the client, check at [duckfromdiscord/mljboard-client](https://github.com/duckfromdiscord/mljboard-client). The repository you're looking at right now is the server.

Everyone knows port forwarding can be a massive pain when you're behind a NAT or using a cheap router. Our Maloja leaderboard, which needs to periodically speak to your local Maloja, can't do its job if it can't connect. With `HOS`, it doesn't have to. Instead, you connect to it! Our `HOS` server is the only thing that needs to be accessible publically, and clients can connect to it and offer a port on their machine that a project using `HOS` can talk to.

*Please don't use HOS if you have access to port forwarding, it takes a lot of bandwidth.*

