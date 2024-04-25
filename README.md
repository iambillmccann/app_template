## Setting up the development environment

### Rust

To install the latest version of Rust on your Ubuntu 20.04 system on WSL2, you can use Rustup, which is the official installer and version management tool for Rust. Here's how you can do it:

1. **Open your Ubuntu Terminal on WSL2:**
   - You can do this by searching for "Ubuntu" in your Windows start menu and opening the app.

2. **Install the build-essential package:**
   - This package contains tools required for compiling Rust programs, such as GCC.
   - Run the following command:
     ```bash
     sudo apt update && sudo apt install build-essential
     ```

3. **Install Rust using Rustup:**
   - Run the following command to download and start the installation of Rust:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
   - This script downloads and installs `rustup`, which is the Rust toolchain installer, along with the default compiler (`rustc`), standard library, and the package manager (`cargo`).

4. **Configure your path (if not automatically done by the installation script):**
   - Typically, the installation script configures the PATH environment variable to include Cargo’s bin directory. If it does not, you might need to add it manually:
     ```bash
     echo 'source $HOME/.cargo/env' >> ~/.bashrc
     source ~/.bashrc
     ```

5. **Verify the installation:**
   - To ensure Rust is installed correctly, you can run:
     ```bash
     rustc --version
     ```

This will confirm the installed version of Rust, and you are all set to start using Rust on your Ubuntu system in WSL2!

### VS Code

Setting up Visual Studio Code (VS Code) for Rust development involves installing the Rust toolchain, which you've already done, and configuring VS Code with the appropriate extensions and settings for an optimal Rust development experience. Here’s how to do it:

1. **Install the Rust Analyzer Extension:**
   - Open VS Code.
   - Go to the Extensions view by clicking on the square icon on the sidebar or pressing `Ctrl+Shift+X`.
   - Search for **Rust Analyzer**. This is a powerful extension for Rust development, providing features like code completion, on-the-fly error checks, and more.
   - Click on **Install** to add the extension to VS Code.

2. **Configure the Editor Settings (Optional):**
   - To enhance your coding experience with features like format on save, you might want to adjust your editor settings. You can open settings by pressing `Ctrl+,` or selecting **File > Preferences > Settings**.
   - In the search bar at the top, type `rust` to find Rust-specific settings, or type specific settings such as `format on save`.
   - To enable format on save, search for **Editor: Format On Save** and make sure the checkbox is enabled.

3. **Install Additional Tools:**
   - **Cargo**: Rust’s package manager, which should already be installed with Rustup.
   - **Rustfmt**: The Rust code formatter, you can install it via Rustup:
     ```bash
     rustup component add rustfmt
     ```
   - **Clippy**: A collection of lints to catch common mistakes and improve your Rust code, you can install it via Rustup:
     ```bash
     rustup component add clippy
     ```

4. **Check Your Rust Environment in VS Code:**
   - Open a Rust project or create a new one (you can create a new project using Cargo by running `cargo new my_project` in the terminal).
   - Open the command palette by pressing `Ctrl+Shift+P` and type **Rust Analyzer: Reload workspace** to ensure the Rust Analyzer extension is properly recognizing your Rust environment.
   - Try opening a `.rs` file and typing some Rust code to see if the autocomplete, error highlighting, and other features are working.

5. **Debugging Setup:**
   - To set up debugging, you'll need to install the **CodeLLDB** extension from the VS Code marketplace, which supports debugging Rust applications.
   - After installing, you can configure your `launch.json` (accessible via the Run and Debug sidebar or by pressing `Ctrl+Shift+D`) to use CodeLLDB for Rust debugging.

With these steps, your VS Code should be well equipped for Rust development.
