> This is unmaintained. Use this more active fork https://github.com/qaz741wsd856/warden-worker

# Warden: A Bitwarden-compatible server for Cloudflare Workers

This project provides a self-hosted, Bitwarden-compatible server that can be deployed to Cloudflare Workers for free. It's designed to be low-maintenance, allowing you to "deploy and forget" without worrying about server management or recurring costs.

## Why another Bitwarden server?

While projects like [Vaultwarden](https://github.com/dani-garcia/vaultwarden) provide excellent self-hosted solutions, they still require you to manage a server or VPS. This can be a hassle, and if you forget to pay for your server, you could lose access to your passwords.

Warden aims to solve this problem by leveraging the Cloudflare Workers ecosystem. By deploying Warden to a Cloudflare Worker and using Cloudflare D1 for storage, you can have a completely free, serverless, and low-maintenance Bitwarden server.

## Features

*   **Core Vault Functionality:** All your basic vault operations are supported, including creating, reading, updating, and deleting ciphers and folders.
*   **TOTP Support:** Store and generate Time-based One-Time Passwords for your accounts.
*   **Bitwarden Compatible:** Works with the official Bitwarden browser extensions and Android app (iOS is untested).
*   **Free to Host:** Runs on Cloudflare's free tier.
*   **Low Maintenance:** Deploy it once and forget about it.
*   **Secure:** Your data is stored in your own Cloudflare D1 database.
*   **Easy to Deploy:** Get up and running in minutes with the Wrangler CLI.

## Current Status

**This project is not yet feature-complete.** It currently supports the core functionality of a personal vault, including TOTP. However, it does **not** support the following features:

*   Sharing
*   Bitwarden Send
*   Organizations
*   Other Bitwarden advanced features

There are no immediate plans to implement these features. The primary goal of this project is to provide a simple, free, and low-maintenance personal password manager.

## Compatibility

*   **Browser Extensions:** Chrome, Firefox, Safari, etc.
*   **Android App:** The official Bitwarden Android app.
*   **iOS App:** Untested. If you have an iOS device, please test and report your findings!

## Getting Started

### Prerequisites

*   A Cloudflare account.
*   The [Wrangler CLI](https://developers.cloudflare.com/workers/wrangler/get-started/) installed and configured.

### Deployment

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/your-username/warden-worker.git
    cd warden-worker
    ```

2.  **Create a D1 Database:**

    ```bash
    wrangler d1 create warden-db
    ```

3.  **Configure your Database ID:**

    When you create a D1 database, Wrangler will output the `database_id`. To avoid committing this secret to your repository, this project uses an environment variable to configure the database ID.

    You have two options:

    **Option 1: (Recommended) Use a `.env` file:**

    Create a file named `.env` in the root of the project and add the following line, replacing the placeholder with your actual `database_id`:

    ```
    D1_DATABASE_ID="your-database-id-goes-here"
    ```

    Make sure to add the `.env` file to your `.gitignore` file to prevent it from being committed to git.

    **Option 2: Set an environment variable in your shell:**

    You can set the environment variable in your shell before deploying:

    ```bash
    export D1_DATABASE_ID="your-database-id-goes-here"
    wrangler deploy
    ```

4.  **Deploy the worker:**

    ```bash
    wrangler deploy
    ```

    This will deploy the worker and set up the necessary database tables.

5.  **Configure your Bitwarden client:**

    In your Bitwarden client, go to the self-hosted login screen and enter the URL of your deployed worker (e.g., `https://warden-worker.your-username.workers.dev`).

## Configuration

This project requires minimal configuration. The main configuration is done in the `wrangler.toml` file, where you specify your D1 database binding.

## Contributing

Contributions are welcome! If you find a bug, have a feature request, or want to improve the code, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
