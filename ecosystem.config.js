module.exports = {
  apps: [
    {
      name: "miguer-api",
      script: "./miguer_api-cli",
      args: "start --environment production",
      exec_mode: "fork",
      watch: ["./miguer_api-cli"],
      watch_delay: 1000,
      autorestart: true,
      restart_delay: 500,
      env: {
        RUST_LOG: "info",
      },
    },
  ],
};
